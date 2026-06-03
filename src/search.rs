use std::collections::HashMap;
use std::error::Error;

use cel::{Context, Program, Value};
use rayon::prelude::*;


pub enum SortOrder {
    Asc,
    Desc,
}


pub trait CelEval {
    fn add_cel_variables(&self, context: &mut Context, variables: &[&str]) -> Result<(), Box<dyn Error>>;
}


pub struct CelExec<'a> {
    context: Context<'a>,
    filter_program: Option<Program>,
    sort_program: Option<Program>,
    condition_program: Option<Program>,
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

    /*
    fn streak_eval<'a, T, U: CelEval>(kv: (&'a T, &'a Vec<U>), context: &Context, program: &Program, variables: &[&str]) -> (&'a T, Vec<bool>)
    {
        let (key, value) = kv;
        let bool_value: Vec<_> = value.iter().map(|e| {
            let mut ctx = context.new_inner_scope();
            if e.add_cel_variables(&mut ctx, &variables).is_err() {
                return false;
            }
            match program.execute(&ctx) {
                Ok(Value::Bool(true)) => true,
                Ok(_) => false,
                Err(error) => {
                    eprintln!("error evaluating: {error}");
                    false
                }
            }
        }).collect();
        (key, bool_value)
    }

    pub fn check_map<K, V>(&self, input: &HashMap<K, V>)
        where V: IntoIterator, V::Item: CelEval
    {
        if let Some(conditional) = self.condition_program.as_ref() {
            let _streak_map = input.values().map(|val| {
                val.iter().map(|v| {})
            });
        }
    }

    fn condition_map<V>(&self, elements: &V, program: &Program) -> Vec<bool>
        where V: IntoIterator, V::Item: CelEval
    {
        elements.iter().map(|v| self.run_condition(v, program)).collect()
    }

    fn run_condition<T: CelEval>(&self, element: T, program: &Program) -> bool {
        let mut ctx = self.context.new_inner_scope();
        if element.add_cel_variables(&mut ctx).is_err() {
            return false;
        }
        match program.execute(&ctx) {
            Ok(Value::Bool(true)) => true,
            Ok(_) => false,
            Err(error) => {
                eprintln!("error evaluating: {error}");
                false
            }
        }
    }
    */

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
