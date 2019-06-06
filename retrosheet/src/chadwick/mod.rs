use std::path::Path;

use csv::ReaderBuilder;
use csv::DeserializeRecordsIntoIter;
use serde::de::{self, Deserialize, Deserializer, Unexpected};


pub mod events;
pub mod games;

pub use events::Event;
pub use events::EventExtended;
pub use events::Handedness;
pub use games::GameLog;
pub use games::TeamGameLog;


pub struct ChadwickFileIter<T> {
    records: DeserializeRecordsIntoIter<std::fs::File, T>,
}


pub fn load_file<T>(file: &Path) -> Vec<T>
    where for<'de> T: Deserialize<'de>
{
    let mut csv_reader = ReaderBuilder::new()
                            .has_headers(true)
                            .from_path(file)
                            .expect("Couldn't open file.");
    let mut records = Vec::new();
    for record in csv_reader.deserialize() {
        let record: T = record.expect("Couldn't decode record");
        records.push(record);
    }
    return records;
}


impl<T> Iterator for ChadwickFileIter<T>
    where for<'de> T: Deserialize<'de>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.records.next()
            .map(|result| {
                let record: Self::Item  = result.expect("Couldn't decode record");
                record
            })
    }
}

pub fn load_file_iter<T>(file: &Path) -> ChadwickFileIter<T>
    where for<'de> T: Deserialize<'de>
{
    let mut csv_reader = ReaderBuilder::new()
                            .has_headers(true)
                            .from_path(file)
                            .expect("Couldn't open file.");
    let records = csv_reader.into_deserialize();
    ChadwickFileIter {
        records: records,
    }
}

fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "T" => Ok(true),
        "F" => Ok(false),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"T or F",
        )),
    }
}

fn parse_handedness<'de, D>(deserializer: D) -> Result<Handedness, D::Error>
    where D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "?" => Ok(Handedness::Unknown),
        "B" => Ok(Handedness::Both),
        "L" => Ok(Handedness::Left),
        "R" => Ok(Handedness::Right),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"L or R",
        )),
    }
}
