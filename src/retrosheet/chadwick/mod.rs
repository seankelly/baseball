use std::path::Path;

use csv::ReaderBuilder;
use serde::Deserialize;


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
                            .has_headers(false)
                            .from_path(file)
                            .expect("Couldn't open file.");
    let mut records = Vec::new();
    for record in csv_reader.deserialize() {
        let record: T = record.expect("Couldn't decode record");
        records.push(record);
    }
    return records;
}
