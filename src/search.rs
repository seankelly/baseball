use std::cmp::Reverse;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

use crate::player;

use cel::{Context, Program, Value};
use rayon::prelude::*;
use tracing::trace;


pub trait CelEval {
    fn add_cel_variables(&self, context: &mut Context, variables: &[&str]) -> Result<(), Box<dyn Error>>;

    fn check_cel_variables(&self, variables: &[&str]) -> bool;
}


pub struct CelExec<'a> {
    context: Context<'a>,
    filter_program: Option<Program>,
    sort_program: Option<Program>,
    condition_program: Option<Program>,
    count_program: Option<Program>,
}


pub enum SortOrder {
    Asc,
    Desc,
}


#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Key {
    pub id: String,
    pub year: i32,
}


pub struct StreakSpan {
    /// The player or team ID for this streak.
    pub id: String,
    /// The first game in the streak.
    pub start: String,
    /// The final game in the streak.
    pub end: String,
    /// The length of the streak in games.
    pub length: u32,
    /// The length of the streak in either games or another countable statistic (e.g. plate
    /// appearances).
    pub count: u32,
}


pub struct StreakEntry {
    pub game_id: String,
    pub result: bool,
    pub count: u8,
}


impl<'a> CelExec<'a> {
    pub fn new() -> Self {
        let context = Context::default();

        Self {
            context,
            filter_program: None,
            sort_program: None,
            condition_program: None,
            count_program: None,
        }
    }

    pub fn set_filter(&mut self, source: &str) -> Result<(), Box<dyn Error>> {
        self.filter_program = Some(Program::compile(source)?);
        Ok(())
    }

    pub fn set_sort(&mut self, source: &str) -> Result<(), Box<dyn Error>> {
        self.sort_program = Some(Program::compile(source)?);
        Ok(())
    }

    pub fn set_condition(&mut self, source: &str) -> Result<(), Box<dyn Error>> {
        self.condition_program = Some(Program::compile(source)?);
        Ok(())
    }

    pub fn set_count(&mut self, source: &str) -> Result<(), Box<dyn Error>> {
        self.count_program = Some(Program::compile(source)?);
        Ok(())
    }

