use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::io;

use csv::ReaderBuilder;


#[derive(Default)]
pub struct Register {
    person: Vec<Person>,

    /// Map Retrosheet player ID to an index in person.
    retro_to_bbref: HashMap<String, usize>
}


impl Register {
    pub fn load<T: io::Read>(&mut self, file: T) {
        let mut reader = ReaderBuilder::new().from_reader(file);
        for result in reader.deserialize() {
            if let Ok(person) = result {
                self.person.push(person);
            }
        }
    }

    pub fn from_file<T: io::Read>(file: T) -> Self {
        let mut register = Register::default();
        register.load(file);
        return register;
    }

    pub fn build_retro_map(&mut self) {
        if self.retro_to_bbref.len() == self.person.len() {
            return;
        }
        else {
            self.retro_to_bbref.clear();
        }

        self.retro_to_bbref.reserve(self.person.len());

        for (idx, person) in self.person.iter().enumerate() {
            self.retro_to_bbref.insert(person.key_retro.clone(), idx);
        }
    }

    pub fn map_retro_to_bbref(&self, retro_id: &str) -> Option<&String> {
        self.retro_to_bbref.get(retro_id)
            .and_then(|idx| self.person.get(*idx))
            .map(|person| &person.key_bbref)
    }
}


#[derive(serde::Deserialize)]
pub struct Person {
    key_person: String,
    key_uuid: String,
    key_mlbam: String,
    key_retro: String,
    key_bbref: String,
    key_bbref_minors: String,
    key_fangraphs: String,
    key_npb: String,
    key_sr_nfl: String,
    key_sr_nba: String,
    key_sr_nhl: String,
    key_wikidata: String,
    name_last: String,
    name_first: String,
    name_given: String,
    name_suffix: String,
    name_matrilineal: String,
    name_nick: String,
    birth_year: String,
    birth_month: String,
    birth_day: String,
    death_year: String,
    death_month: String,
    death_day: String,
    pro_played_first: String,
    pro_played_last: String,
    mlb_played_first: String,
    mlb_played_last: String,
    col_played_first: String,
    col_played_last: String,
    pro_managed_first: String,
    pro_managed_last: String,
    mlb_managed_first: String,
    mlb_managed_last: String,
    col_managed_first: String,
    col_managed_last: String,
    pro_umpired_first: String,
    pro_umpired_last: String,
    mlb_umpired_first: String,
    mlb_umpired_last: String,
}
