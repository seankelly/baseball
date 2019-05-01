use std::path::Path;

use csv::ReaderBuilder;
use serde::de::{self, Deserialize, Deserializer, Unexpected};


pub mod events;
pub mod games;

pub use events::Event;
pub use events::EventExtended;
pub use games::GameLog;
pub use games::TeamGameLog;


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

fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "T" => Ok(true),
        "F" => Ok(false),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"OK or nOK",
        )),
    }
}
