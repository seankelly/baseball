use std::default::Default;
use std::io;

use csv::ReaderBuilder;
use serde::Serialize;
use serde_derive::Deserialize;

use crate::chadwick::bool_from_string;


#[derive(Deserialize)]
pub struct RawGame<'a> {
    // 0
    pub game_id: &'a str,
    pub date: &'a str,
    pub game_number: u16,
    pub day_of_week: &'a str,
    pub start_time: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub dh_used: bool,
    pub day_night: String,
    pub away_team: &'a str,
    pub home_team: &'a str,
    pub park_id: &'a str,
    // 10
    pub visitor_starting_pitcher: &'a str,
    pub home_starting_pitcher: &'a str,
    pub home_plate_umpire: &'a str,
    pub first_base_umpire: &'a str,
    pub second_base_umpire: &'a str,
    pub third_base_umpire: &'a str,
    pub left_field_umpire: &'a str,
    pub right_field_umpire: &'a str,
    pub attendance: u32,
    pub scorer: &'a str,
    // 20
    pub translator: &'a str,
    pub inputter: &'a str,
    pub input_time: &'a str,
    pub edit_time: &'a str,
    pub how_scored: &'a str,
    pub pitch_info: u8,
    pub temperature: u8,
    pub wind_direction: u8,
    pub wind_speed: i8, // Some games have negative wind speed listed.
    pub field_condition: u8,
    // 30
    pub precipitation: u8,
    pub sky: u8,
    pub game_time: u8,
    pub innings: u8,
    pub visitor_score: u8,
    pub home_score: u8,
    pub visitor_hits: Option<u8>,
    pub home_hits: Option<u8>,
    pub visitor_errors: Option<u8>,
    pub home_errors: Option<u8>,
    // 40
    pub visitor_left_on_base: Option<u8>,
    pub home_left_on_base: Option<u8>,
    pub winning_pitcher: &'a str,
    pub losing_pitcher: &'a str,
    pub save_pitcher: &'a str,
    pub gwrbi_player: &'a str,
    pub visitor_1: &'a str,
    pub visitor_1_pos: &'a str,
    pub visitor_2: &'a str,
    pub visitor_2_pos: &'a str,
    // 50
    pub visitor_3: &'a str,
    pub visitor_3_pos: &'a str,
    pub visitor_4: &'a str,
    pub visitor_4_pos: &'a str,
    pub visitor_5: &'a str,
    pub visitor_5_pos: &'a str,
    pub visitor_6: &'a str,
    pub visitor_6_pos: &'a str,
    pub visitor_7: &'a str,
    pub visitor_7_pos: &'a str,
    // 60
    pub visitor_8: &'a str,
    pub visitor_8_pos: &'a str,
    pub visitor_9: &'a str,
    pub visitor_9_pos: &'a str,
    pub home_1: &'a str,
    pub home_1_pos: &'a str,
    pub home_2: &'a str,
    pub home_2_pos: &'a str,
    pub home_3: &'a str,
    pub home_3_pos: &'a str,
    // 70
    pub home_4: &'a str,
    pub home_4_pos: &'a str,
    pub home_5: &'a str,
    pub home_5_pos: &'a str,
    pub home_6: &'a str,
    pub home_6_pos: &'a str,
    pub home_7: &'a str,
    pub home_7_pos: &'a str,
    pub home_8: &'a str,
    pub home_8_pos: &'a str,
    // 80
    pub home_9: &'a str,
    pub home_9_pos: &'a str,
    pub visitor_finishing_pitcher: &'a str,
    pub home_finishing_pitcher: &'a str,
    // 84
    pub game_type: &'a str,
}

#[derive(Default, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Game {
    // 0
    pub game_id: String,
    pub date: String,
    pub game_number: u16,
    pub start_time: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub dh_used: bool,
    pub day_night: String,
    pub away_team: String,
    pub home_team: String,
    pub park: String,
    pub attendance: u32,
    // Field 10
    pub pitch_info: u8,
    pub temperature: u8,
    pub wind_direction: u8,
    pub wind_speed: i8, // Some games have negative wind speed listed.
    pub field_condition: u8,
    pub precipitation: u8,
    pub sky: u8,
    pub game_time: u16,
    pub innings: u8,
    pub game_type: String,
}


impl Game {
    pub fn load<T: io::Read>(file: T) -> Vec<Self> {
        let mut games = Vec::new();
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);
        for result in reader.deserialize() {
            match result {
                Ok(game) => {
                    games.push(game);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        games
    }
}
