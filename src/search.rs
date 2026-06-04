use std::collections::HashMap;
use std::hash::Hash;
use std::error::Error;

use crate::player;

use cel::{Context, Program, Value};
use rayon::prelude::*;


pub trait CelEval {
    fn add_cel_variables(&self, context: &mut Context, variables: &[&str]) -> Result<(), Box<dyn Error>>;
}


pub struct CelExec<'a> {
    context: Context<'a>,
    filter_program: Option<Program>,
    sort_program: Option<Program>,
    condition_program: Option<Program>,
}


pub enum SortOrder {
    Asc,
    Desc,
}


pub struct StreakSpan<T> {
    pub key: T,
    pub start: String,
    pub end: String,
    pub length: u32,
}


pub struct StreakEntry {
    pub game_id: String,
    pub result: bool,
}


impl<'a> CelExec<'a> {
    pub fn new() -> Self {
        let context = Context::default();

        Self {
            context,
            filter_program: None,
            sort_program: None,
            condition_program: None,
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

    pub fn streak_eval<'data, T, U>(&self, map: &'data HashMap<T, Vec<U>>) -> HashMap<&'data T, Vec<StreakEntry>>
        where T: Eq + Hash + Sync,
              U: Sync + CelEval + player::PlayerGamelog,
    {
        if let Some(ref program) = self.condition_program {
            let references = program.references();
            let variables = references.variables();
            let player_streaks: HashMap<_, _> = map.par_iter().map(|kv| {
                let (key, value) = kv;
                let bool_value: Vec<_> = value.iter().map(|e| {
                    let mut ctx = self.context.new_inner_scope();
                    let result;
                    if e.add_cel_variables(&mut ctx, &variables).is_err() {
                        result = false;
                    }
                    else {
                        result = match program.execute(&ctx) {
                            Ok(Value::Bool(true)) => true,
                            Ok(_) => false,
                            Err(error) => {
                                eprintln!("error evaluating: {error}");
                                false
                            }
                        };
                    }
                    let entry = StreakEntry {
                        game_id: e.game_id().to_string(),
                        result,
                    };
                    entry
                }).collect();
                (key, bool_value)
            }).collect();
            player_streaks
        }
        else {
            HashMap::new()
        }
    }

    pub fn find_streaks<T>(streak_map: &HashMap<T, Vec<StreakEntry>>) -> Vec<StreakSpan<&T>> {
        let mut streaks = Vec::with_capacity(150);
        let mut streak_minimum = 2;
        for (key, entries) in streak_map.iter() {
            let mut streak_start = None;
            let mut streak_end = None;
            let mut length = 0;
            for entry in entries {
                if entry.result {
                    length += 1;
                    if streak_start.is_none() {
                        streak_start = Some(&entry.game_id);
                    }
                    streak_end = Some(&entry.game_id);
                }
                else {
                    if let (Some(start), Some(end)) = (streak_start, streak_end) {
                        if length >= streak_minimum {
                            let span = StreakSpan {
                                key,
                                start: start.clone(),
                                end: end.clone(),
                                length,
                            };
                            streaks.push(span);
                        }
                    }
                    streak_start = None;
                    streak_end = None;
                    length = 0;
                }
            }

            // Check for streaks that end with the final entry of the Vec.
            if let (Some(start), Some(end)) = (streak_start, streak_end) {
                if length >= streak_minimum {
                    let span = StreakSpan {
                        key,
                        start: start.clone(),
                        end: end.clone(),
                        length,
                    };
                    streaks.push(span);
                }
            }

            // Sort the spans and check the 100th entry to see if the streak minimum length should
            // increase. If so, prune the list to only spans meeting the new minimum.
            streaks.sort_unstable_by(|a, b| b.length.cmp(&a.length));
            let mut prune_streaks = false;
            if let Some(span) = streaks.get(100) {
                if span.length > streak_minimum {
                    prune_streaks = true;
                    streak_minimum = span.length;
                }
            }
            if prune_streaks {
                streaks.retain(|span| span.length >= streak_minimum);
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
