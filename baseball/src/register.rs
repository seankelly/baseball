use std::collections::HashMap;
use std::default::Default;
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
    pub key_person: String,
    pub key_uuid: String,
    pub key_mlbam: String,
    pub key_retro: String,
    pub key_bbref: String,
    pub key_bbref_minors: String,
    pub key_fangraphs: String,
    pub key_npb: String,
    pub key_sr_nfl: String,
    pub key_sr_nba: String,
    pub key_sr_nhl: String,
    pub key_wikidata: String,
    pub name_last: String,
    pub name_first: String,
    pub name_given: String,
    pub name_suffix: String,
    pub name_matrilineal: String,
    pub name_nick: String,
    pub birth_year: String,
    pub birth_month: String,
    pub birth_day: String,
    pub death_year: String,
    pub death_month: String,
    pub death_day: String,
    pub pro_played_first: String,
    pub pro_played_last: String,
    pub mlb_played_first: String,
    pub mlb_played_last: String,
    pub col_played_first: String,
    pub col_played_last: String,
    pub pro_managed_first: String,
    pub pro_managed_last: String,
    pub mlb_managed_first: String,
    pub mlb_managed_last: String,
    pub col_managed_first: String,
    pub col_managed_last: String,
    pub pro_umpired_first: String,
    pub pro_umpired_last: String,
    pub mlb_umpired_first: String,
    pub mlb_umpired_last: String,
}
