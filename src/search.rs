use std::error::Error;

use cel_interpreter::{Context, Program, Value};
use rayon::prelude::*;


#[derive(Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}


pub trait CelSearch {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>>;
}


pub struct Search<'a> {
    context: Context<'a>,
    filter_program: Option<Program>,
    sort_program: Option<Program>,
}


impl<'a> Search<'a> {
    pub fn new(filter: Option<&str>, sort: Option<&str>) -> Result<Self, Box<dyn Error>> {
        let filter_program = match filter {
            Some(source) => Some(Program::compile(source)?),
            None => None,
        };
        let sort_program = match sort {
            Some(source) => Some(Program::compile(source)?),
            None => None,
        };
        let context = Context::default();

        Ok(Self {
            context,
            filter_program,
            sort_program,
        })
    }

    pub fn filter<T: CelSearch>(&self, input: &mut Vec<T>) {
        /*
        match self.filter_program.as_ref() {
            Some(filter_program) => {
                input
                    .into_par_iter()
                    .filter(|element| self.filter_option(element, &filter_program)).collect()
            }
            None => input,
        }
        */
        if let Some(filter_program) = self.filter_program.as_ref() {
            input.retain(|element| self.filter_option(element, &filter_program));
        }
    }

    pub fn sort<T: CelSearch + Send>(&self, input: &mut Vec<T>, sort_order: &SortOrder) {
        if let Some(sort_program) = self.sort_program.as_ref() {
            input.par_sort_unstable_by(|a, b| {
                let a_res = self.sort_key(a, &sort_program);
                let b_res = self.sort_key(b, &sort_program);
                match sort_order {
                    SortOrder::Asc => { a_res.total_cmp(&b_res) }
                    SortOrder::Desc => { b_res.total_cmp(&a_res) }
                }
            });
        }
    }

    fn filter_option<T: CelSearch>(&self, element: &T, program: &Program) -> bool {
        let mut player_ctx = self.context.new_inner_scope();
        if let Err(_) = element.add_cel_variables(&mut player_ctx) {
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

    fn sort_key<T: CelSearch>(&self, career: &T, program: &Program) -> f64 {
        let mut player_ctx = self.context.new_inner_scope();
        if let Err(_) = career.add_cel_variables(&mut player_ctx) {
            return f64::NEG_INFINITY;
        }

        let result = program.execute(&player_ctx);
        let f_result = match result {
            Ok(Value::Int(i)) => { i as f64 }
            Ok(Value::UInt(u)) => { u as f64 }
            Ok(Value::Float(f)) => { f }
            _ => f64::INFINITY
        };
        f_result
    }
}