    pub fn streak_eval<'data, T, U>(&self, map: &'data HashMap<T, Vec<U>>) -> HashMap<&'data T, Vec<StreakEntry>>
        where T: Eq + Hash + Sync,
              U: Sync + CelEval + player::PlayerGamelog,
    {
        if let Some(ref program) = self.condition_program {
            let references = program.references();
            let variables = references.variables();
            let player_streaks: HashMap<_, _> = map.par_iter().map(|kv| {
                let (key, value) = kv;
                let entries: Vec<_> = value.iter().map(|e| self.streak_eval_each(e, program, &variables)).collect();
                (key, entries)
            }).collect();
            player_streaks
        }
        else {
            HashMap::new()
        }
    }

    fn streak_eval_each<T>(&self, element: &T, program: &Program, variables: &[&str]) -> StreakEntry
        where T: CelEval + player::PlayerGamelog,
    {
        let mut ctx = self.context.new_inner_scope();
        let mut count = 0;
        let result = if element.add_cel_variables(&mut ctx, variables).is_err() {
            false
        }
        else {
            match program.execute(&ctx) {
                Ok(Value::Bool(true)) => true,
                Ok(_) => false,
                Err(error) => {
                    eprintln!("error evaluating streak program: {error}");
                    false
                }
            }
        };

        if result {
            count = 1;
            if let Some(ref program) = self.count_program {
                let references = program.references();
                let variables = references.variables();
                let mut count_ctx = self.context.new_inner_scope();
                if element.add_cel_variables(&mut count_ctx, &variables).is_ok() {
                    count = match program.execute(&count_ctx) {
                        Ok(Value::Int(i)) => { i as u8 }
                        Ok(Value::UInt(u)) => { u as u8 }
                        Ok(_) => 1,
                        Err(error) => {
                            eprintln!("error evaluating count program: {error}");
                            0
                        }
                    }
                }
            }
        }

        StreakEntry {
            game_id: element.game_id().to_string(),
            result,
            count,
        }
    }

    pub fn find_streaks(streak_map: &HashMap<&Key, Vec<StreakEntry>>) -> Vec<StreakSpan> {
        let mut streaks = Vec::with_capacity(150);
        let mut streak_minimum = 2;
        for (key, entries) in streak_map.iter() {
            let mut streak_start = None;
            let mut streak_end = None;
            let mut length = 0;
            let mut count = 0;
            for entry in entries {
                if entry.result {
                    length += 1;
                    count += entry.count as u32;
                    if streak_start.is_none() {
                        streak_start = Some(&entry.game_id);
                    }
                    streak_end = Some(&entry.game_id);
                }
                else {
                    if let (Some(start), Some(end)) = (streak_start, streak_end)
                        && count >= streak_minimum {
                        let span = StreakSpan {
                            id: key.id.to_owned(),
                            start: start.clone(),
                            end: end.clone(),
                            length,
                            count,
                        };
                        streaks.push(span);
                    }
                    streak_start = None;
                    streak_end = None;
                    length = 0;
                    count = 0;
                }
            }

            // Check for streaks that end with the final entry of the Vec.
            if let (Some(start), Some(end)) = (streak_start, streak_end)
                && count >= streak_minimum {
                let span = StreakSpan {
                    id: key.id.to_owned(),
                    start: start.clone(),
                    end: end.clone(),
                    length,
                    count,
                };
                streaks.push(span);
            }

            // Sort the spans and check the 100th entry to see if the streak minimum length should
            // increase. If so, prune the list to only spans meeting the new minimum.
            streaks.sort_unstable_by_key(|a| Reverse(a.count));
            if let Some(span) = streaks.get(100)
                && span.count > streak_minimum {
                streak_minimum = span.count;
                trace!(streak_minimum = streak_minimum, streaks = streaks.len(), "Increasing streak minimum");
                streaks.retain(|span| span.count >= streak_minimum);
            }
        }

        streaks
    }

    pub fn filter<T: CelEval>(&self, input: &mut Vec<T>) {
        if let Some(filter_program) = self.filter_program.as_ref() {
            let references = filter_program.references();
            let variables = references.variables();
            input.retain(|element| self.filter_option(element, filter_program, &variables));
        }
    }

    fn filter_option<T: CelEval>(&self, element: &T, program: &Program, variables: &[&str]) -> bool {
        let mut player_ctx = self.context.new_inner_scope();
        if element.add_cel_variables(&mut player_ctx, variables).is_err() {
            return false;
        }
        match program.execute(&player_ctx) {
            Ok(Value::Bool(true)) => true,
            Ok(_) => false,
            Err(error) => {
                eprintln!("error evaluating: {error}");
                false
            }
        }
    }

    pub fn sort<T: CelEval + Send>(&self, input: &mut [T], sort_order: &SortOrder) {
        if let Some(sort_program) = self.sort_program.as_ref() {
            let references = sort_program.references();
            let variables = references.variables();
            input.par_sort_unstable_by(|a, b| {
                let a_res = self.sort_key(a, sort_program, &variables);
                let b_res = self.sort_key(b, sort_program, &variables);
                match sort_order {
                    SortOrder::Asc => { a_res.total_cmp(&b_res) }
                    SortOrder::Desc => { b_res.total_cmp(&a_res) }
                }
            });
        }
    }

    fn sort_key<T: CelEval>(&self, career: &T, program: &Program, variables: &[&str]) -> f64 {
        let mut player_ctx = self.context.new_inner_scope();
        if career.add_cel_variables(&mut player_ctx, variables).is_err() {
            return f64::NEG_INFINITY;
        }

        let result = program.execute(&player_ctx);
        match result {
            Ok(Value::Int(i)) => { i as f64 }
            Ok(Value::UInt(u)) => { u as f64 }
            Ok(Value::Float(f)) => { f }
            _ => f64::INFINITY
        }
    }
}
