use std::cmp::Reverse;
use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::hash::Hash;

use cel::{Context, Program, Value};
use rayon::prelude::*;
use tracing::trace;


const DEFAULT_RESULT_LIMIT: usize = 10;

pub trait CelEval {
    fn add_cel_variables(&self, context: &mut Context, variables: &[&str]) -> Result<(), Box<dyn Error>>;

    fn check_cel_variables(&self, variables: &[&str]) -> bool;
}


pub trait SearchKey {
    fn id(&self) -> &str;

    fn subject_id(&self) -> &str;

    fn order(&self, career: bool) -> u16;
}

pub struct CelExec<'a> {
    context: Context<'a>,
    career_mode: bool,
    result_limit: usize,
    game_start: Option<u16>,
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
    /// The first game number in the streak.
    pub game_start: u16,
    /// The length of the streak in games.
    pub length: u32,
    /// The length of the streak in either games or another countable statistic (e.g. plate
    /// appearances).
    pub count: u32,
}


pub struct StreakEntry {
    pub game_id: String,
    pub order: u16,
    pub result: bool,
    pub count: u8,
}


pub struct WindowEntry {
    pub id: String,
    pub start: String,
    pub end: String,
    pub count: u32,
}


impl<'a> CelExec<'a> {
    pub fn new(limit: usize, career_mode: bool) -> Self {
        let context = Context::default();

        Self {
            context,
            career_mode,
            result_limit: limit,
            game_start: None,
            filter_program: None,
            sort_program: None,
            condition_program: None,
            count_program: None,
        }
    }

    pub fn set_career_mode(&mut self, mode: bool) {
        self.career_mode = mode;
    }

    pub fn set_limit(&mut self, limit: usize) {
        self.result_limit = limit;
    }

