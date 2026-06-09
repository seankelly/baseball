use std::error::Error;

use rusqlite::{Row, Statement, Transaction};

pub trait Sql where Self: Sized {
    fn create_table(tx: &mut Transaction) -> Result<(), Box<dyn Error>>;

    fn table_name<'a>() -> &'a str;

    fn read_row(row: &Row, offset: usize) -> Result<Self, rusqlite::Error>;

    fn write_row(&self, statement: &mut Statement) -> Result<usize, rusqlite::Error>;

    fn column_names<'a>() -> Vec<&'a str>;
}