    pub fn set_game_start(&mut self, start: u16) {
        self.game_start = Some(start);
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
              U: Sync + CelEval + SearchKey,
    {
        if let Some(ref program) = self.condition_program {
            let references = program.references();
            let variables = references.variables();
            let player_streaks: HashMap<_, _> = map.par_iter().map(|kv| {
                let (key, value) = kv;
                let entries = Self::eval_slice(value, &self.context, program, &variables)
                    .iter().map(|i| self.streak_eval_each(i)).collect();
                (key, entries)
            }).collect();
            player_streaks
        }
        else {
            HashMap::new()
        }
    }

    fn streak_eval_each<T>(&self, item: &(&T, Value)) -> StreakEntry
        where T: CelEval + SearchKey,
    {
        let (element, value) = item;
        let result = matches!(value, Value::Bool(true));
        let mut count = 0;

        if result {
            count = 1;
            if let Some(ref program) = self.count_program {
                let references = program.references();
                let variables = references.variables();
                let mut ctx = self.context.new_inner_scope();
                if element.add_cel_variables(&mut ctx, &variables).is_ok() {
                    count = match program.execute(&ctx) {
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
            game_id: element.id().to_string(),
            order: element.order(self.career_mode),
            result,
            count,
        }
    }

    pub fn find_streaks(&self, streak_map: &HashMap<&Key, Vec<StreakEntry>>) -> Vec<StreakSpan> {
        let mut streaks = Vec::with_capacity(150);
        let mut streak_minimum = 2;
        for (key, entries) in streak_map.iter() {
            let mut streak_start = None;
            let mut streak_end = None;
            let mut game_start = None;
            let mut length = 0;
            let mut count = 0;
            for entry in entries {
                if let Some(anchor_start) = self.game_start {
                    // Too early to start searching for a streak, try next entry.
                    if entry.order < anchor_start {
                        continue;
                    }
                    else if entry.order > anchor_start {
                        if let Some(game) = game_start {
                            // Started another streak but after the desired anchor. Clear the start
                            // and skip further processing of this key.
                            if game > anchor_start {
                                streak_start = None;
                                game_start = None;
                                break;
                            }
                        }
                        // If don't already have a streak started, or it already finished, skip
                        // further processing of this key.
                        else if game_start.is_none() {
                            streak_start = None;
                            game_start = None;
                            break;
                        }
                    }
                }

                if entry.result {
                    length += 1;
                    count += entry.count as u32;
                    if streak_start.is_none() {
                        streak_start = Some(&entry.game_id);
                    }
                    if game_start.is_none() {
                        game_start = Some(entry.order);
                    }
                    streak_end = Some(&entry.game_id);
                }
                else {
                    if let (Some(start), Some(end), Some(game_start)) = (streak_start, streak_end, game_start) && count >= streak_minimum {
                        let span = StreakSpan {
                            id: key.id.to_owned(),
                            start: start.clone(),
                            end: end.clone(),
                            game_start,
                            length,
                            count,
                        };
                        streaks.push(span);
                    }
                    streak_start = None;
                    streak_end = None;
                    game_start = None;
                    length = 0;
                    count = 0;
                }
            }

            // Check for streaks that end with the final entry of the Vec or if the loop ended
            // early.
            if let (Some(start), Some(end), Some(game_start)) = (streak_start, streak_end, game_start) && count >= streak_minimum {
                let span = StreakSpan {
                    id: key.id.to_owned(),
                    start: start.clone(),
                    end: end.clone(),
                    game_start,
                    length,
                    count,
                };
                streaks.push(span);
            }

            // Sort the spans and check the 100th entry to see if the streak minimum length should
            // increase. If so, prune the list to only spans meeting the new minimum.
            streaks.sort_unstable_by_key(|a| Reverse(a.count));
            if let Some(span) = streaks.get(self.result_limit) && span.count > streak_minimum {
                streak_minimum = span.count;
                trace!(streak_minimum = streak_minimum, streaks = streaks.len(), "Increasing streak minimum");
                streaks.retain(|span| span.count >= streak_minimum);
            }
        }

        streaks
    }

    pub fn window_eval<'data, T, U>(&self, map: &'data HashMap<T, Vec<U>>, size: usize) -> HashMap<&'data T, Vec<WindowEntry>>
        where T: Eq + Hash + Sync,
              U: Sync + CelEval + SearchKey,
    {
        if let Some(ref program) = self.count_program {
            let player_windows: HashMap<_, _> = map.par_iter().map(|kv| {
                let (key, value) = kv;
                let references = program.references();
                let variables = references.variables();
                // Run the program (the slowest part) on every element in the Vec.
                let processed_values = Self::eval_slice(value, &self.context, program, &variables);
                // The windows method will skip any processing if the input Vec is shorter than the
                // chosen size. I don't want to skip those windows so check the size and manually
                // run the inner method on the whole thing if too short.
                let entries: Vec<_> = if processed_values.len() >= size {
                    processed_values.windows(size).map(|w| self.window_eval_each(w)).collect()
                }
                else {
                    vec![self.window_eval_each(&processed_values)]
                };
                (key, entries)
            }).collect();
            player_windows
        }
        else {
            HashMap::new()
        }
    }

    /// Evaluate every item within the window to produce a count from the program.
    fn window_eval_each<T>(&self, window: &[(&T, Value)]) -> WindowEntry
        where T: SearchKey,
    {
        let mut count = 0;
        let start = window.first();
        let end = window.last();
        for (_item, value) in window {
            let item_count = match value {
                Value::Int(i) => { *i as u32 }
                Value::UInt(u) => { *u as u32 }
                Value::Bool(true) => { 1 }
                _ => 0,
            };
            count += item_count;
        }

        WindowEntry {
            id: start.map_or("id", |e| e.0.subject_id()).to_owned(),
            start: start.map_or("unknown", |e| e.0.id()).to_owned(),
            end: end.map_or("unknown", |e| e.0.id()).to_owned(),
            count,
        }
    }

    pub fn sort_windows<'data, T>(&self, window_map: &'data HashMap<&T, Vec<WindowEntry>>) -> Vec<&'data WindowEntry> {
        let mut windows = Vec::with_capacity(150);
        let mut minimum_count = 1;
        for entries in window_map.values() {
            for window in entries.iter() {
                if window.count >= minimum_count {
                    windows.push(window);
                }
            }

            // Sort the seen windows so far and check the result limit entry to see if the minimum
            // count should increase. If so, prune the list to the new minimum.
            windows.sort_unstable_by_key(|w| Reverse(w.count));
            if let Some(window) = windows.get(self.result_limit) && window.count > minimum_count {
                minimum_count = window.count;
                trace!(minimum_count = minimum_count, windows = windows.len(), "Increase window minimum count");
                windows.retain(|window| window.count >= minimum_count);
            }
        }

        windows
    }

    /// Run CEL program on every item on the provided slice, returning a Vec of tuples containing
    /// the item and result of the CEL program.
    fn eval_slice<'data, T: CelEval>(items: &'data [T], context: &Context, program: &Program, variables: &[&str]) -> Vec<(&'data T, Value)> {
        items.iter().map(|item| {
            let mut ctx = context.new_inner_scope();
            let result = if item.add_cel_variables(&mut ctx, variables).is_err() {
                Value::Null
            }
            else {
                match program.execute(&ctx) {
                    Ok(v) => v,
                    Err(_) => Value::Null,
                }
            };
            (item, result)
        }).collect()
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

impl<'a> Default for CelExec<'a> {
    fn default() -> Self {
        let context = Context::default();

        Self {
            context,
            career_mode: false,
            result_limit: DEFAULT_RESULT_LIMIT,
            game_start: None,
            filter_program: None,
            sort_program: None,
            condition_program: None,
            count_program: None,
        }
    }
}
