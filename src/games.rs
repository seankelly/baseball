use std::collections::HashMap;
use std::error::Error;

use baseball::retrosheet::game;

use crate::search::{CelEval, SearchKey};
use crate::database::Sql;

use cel::{Context, Value};
use chrono::Datelike;
use rusqlite::{Row, Statement, Transaction, named_params};
use rusqlite::types::{FromSql, FromSqlResult, Null, ToSql, ToSqlOutput, ValueRef};


#[derive(Clone, Debug)]
pub enum RetrosheetOption {
    None,
    Unknown,
    Some(u8),
}


#[derive(Clone, Debug)]
pub struct Linescore {
    linescore: Vec<Option<u8>>,
}


#[derive(Clone, Debug)]
pub struct GameLog {
    /// This game ID used on many sites.
    pub game_id: String,
    // 1
    pub date: chrono::NaiveDate,
    // These fields are copied straight from GameLog with some movement.
    pub number_of_game: String,
    pub day_of_week: String,
    pub visitor_team: String,
    pub visitor_league: String,
    pub visitor_team_game_number: u16,
    pub home_team: String,
    pub home_league: String,
    pub home_team_game_number: u16,
    // 10
    pub visitor_score: u8,
    pub home_score: u8,
    pub number_of_outs: Option<u8>,
    pub day_night: String,
    pub completion_info: String,
    pub forfeit_info: String,
    pub protest_info: String,
    pub park_id: String,
    pub attendance: Option<i32>,
    pub time_of_game: Option<u16>,
    // 20
    pub visitor_line_score: Linescore,
    pub home_line_score: Linescore,
    pub visitor_ab: Option<u8>,
    pub visitor_hits: Option<u8>,
    pub visitor_doubles: RetrosheetOption,
    pub visitor_triples: RetrosheetOption,
    pub visitor_homeruns: RetrosheetOption,
    pub visitor_rbi: RetrosheetOption,
    pub visitor_sac_hits: Option<u8>,
    pub visitor_sac_flies: RetrosheetOption,
    // 30
    pub visitor_hbp: RetrosheetOption,
    pub visitor_walks: RetrosheetOption,
    pub visitor_intentional_walks: RetrosheetOption,
    pub visitor_strikeouts: RetrosheetOption,
    pub visitor_stolen_bases: RetrosheetOption,
    pub visitor_caught_stealing: RetrosheetOption,
    pub visitor_gidp: RetrosheetOption,
    pub visitor_catcher_interference: RetrosheetOption,
    pub visitor_left_on_base: RetrosheetOption,
    pub visitor_pitchers_used: Option<u8>,
    // 40
    pub visitor_individual_earned_runs: RetrosheetOption,
    pub visitor_team_earned_runs: Option<u8>,
    pub visitor_wild_pitches: RetrosheetOption,
    pub visitor_balks: Option<u8>,
    pub visitor_putouts: RetrosheetOption,
    pub visitor_assists: RetrosheetOption,
    pub visitor_errors: RetrosheetOption,
    pub visitor_passed_balls: RetrosheetOption,
    pub visitor_double_plays: RetrosheetOption,
    pub visitor_triple_plays: Option<u8>,
    // 50
    pub home_ab: Option<u8>,
    pub home_hits: Option<u8>,
    pub home_doubles: RetrosheetOption,
    pub home_triples: RetrosheetOption,
    pub home_homeruns: RetrosheetOption,
    pub home_rbi: RetrosheetOption,
    pub home_sac_hits: Option<u8>,
    pub home_sac_flies: RetrosheetOption,
    pub home_hbp: RetrosheetOption,
    pub home_walks: RetrosheetOption,
    // 60
    pub home_intentional_walks: RetrosheetOption,
    pub home_strikeouts: RetrosheetOption,
    pub home_stolen_bases: RetrosheetOption,
    pub home_caught_stealing: RetrosheetOption,
    pub home_gidp: RetrosheetOption,
    pub home_catcher_interference: RetrosheetOption,
    pub home_left_on_base: RetrosheetOption,
    pub home_pitchers_used: Option<u8>,
    pub home_individual_earned_runs: RetrosheetOption,
    pub home_team_earned_runs: Option<u8>,
    // 70
    pub home_wild_pitches: RetrosheetOption,
    pub home_balks: Option<u8>,
    pub home_putouts: RetrosheetOption,
    pub home_assists: RetrosheetOption,
    pub home_errors: RetrosheetOption,
    pub home_passed_balls: RetrosheetOption,
    pub home_double_plays: RetrosheetOption,
    pub home_triple_plays: Option<u8>,
    pub home_plate_umpire_name: String,
    pub home_plate_umpire_id: String,
    // 80
    pub first_base_umpire_name: String,
    pub first_base_umpire_id: String,
    pub second_base_umpire_name: String,
    pub second_base_umpire_id: String,
    pub third_base_umpire_name: String,
    pub third_base_umpire_id: String,
    pub left_field_umpire_name: String,
    pub left_field_umpire_id: String,
    pub right_field_umpire_name: String,
    pub right_field_umpire_id: String,
    // 90
    pub visitor_manager_id: String,
    pub visitor_manager_name: String,
    pub home_manager_id: String,
    pub home_manager_name: String,
    pub winning_pitcher_name: String,
    pub winning_pitcher_id: String,
    pub losing_pitcher_name: String,
    pub losing_pitcher_id: String,
    pub saving_pitcher_name: String,
    pub saving_pitcher_id: String,
    // 100
    pub gwrbi_player_name: String,
    pub gwrbi_player_id: String,
    pub visitor_starter_name: String,
    pub visitor_starter_id: String,
    pub home_starter_name: String,
    pub home_starter_id: String,
    pub visitor_1_id: String,
    pub visitor_1_name: String,
    pub visitor_1_pos: String,
    pub visitor_2_id: String,
    // 110
    pub visitor_2_name: String,
    pub visitor_2_pos: String,
    pub visitor_3_id: String,
    pub visitor_3_name: String,
    pub visitor_3_pos: String,
    pub visitor_4_id: String,
    pub visitor_4_name: String,
    pub visitor_4_pos: String,
    pub visitor_5_id: String,
    pub visitor_5_name: String,
    // 120
    pub visitor_5_pos: String,
    pub visitor_6_id: String,
    pub visitor_6_name: String,
    pub visitor_6_pos: String,
    pub visitor_7_id: String,
    pub visitor_7_name: String,
    pub visitor_7_pos: String,
    pub visitor_8_id: String,
    pub visitor_8_name: String,
    pub visitor_8_pos: String,
    // 130
    pub visitor_9_id: String,
    pub visitor_9_name: String,
    pub visitor_9_pos: String,
    pub home_1_id: String,
    pub home_1_name: String,
    pub home_1_pos: String,
    pub home_2_id: String,
    pub home_2_name: String,
    pub home_2_pos: String,
    pub home_3_id: String,
    // 140
    pub home_3_name: String,
    pub home_3_pos: String,
    pub home_4_id: String,
    pub home_4_name: String,
    pub home_4_pos: String,
    pub home_5_id: String,
    pub home_5_name: String,
    pub home_5_pos: String,
    pub home_6_id: String,
    pub home_6_name: String,
    // 150
    pub home_6_pos: String,
    pub home_7_id: String,
    pub home_7_name: String,
    pub home_7_pos: String,
    pub home_8_id: String,
    pub home_8_name: String,
    pub home_8_pos: String,
    pub home_9_id: String,
    pub home_9_name: String,
    pub home_9_pos: String,
    // 160
    pub additional_info: String,
    pub acquisition_info: String,
}


/// Game log with all player, manager, and umpire fields removed.
#[derive(Clone, Debug)]
pub struct GameLogSmall {
    /// This game ID used on many sites.
    pub game_id: String,
    // 1
    pub date: chrono::NaiveDate,
    // These fields are copied straight from GameLog with some movement.
    pub number_of_game: String,
    pub visitor_team: String,
    pub visitor_league: String,
    pub visitor_team_game_number: u16,
    pub home_team: String,
    pub home_league: String,
    pub home_team_game_number: u16,
    // 10
    pub visitor_score: u8,
    pub home_score: u8,
    pub number_of_outs: Option<u8>,
    pub day_night: String,
    pub completion_info: String,
    pub forfeit_info: String,
    pub protest_info: String,
    pub park_id: String,
    pub attendance: Option<i32>,
    pub time_of_game: Option<u16>,
    // 20
    pub visitor_line_score: Linescore,
    pub home_line_score: Linescore,
    pub visitor_ab: Option<u8>,
    pub visitor_hits: Option<u8>,
    pub visitor_doubles: RetrosheetOption,
    pub visitor_triples: RetrosheetOption,
    pub visitor_homeruns: RetrosheetOption,
    pub visitor_rbi: RetrosheetOption,
    pub visitor_sac_hits: Option<u8>,
    pub visitor_sac_flies: RetrosheetOption,
    // 30
    pub visitor_hbp: RetrosheetOption,
    pub visitor_walks: RetrosheetOption,
    pub visitor_intentional_walks: RetrosheetOption,
    pub visitor_strikeouts: RetrosheetOption,
    pub visitor_stolen_bases: RetrosheetOption,
    pub visitor_caught_stealing: RetrosheetOption,
    pub visitor_gidp: RetrosheetOption,
    pub visitor_catcher_interference: RetrosheetOption,
    pub visitor_left_on_base: RetrosheetOption,
    pub visitor_pitchers_used: Option<u8>,
    // 40
    pub visitor_individual_earned_runs: RetrosheetOption,
    pub visitor_team_earned_runs: Option<u8>,
    pub visitor_wild_pitches: RetrosheetOption,
    pub visitor_balks: Option<u8>,
    pub visitor_putouts: RetrosheetOption,
    pub visitor_assists: RetrosheetOption,
    pub visitor_errors: RetrosheetOption,
    pub visitor_passed_balls: RetrosheetOption,
    pub visitor_double_plays: RetrosheetOption,
    pub visitor_triple_plays: Option<u8>,
    // 50
    pub home_ab: Option<u8>,
    pub home_hits: Option<u8>,
    pub home_doubles: RetrosheetOption,
    pub home_triples: RetrosheetOption,
    pub home_homeruns: RetrosheetOption,
    pub home_rbi: RetrosheetOption,
    pub home_sac_hits: Option<u8>,
    pub home_sac_flies: RetrosheetOption,
    pub home_hbp: RetrosheetOption,
    pub home_walks: RetrosheetOption,
    // 60
    pub home_intentional_walks: RetrosheetOption,
    pub home_strikeouts: RetrosheetOption,
    pub home_stolen_bases: RetrosheetOption,
    pub home_caught_stealing: RetrosheetOption,
    pub home_gidp: RetrosheetOption,
    pub home_catcher_interference: RetrosheetOption,
    pub home_left_on_base: RetrosheetOption,
    pub home_pitchers_used: Option<u8>,
    pub home_individual_earned_runs: RetrosheetOption,
    pub home_team_earned_runs: Option<u8>,
    // 70
    pub home_wild_pitches: RetrosheetOption,
    pub home_balks: Option<u8>,
    pub home_putouts: RetrosheetOption,
    pub home_assists: RetrosheetOption,
    pub home_errors: RetrosheetOption,
    pub home_passed_balls: RetrosheetOption,
    pub home_double_plays: RetrosheetOption,
    pub home_triple_plays: Option<u8>,
}


#[derive(Clone, Debug)]
pub struct TeamGameLog {
    // This game ID used on many sites.
    pub game_id: String,
    // Other synthetic fields to simplify common queries.
    pub w: bool,
    pub l: bool,
    pub t: bool,
    // 1
    pub date: chrono::NaiveDate,
    // These fields are copied straight from GameLog with some movement.
    pub number_of_game: String,
    pub day_of_week: String,
    pub team: String,
    pub league: String,
    pub team_game_number: u16,
    pub opponent_team: String,
    pub opponent_league: String,
    pub opponent_team_game_number: u16,
    // 10
    pub score: u8,
    pub opponent_score: u8,
    pub number_of_outs: Option<u8>,
    pub day_night: String,
    pub completion_info: String,
    pub forfeit_info: String,
    pub protest_info: String,
    pub park_id: String,
    pub attendance: Option<i32>,
    pub time_of_game: Option<u16>,
    // 20
    pub line_score: Linescore,
    pub opponent_line_score: Linescore,
    pub ab: Option<u8>,
    pub hits: Option<u8>,
    pub doubles: RetrosheetOption,
    pub triples: RetrosheetOption,
    pub homeruns: RetrosheetOption,
    pub rbi: RetrosheetOption,
    pub sac_hits: Option<u8>,
    pub sac_flies: RetrosheetOption,
    // 30
    pub hbp: RetrosheetOption,
    pub walks: RetrosheetOption,
    pub intentional_walks: RetrosheetOption,
    pub strikeouts: RetrosheetOption,
    pub stolen_bases: RetrosheetOption,
    pub caught_stealing: RetrosheetOption,
    pub gidp: RetrosheetOption,
    pub catcher_interference: RetrosheetOption,
    pub left_on_base: RetrosheetOption,
    pub pitchers_used: Option<u8>,
    // 40
    pub individual_earned_runs: RetrosheetOption,
    pub team_earned_runs: Option<u8>,
    pub wild_pitches: RetrosheetOption,
    pub balks: Option<u8>,
    pub putouts: RetrosheetOption,
    pub assists: RetrosheetOption,
    pub errors: RetrosheetOption,
    pub passed_balls: RetrosheetOption,
    pub double_plays: RetrosheetOption,
    pub triple_plays: Option<u8>,
    // 50
    pub opponent_ab: Option<u8>,
    pub opponent_hits: Option<u8>,
    pub opponent_doubles: RetrosheetOption,
    pub opponent_triples: RetrosheetOption,
    pub opponent_homeruns: RetrosheetOption,
    pub opponent_rbi: RetrosheetOption,
    pub opponent_sac_hits: Option<u8>,
    pub opponent_sac_flies: RetrosheetOption,
    pub opponent_hbp: RetrosheetOption,
    pub opponent_walks: RetrosheetOption,
    // 60
    pub opponent_intentional_walks: RetrosheetOption,
    pub opponent_strikeouts: RetrosheetOption,
    pub opponent_stolen_bases: RetrosheetOption,
    pub opponent_caught_stealing: RetrosheetOption,
    pub opponent_gidp: RetrosheetOption,
    pub opponent_catcher_interference: RetrosheetOption,
    pub opponent_left_on_base: RetrosheetOption,
    pub opponent_pitchers_used: Option<u8>,
    pub opponent_individual_earned_runs: RetrosheetOption,
    pub opponent_team_earned_runs: Option<u8>,
    // 70
    pub opponent_wild_pitches: RetrosheetOption,
    pub opponent_balks: Option<u8>,
    pub opponent_putouts: RetrosheetOption,
    pub opponent_assists: RetrosheetOption,
    pub opponent_errors: RetrosheetOption,
    pub opponent_passed_balls: RetrosheetOption,
    pub opponent_double_plays: RetrosheetOption,
    pub opponent_triple_plays: Option<u8>,
    pub home_plate_umpire_name: String,
    pub home_plate_umpire_id: String,
    // 80
    pub first_base_umpire_name: String,
    pub first_base_umpire_id: String,
    pub second_base_umpire_name: String,
    pub second_base_umpire_id: String,
    pub third_base_umpire_name: String,
    pub third_base_umpire_id: String,
    pub left_field_umpire_name: String,
    pub left_field_umpire_id: String,
    pub right_field_umpire_name: String,
    pub right_field_umpire_id: String,
    // 90
    pub manager_id: String,
    pub manager_name: String,
    pub opponent_manager_id: String,
    pub opponent_manager_name: String,
    pub winning_pitcher_name: String,
    pub winning_pitcher_id: String,
    pub losing_pitcher_name: String,
    pub losing_pitcher_id: String,
    pub saving_pitcher_name: String,
    pub saving_pitcher_id: String,
    // 100
    pub gwrbi_player_name: String,
    pub gwrbi_player_id: String,
    pub starter_name: String,
    pub starter_id: String,
    pub opponent_starter_name: String,
    pub opponent_starter_id: String,
    pub lineup_1_id: String,
    pub lineup_1_name: String,
    pub lineup_1_pos: String,
    pub lineup_2_id: String,
    // 110
    pub lineup_2_name: String,
    pub lineup_2_pos: String,
    pub lineup_3_id: String,
    pub lineup_3_name: String,
    pub lineup_3_pos: String,
    pub lineup_4_id: String,
    pub lineup_4_name: String,
    pub lineup_4_pos: String,
    pub lineup_5_id: String,
    pub lineup_5_name: String,
    // 120
    pub lineup_5_pos: String,
    pub lineup_6_id: String,
    pub lineup_6_name: String,
    pub lineup_6_pos: String,
    pub lineup_7_id: String,
    pub lineup_7_name: String,
    pub lineup_7_pos: String,
    pub lineup_8_id: String,
    pub lineup_8_name: String,
    pub lineup_8_pos: String,
    // 130
    pub lineup_9_id: String,
    pub lineup_9_name: String,
    pub lineup_9_pos: String,
    pub opponent_1_id: String,
    pub opponent_1_name: String,
    pub opponent_1_pos: String,
    pub opponent_2_id: String,
    pub opponent_2_name: String,
    pub opponent_2_pos: String,
    pub opponent_3_id: String,
    // 140
    pub opponent_3_name: String,
    pub opponent_3_pos: String,
    pub opponent_4_id: String,
    pub opponent_4_name: String,
    pub opponent_4_pos: String,
    pub opponent_5_id: String,
    pub opponent_5_name: String,
    pub opponent_5_pos: String,
    pub opponent_6_id: String,
    pub opponent_6_name: String,
    // 150
    pub opponent_6_pos: String,
    pub opponent_7_id: String,
    pub opponent_7_name: String,
    pub opponent_7_pos: String,
    pub opponent_8_id: String,
    pub opponent_8_name: String,
    pub opponent_8_pos: String,
    pub opponent_9_id: String,
    pub opponent_9_name: String,
    pub opponent_9_pos: String,
    // 160
    pub additional_info: String,
    pub acquisition_info: String,
}


/// Team game log with all player, manager, and umpire IDs and related fields removed.
#[derive(Clone, Debug)]
pub struct TeamGameLogSmall {
    // This game ID used on many sites.
    pub game_id: String,
    // Other synthetic fields to simplify common queries.
    pub w: bool,
    pub l: bool,
    pub t: bool,
    // 1
    pub date: chrono::NaiveDate,
    // These fields are copied straight from GameLog with some movement.
    pub number_of_game: String,
    pub team: String,
    pub league: String,
    pub team_game_number: u16,
    pub opponent_team: String,
    pub opponent_league: String,
    pub opponent_team_game_number: u16,
    // 10
    pub score: u8,
    pub opponent_score: u8,
    pub number_of_outs: Option<u8>,
    pub day_night: String,
    pub completion_info: String,
    pub forfeit_info: String,
    pub protest_info: String,
    pub park_id: String,
    pub attendance: Option<i32>,
    pub time_of_game: Option<u16>,
    // 20
    pub line_score: Linescore,
    pub opponent_line_score: Linescore,
    pub ab: Option<u8>,
    pub hits: Option<u8>,
    pub doubles: RetrosheetOption,
    pub triples: RetrosheetOption,
    pub homeruns: RetrosheetOption,
    pub rbi: RetrosheetOption,
    pub sac_hits: Option<u8>,
    pub sac_flies: RetrosheetOption,
    // 30
    pub hbp: RetrosheetOption,
    pub walks: RetrosheetOption,
    pub intentional_walks: RetrosheetOption,
    pub strikeouts: RetrosheetOption,
    pub stolen_bases: RetrosheetOption,
    pub caught_stealing: RetrosheetOption,
    pub gidp: RetrosheetOption,
    pub catcher_interference: RetrosheetOption,
    pub left_on_base: RetrosheetOption,
    pub pitchers_used: Option<u8>,
    // 40
    pub individual_earned_runs: RetrosheetOption,
    pub team_earned_runs: Option<u8>,
    pub wild_pitches: RetrosheetOption,
    pub balks: Option<u8>,
    pub putouts: RetrosheetOption,
    pub assists: RetrosheetOption,
    pub errors: RetrosheetOption,
    pub passed_balls: RetrosheetOption,
    pub double_plays: RetrosheetOption,
    pub triple_plays: Option<u8>,
    // 50
    pub opponent_ab: Option<u8>,
    pub opponent_hits: Option<u8>,
    pub opponent_doubles: RetrosheetOption,
    pub opponent_triples: RetrosheetOption,
    pub opponent_homeruns: RetrosheetOption,
    pub opponent_rbi: RetrosheetOption,
    pub opponent_sac_hits: Option<u8>,
    pub opponent_sac_flies: RetrosheetOption,
    pub opponent_hbp: RetrosheetOption,
    pub opponent_walks: RetrosheetOption,
    // 60
    pub opponent_intentional_walks: RetrosheetOption,
    pub opponent_strikeouts: RetrosheetOption,
    pub opponent_stolen_bases: RetrosheetOption,
    pub opponent_caught_stealing: RetrosheetOption,
    pub opponent_gidp: RetrosheetOption,
    pub opponent_catcher_interference: RetrosheetOption,
    pub opponent_left_on_base: RetrosheetOption,
    pub opponent_pitchers_used: Option<u8>,
    pub opponent_individual_earned_runs: RetrosheetOption,
    pub opponent_team_earned_runs: Option<u8>,
    // 70
    pub opponent_wild_pitches: RetrosheetOption,
    pub opponent_balks: Option<u8>,
    pub opponent_putouts: RetrosheetOption,
    pub opponent_assists: RetrosheetOption,
    pub opponent_errors: RetrosheetOption,
    pub opponent_passed_balls: RetrosheetOption,
    pub opponent_double_plays: RetrosheetOption,
    pub opponent_triple_plays: Option<u8>,
}


impl GameLog {
    /// Split the game log entry into separate home and visitor team game logs.
    pub fn each_team_game(&self) -> (TeamGameLog, TeamGameLog) {
        (TeamGameLog::from_home_team(self),
         TeamGameLog::from_visitor_team(self))
    }
}


impl Sql for GameLog {
    fn create_table(tx: &mut Transaction) -> Result<(), Box<dyn Error>> {
        tx.execute("DROP TABLE IF EXISTS games", ())?;
        tx.execute(include_str!("sql/create_games.sql"), ())?;
        Ok(())
    }

    fn table_name<'a>() -> &'a str { "games" }

    fn read_row(row: &Row, offset: usize) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            game_id:                        row.get(offset)?,
            date:                           row.get(offset + 1)?,
            number_of_game:                 row.get(offset + 2)?,
            day_of_week:                    row.get(offset + 3)?,
            visitor_team:                   row.get(offset + 4)?,
            visitor_league:                 row.get(offset + 5)?,
            visitor_team_game_number:       row.get(offset + 6)?,
            home_team:                      row.get(offset + 7)?,
            home_league:                    row.get(offset + 8)?,
            home_team_game_number:          row.get(offset + 9)?,
            visitor_score:                  row.get(offset + 10)?,
            home_score:                     row.get(offset + 11)?,
            number_of_outs:                 row.get(offset + 12)?,
            day_night:                      row.get(offset + 13)?,
            completion_info:                row.get(offset + 14)?,
            forfeit_info:                   row.get(offset + 15)?,
            protest_info:                   row.get(offset + 16)?,
            park_id:                        row.get(offset + 17)?,
            attendance:                     row.get(offset + 18)?,
            time_of_game:                   row.get(offset + 19)?,
            visitor_line_score:             row.get(offset + 20)?,
            home_line_score:                row.get(offset + 21)?,
            visitor_ab:                     row.get(offset + 22)?,
            visitor_hits:                   row.get(offset + 23)?,
            visitor_doubles:                row.get(offset + 24)?,
            visitor_triples:                row.get(offset + 25)?,
            visitor_homeruns:               row.get(offset + 26)?,
            visitor_rbi:                    row.get(offset + 27)?,
            visitor_sac_hits:               row.get(offset + 28)?,
            visitor_sac_flies:              row.get(offset + 29)?,
            visitor_hbp:                    row.get(offset + 30)?,
            visitor_walks:                  row.get(offset + 31)?,
            visitor_intentional_walks:      row.get(offset + 32)?,
            visitor_strikeouts:             row.get(offset + 33)?,
            visitor_stolen_bases:           row.get(offset + 34)?,
            visitor_caught_stealing:        row.get(offset + 35)?,
            visitor_gidp:                   row.get(offset + 36)?,
            visitor_catcher_interference:   row.get(offset + 37)?,
            visitor_left_on_base:           row.get(offset + 38)?,
            visitor_pitchers_used:          row.get(offset + 39)?,
            visitor_individual_earned_runs: row.get(offset + 40)?,
            visitor_team_earned_runs:       row.get(offset + 41)?,
            visitor_wild_pitches:           row.get(offset + 42)?,
            visitor_balks:                  row.get(offset + 43)?,
            visitor_putouts:                row.get(offset + 44)?,
            visitor_assists:                row.get(offset + 45)?,
            visitor_errors:                 row.get(offset + 46)?,
            visitor_passed_balls:           row.get(offset + 47)?,
            visitor_double_plays:           row.get(offset + 48)?,
            visitor_triple_plays:           row.get(offset + 49)?,
            home_ab:                        row.get(offset + 50)?,
            home_hits:                      row.get(offset + 51)?,
            home_doubles:                   row.get(offset + 52)?,
            home_triples:                   row.get(offset + 53)?,
            home_homeruns:                  row.get(offset + 54)?,
            home_rbi:                       row.get(offset + 55)?,
            home_sac_hits:                  row.get(offset + 56)?,
            home_sac_flies:                 row.get(offset + 57)?,
            home_hbp:                       row.get(offset + 58)?,
            home_walks:                     row.get(offset + 59)?,
            home_intentional_walks:         row.get(offset + 60)?,
            home_strikeouts:                row.get(offset + 61)?,
            home_stolen_bases:              row.get(offset + 62)?,
            home_caught_stealing:           row.get(offset + 63)?,
            home_gidp:                      row.get(offset + 64)?,
            home_catcher_interference:      row.get(offset + 65)?,
            home_left_on_base:              row.get(offset + 66)?,
            home_pitchers_used:             row.get(offset + 67)?,
            home_individual_earned_runs:    row.get(offset + 68)?,
            home_team_earned_runs:          row.get(offset + 69)?,
            home_wild_pitches:              row.get(offset + 70)?,
            home_balks:                     row.get(offset + 71)?,
            home_putouts:                   row.get(offset + 72)?,
            home_assists:                   row.get(offset + 73)?,
            home_errors:                    row.get(offset + 74)?,
            home_passed_balls:              row.get(offset + 75)?,
            home_double_plays:              row.get(offset + 76)?,
            home_triple_plays:              row.get(offset + 77)?,
            home_plate_umpire_name:         row.get(offset + 78)?,
            home_plate_umpire_id:           row.get(offset + 79)?,
            first_base_umpire_name:         row.get(offset + 80)?,
            first_base_umpire_id:           row.get(offset + 81)?,
            second_base_umpire_name:        row.get(offset + 82)?,
            second_base_umpire_id:          row.get(offset + 83)?,
            third_base_umpire_name:         row.get(offset + 84)?,
            third_base_umpire_id:           row.get(offset + 85)?,
            left_field_umpire_name:         row.get(offset + 86)?,
            left_field_umpire_id:           row.get(offset + 87)?,
            right_field_umpire_name:        row.get(offset + 88)?,
            right_field_umpire_id:          row.get(offset + 89)?,
            visitor_manager_id:             row.get(offset + 90)?,
            visitor_manager_name:           row.get(offset + 91)?,
            home_manager_id:                row.get(offset + 92)?,
            home_manager_name:              row.get(offset + 93)?,
            winning_pitcher_name:           row.get(offset + 94)?,
            winning_pitcher_id:             row.get(offset + 95)?,
            losing_pitcher_name:            row.get(offset + 96)?,
            losing_pitcher_id:              row.get(offset + 97)?,
            saving_pitcher_name:            row.get(offset + 98)?,
            saving_pitcher_id:              row.get(offset + 99)?,
            gwrbi_player_name:              row.get(offset + 100)?,
            gwrbi_player_id:                row.get(offset + 101)?,
            visitor_starter_name:           row.get(offset + 102)?,
            visitor_starter_id:             row.get(offset + 103)?,
            home_starter_name:              row.get(offset + 104)?,
            home_starter_id:                row.get(offset + 105)?,
            visitor_1_id:                   row.get(offset + 106)?,
            visitor_1_name:                 row.get(offset + 107)?,
            visitor_1_pos:                  row.get(offset + 108)?,
            visitor_2_id:                   row.get(offset + 109)?,
            visitor_2_name:                 row.get(offset + 110)?,
            visitor_2_pos:                  row.get(offset + 111)?,
            visitor_3_id:                   row.get(offset + 112)?,
            visitor_3_name:                 row.get(offset + 113)?,
            visitor_3_pos:                  row.get(offset + 114)?,
            visitor_4_id:                   row.get(offset + 115)?,
            visitor_4_name:                 row.get(offset + 116)?,
            visitor_4_pos:                  row.get(offset + 117)?,
            visitor_5_id:                   row.get(offset + 118)?,
            visitor_5_name:                 row.get(offset + 119)?,
            visitor_5_pos:                  row.get(offset + 120)?,
            visitor_6_id:                   row.get(offset + 121)?,
            visitor_6_name:                 row.get(offset + 122)?,
            visitor_6_pos:                  row.get(offset + 123)?,
            visitor_7_id:                   row.get(offset + 124)?,
            visitor_7_name:                 row.get(offset + 125)?,
            visitor_7_pos:                  row.get(offset + 126)?,
            visitor_8_id:                   row.get(offset + 127)?,
            visitor_8_name:                 row.get(offset + 128)?,
            visitor_8_pos:                  row.get(offset + 129)?,
            visitor_9_id:                   row.get(offset + 130)?,
            visitor_9_name:                 row.get(offset + 131)?,
            visitor_9_pos:                  row.get(offset + 132)?,
            home_1_id:                      row.get(offset + 133)?,
            home_1_name:                    row.get(offset + 134)?,
            home_1_pos:                     row.get(offset + 135)?,
            home_2_id:                      row.get(offset + 136)?,
            home_2_name:                    row.get(offset + 137)?,
            home_2_pos:                     row.get(offset + 138)?,
            home_3_id:                      row.get(offset + 139)?,
            home_3_name:                    row.get(offset + 140)?,
            home_3_pos:                     row.get(offset + 141)?,
            home_4_id:                      row.get(offset + 142)?,
            home_4_name:                    row.get(offset + 143)?,
            home_4_pos:                     row.get(offset + 144)?,
            home_5_id:                      row.get(offset + 145)?,
            home_5_name:                    row.get(offset + 146)?,
            home_5_pos:                     row.get(offset + 147)?,
            home_6_id:                      row.get(offset + 148)?,
            home_6_name:                    row.get(offset + 149)?,
            home_6_pos:                     row.get(offset + 150)?,
            home_7_id:                      row.get(offset + 151)?,
            home_7_name:                    row.get(offset + 152)?,
            home_7_pos:                     row.get(offset + 153)?,
            home_8_id:                      row.get(offset + 154)?,
            home_8_name:                    row.get(offset + 155)?,
            home_8_pos:                     row.get(offset + 156)?,
            home_9_id:                      row.get(offset + 157)?,
            home_9_name:                    row.get(offset + 158)?,
            home_9_pos:                     row.get(offset + 159)?,
            additional_info:                row.get(offset + 160)?,
            acquisition_info:               row.get(offset + 161)?,
        })
    }

    fn write_row(&self, statement: &mut Statement) -> Result<usize, rusqlite::Error> {
        statement.execute(
            named_params! {
                ":game_id": &self.game_id,
                ":date": &self.date,
                ":number_of_game": &self.number_of_game,
                ":day_of_week": &self.day_of_week,
                ":visitor_team": &self.visitor_team,
                ":visitor_league": &self.visitor_league,
                ":visitor_team_game_number": &self.visitor_team_game_number,
                ":home_team": &self.home_team,
                ":home_league": &self.home_league,
                ":home_team_game_number": &self.home_team_game_number,
                ":visitor_score": &self.visitor_score,
                ":home_score": &self.home_score,
                ":number_of_outs": &self.number_of_outs,
                ":day_night": &self.day_night,
                ":completion_info": &self.completion_info,
                ":forfeit_info": &self.forfeit_info,
                ":protest_info": &self.protest_info,
                ":park_id": &self.park_id,
                ":attendance": &self.attendance,
                ":time_of_game": &self.time_of_game,
                ":visitor_line_score": &self.visitor_line_score,
                ":home_line_score": &self.home_line_score,
                ":visitor_ab": &self.visitor_ab,
                ":visitor_hits": &self.visitor_hits,
                ":visitor_doubles": &self.visitor_doubles,
                ":visitor_triples": &self.visitor_triples,
                ":visitor_homeruns": &self.visitor_homeruns,
                ":visitor_rbi": &self.visitor_rbi,
                ":visitor_sac_hits": &self.visitor_sac_hits,
                ":visitor_sac_flies": &self.visitor_sac_flies,
                ":visitor_hbp": &self.visitor_hbp,
                ":visitor_walks": &self.visitor_walks,
                ":visitor_intentional_walks": &self.visitor_intentional_walks,
                ":visitor_strikeouts": &self.visitor_strikeouts,
                ":visitor_stolen_bases": &self.visitor_stolen_bases,
                ":visitor_caught_stealing": &self.visitor_caught_stealing,
                ":visitor_gidp": &self.visitor_gidp,
                ":visitor_catcher_interference": &self.visitor_catcher_interference,
                ":visitor_left_on_base": &self.visitor_left_on_base,
                ":visitor_pitchers_used": &self.visitor_pitchers_used,
                ":visitor_individual_earned_runs": &self.visitor_individual_earned_runs,
                ":visitor_team_earned_runs": &self.visitor_team_earned_runs,
                ":visitor_wild_pitches": &self.visitor_wild_pitches,
                ":visitor_balks": &self.visitor_balks,
                ":visitor_putouts": &self.visitor_putouts,
                ":visitor_assists": &self.visitor_assists,
                ":visitor_errors": &self.visitor_errors,
                ":visitor_passed_balls": &self.visitor_passed_balls,
                ":visitor_double_plays": &self.visitor_double_plays,
                ":visitor_triple_plays": &self.visitor_triple_plays,
                ":home_ab": &self.home_ab,
                ":home_hits": &self.home_hits,
                ":home_doubles": &self.home_doubles,
                ":home_triples": &self.home_triples,
                ":home_homeruns": &self.home_homeruns,
                ":home_rbi": &self.home_rbi,
                ":home_sac_hits": &self.home_sac_hits,
                ":home_sac_flies": &self.home_sac_flies,
                ":home_hbp": &self.home_hbp,
                ":home_walks": &self.home_walks,
                ":home_intentional_walks": &self.home_intentional_walks,
                ":home_strikeouts": &self.home_strikeouts,
                ":home_stolen_bases": &self.home_stolen_bases,
                ":home_caught_stealing": &self.home_caught_stealing,
                ":home_gidp": &self.home_gidp,
                ":home_catcher_interference": &self.home_catcher_interference,
                ":home_left_on_base": &self.home_left_on_base,
                ":home_pitchers_used": &self.home_pitchers_used,
                ":home_individual_earned_runs": &self.home_individual_earned_runs,
                ":home_team_earned_runs": &self.home_team_earned_runs,
                ":home_wild_pitches": &self.home_wild_pitches,
                ":home_balks": &self.home_balks,
                ":home_putouts": &self.home_putouts,
                ":home_assists": &self.home_assists,
                ":home_errors": &self.home_errors,
                ":home_passed_balls": &self.home_passed_balls,
                ":home_double_plays": &self.home_double_plays,
                ":home_triple_plays": &self.home_triple_plays,
                ":home_plate_umpire_name": &self.home_plate_umpire_name,
                ":home_plate_umpire_id": &self.home_plate_umpire_id,
                ":first_base_umpire_name": &self.first_base_umpire_name,
                ":first_base_umpire_id": &self.first_base_umpire_id,
                ":second_base_umpire_name": &self.second_base_umpire_name,
                ":second_base_umpire_id": &self.second_base_umpire_id,
                ":third_base_umpire_name": &self.third_base_umpire_name,
                ":third_base_umpire_id": &self.third_base_umpire_id,
                ":left_field_umpire_name": &self.left_field_umpire_name,
                ":left_field_umpire_id": &self.left_field_umpire_id,
                ":right_field_umpire_name": &self.right_field_umpire_name,
                ":right_field_umpire_id": &self.right_field_umpire_id,
                ":visitor_manager_id": &self.visitor_manager_id,
                ":visitor_manager_name": &self.visitor_manager_name,
                ":home_manager_id": &self.home_manager_id,
                ":home_manager_name": &self.home_manager_name,
                ":winning_pitcher_name": &self.winning_pitcher_name,
                ":winning_pitcher_id": &self.winning_pitcher_id,
                ":losing_pitcher_name": &self.losing_pitcher_name,
                ":losing_pitcher_id": &self.losing_pitcher_id,
                ":saving_pitcher_name": &self.saving_pitcher_name,
                ":saving_pitcher_id": &self.saving_pitcher_id,
                ":gwrbi_player_name": &self.gwrbi_player_name,
                ":gwrbi_player_id": &self.gwrbi_player_id,
                ":visitor_starter_name": &self.visitor_starter_name,
                ":visitor_starter_id": &self.visitor_starter_id,
                ":home_starter_name": &self.home_starter_name,
                ":home_starter_id": &self.home_starter_id,
                ":visitor_1_id": &self.visitor_1_id,
                ":visitor_1_name": &self.visitor_1_name,
                ":visitor_1_pos": &self.visitor_1_pos,
                ":visitor_2_id": &self.visitor_2_id,
                ":visitor_2_name": &self.visitor_2_name,
                ":visitor_2_pos": &self.visitor_2_pos,
                ":visitor_3_id": &self.visitor_3_id,
                ":visitor_3_name": &self.visitor_3_name,
                ":visitor_3_pos": &self.visitor_3_pos,
                ":visitor_4_id": &self.visitor_4_id,
                ":visitor_4_name": &self.visitor_4_name,
                ":visitor_4_pos": &self.visitor_4_pos,
                ":visitor_5_id": &self.visitor_5_id,
                ":visitor_5_name": &self.visitor_5_name,
                ":visitor_5_pos": &self.visitor_5_pos,
                ":visitor_6_id": &self.visitor_6_id,
                ":visitor_6_name": &self.visitor_6_name,
                ":visitor_6_pos": &self.visitor_6_pos,
                ":visitor_7_id": &self.visitor_7_id,
                ":visitor_7_name": &self.visitor_7_name,
                ":visitor_7_pos": &self.visitor_7_pos,
                ":visitor_8_id": &self.visitor_8_id,
                ":visitor_8_name": &self.visitor_8_name,
                ":visitor_8_pos": &self.visitor_8_pos,
                ":visitor_9_id": &self.visitor_9_id,
                ":visitor_9_name": &self.visitor_9_name,
                ":visitor_9_pos": &self.visitor_9_pos,
                ":home_1_id": &self.home_1_id,
                ":home_1_name": &self.home_1_name,
                ":home_1_pos": &self.home_1_pos,
                ":home_2_id": &self.home_2_id,
                ":home_2_name": &self.home_2_name,
                ":home_2_pos": &self.home_2_pos,
                ":home_3_id": &self.home_3_id,
                ":home_3_name": &self.home_3_name,
                ":home_3_pos": &self.home_3_pos,
                ":home_4_id": &self.home_4_id,
                ":home_4_name": &self.home_4_name,
                ":home_4_pos": &self.home_4_pos,
                ":home_5_id": &self.home_5_id,
                ":home_5_name": &self.home_5_name,
                ":home_5_pos": &self.home_5_pos,
                ":home_6_id": &self.home_6_id,
                ":home_6_name": &self.home_6_name,
                ":home_6_pos": &self.home_6_pos,
                ":home_7_id": &self.home_7_id,
                ":home_7_name": &self.home_7_name,
                ":home_7_pos": &self.home_7_pos,
                ":home_8_id": &self.home_8_id,
                ":home_8_name": &self.home_8_name,
                ":home_8_pos": &self.home_8_pos,
                ":home_9_id": &self.home_9_id,
                ":home_9_name": &self.home_9_name,
                ":home_9_pos": &self.home_9_pos,
                ":additional_info": &self.additional_info,
                ":acquisition_info": &self.acquisition_info,
            }
        )
    }

    fn column_names<'a>() -> Vec<&'a str> {
        vec![
            "game_id",
            "date",
            "number_of_game",
            "visitor_team",
            "visitor_league",
            "visitor_team_game_number",
            "home_team",
            "home_league",
            "home_team_game_number",
            "visitor_score",
            "home_score",
            "number_of_outs",
            "day_night",
            "completion_info",
            "forfeit_info",
            "protest_info",
            "park_id",
            "attendance",
            "time_of_game",
            "visitor_line_score",
            "home_line_score",
            "visitor_ab",
            "visitor_hits",
            "visitor_doubles",
            "visitor_triples",
            "visitor_homeruns",
            "visitor_rbi",
            "visitor_sac_hits",
            "visitor_sac_flies",
            "visitor_hbp",
            "visitor_walks",
            "visitor_intentional_walks",
            "visitor_strikeouts",
            "visitor_stolen_bases",
            "visitor_caught_stealing",
            "visitor_gidp",
            "visitor_catcher_interference",
            "visitor_left_on_base",
            "visitor_pitchers_used",
            "visitor_individual_earned_runs",
            "visitor_team_earned_runs",
            "visitor_wild_pitches",
            "visitor_balks",
            "visitor_putouts",
            "visitor_assists",
            "visitor_errors",
            "visitor_passed_balls",
            "visitor_double_plays",
            "visitor_triple_plays",
            "home_ab",
            "home_hits",
            "home_doubles",
            "home_triples",
            "home_homeruns",
            "home_rbi",
            "home_sac_hits",
            "home_sac_flies",
            "home_hbp",
            "home_walks",
            "home_intentional_walks",
            "home_strikeouts",
            "home_stolen_bases",
            "home_caught_stealing",
            "home_gidp",
            "home_catcher_interference",
            "home_left_on_base",
            "home_pitchers_used",
            "home_individual_earned_runs",
            "home_team_earned_runs",
            "home_wild_pitches",
            "home_balks",
            "home_putouts",
            "home_assists",
            "home_errors",
            "home_passed_balls",
            "home_double_plays",
            "home_triple_plays",
        ]
    }
}


impl GameLogSmall {
    /// Split the game log entry into separate home and visitor team game logs.
    pub fn each_team_game(&self) -> (TeamGameLogSmall, TeamGameLogSmall) {
        (TeamGameLogSmall::from_home_team(self),
         TeamGameLogSmall::from_visitor_team(self))
    }
}


impl Sql for GameLogSmall {
    // Should never be used.
    fn create_table(_tx: &mut Transaction) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn table_name<'a>() -> &'a str { "games" }

    fn read_row(row: &Row, offset: usize) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            game_id:                        row.get(offset)?,
            date:                           row.get(offset + 1)?,
            number_of_game:                 row.get(offset + 2)?,
            visitor_team:                   row.get(offset + 3)?,
            visitor_league:                 row.get(offset + 4)?,
            visitor_team_game_number:       row.get(offset + 5)?,
            home_team:                      row.get(offset + 6)?,
            home_league:                    row.get(offset + 7)?,
            home_team_game_number:          row.get(offset + 8)?,
            visitor_score:                  row.get(offset + 9)?,
            home_score:                     row.get(offset + 10)?,
            number_of_outs:                 row.get(offset + 11)?,
            day_night:                      row.get(offset + 12)?,
            completion_info:                row.get(offset + 13)?,
            forfeit_info:                   row.get(offset + 14)?,
            protest_info:                   row.get(offset + 15)?,
            park_id:                        row.get(offset + 16)?,
            attendance:                     row.get(offset + 17)?,
            time_of_game:                   row.get(offset + 18)?,
            visitor_line_score:             row.get(offset + 19)?,
            home_line_score:                row.get(offset + 20)?,
            visitor_ab:                     row.get(offset + 21)?,
            visitor_hits:                   row.get(offset + 22)?,
            visitor_doubles:                row.get(offset + 23)?,
            visitor_triples:                row.get(offset + 24)?,
            visitor_homeruns:               row.get(offset + 25)?,
            visitor_rbi:                    row.get(offset + 26)?,
            visitor_sac_hits:               row.get(offset + 27)?,
            visitor_sac_flies:              row.get(offset + 28)?,
            visitor_hbp:                    row.get(offset + 29)?,
            visitor_walks:                  row.get(offset + 30)?,
            visitor_intentional_walks:      row.get(offset + 31)?,
            visitor_strikeouts:             row.get(offset + 32)?,
            visitor_stolen_bases:           row.get(offset + 33)?,
            visitor_caught_stealing:        row.get(offset + 34)?,
            visitor_gidp:                   row.get(offset + 35)?,
            visitor_catcher_interference:   row.get(offset + 36)?,
            visitor_left_on_base:           row.get(offset + 37)?,
            visitor_pitchers_used:          row.get(offset + 38)?,
            visitor_individual_earned_runs: row.get(offset + 39)?,
            visitor_team_earned_runs:       row.get(offset + 40)?,
            visitor_wild_pitches:           row.get(offset + 41)?,
            visitor_balks:                  row.get(offset + 42)?,
            visitor_putouts:                row.get(offset + 43)?,
            visitor_assists:                row.get(offset + 44)?,
            visitor_errors:                 row.get(offset + 45)?,
            visitor_passed_balls:           row.get(offset + 46)?,
            visitor_double_plays:           row.get(offset + 47)?,
            visitor_triple_plays:           row.get(offset + 48)?,
            home_ab:                        row.get(offset + 49)?,
            home_hits:                      row.get(offset + 50)?,
            home_doubles:                   row.get(offset + 51)?,
            home_triples:                   row.get(offset + 52)?,
            home_homeruns:                  row.get(offset + 53)?,
            home_rbi:                       row.get(offset + 54)?,
            home_sac_hits:                  row.get(offset + 55)?,
            home_sac_flies:                 row.get(offset + 56)?,
            home_hbp:                       row.get(offset + 57)?,
            home_walks:                     row.get(offset + 58)?,
            home_intentional_walks:         row.get(offset + 59)?,
            home_strikeouts:                row.get(offset + 60)?,
            home_stolen_bases:              row.get(offset + 61)?,
            home_caught_stealing:           row.get(offset + 62)?,
            home_gidp:                      row.get(offset + 63)?,
            home_catcher_interference:      row.get(offset + 64)?,
            home_left_on_base:              row.get(offset + 65)?,
            home_pitchers_used:             row.get(offset + 66)?,
            home_individual_earned_runs:    row.get(offset + 67)?,
            home_team_earned_runs:          row.get(offset + 68)?,
            home_wild_pitches:              row.get(offset + 69)?,
            home_balks:                     row.get(offset + 70)?,
            home_putouts:                   row.get(offset + 71)?,
            home_assists:                   row.get(offset + 72)?,
            home_errors:                    row.get(offset + 73)?,
            home_passed_balls:              row.get(offset + 74)?,
            home_double_plays:              row.get(offset + 75)?,
            home_triple_plays:              row.get(offset + 76)?,
        })
    }

    // Should never be used.
    fn write_row(&self, _statement: &mut Statement) -> Result<usize, rusqlite::Error> {
        Ok(0)
    }

    fn column_names<'a>() -> Vec<&'a str> {
        vec![
            "game_id",
            "date",
            "number_of_game",
            "visitor_team",
            "visitor_league",
            "visitor_team_game_number",
            "home_team",
            "home_league",
            "home_team_game_number",
            "visitor_score",
            "home_score",
            "number_of_outs",
            "day_night",
            "completion_info",
            "forfeit_info",
            "protest_info",
            "park_id",
            "attendance",
            "time_of_game",
            "visitor_line_score",
            "home_line_score",
            "visitor_ab",
            "visitor_hits",
            "visitor_doubles",
            "visitor_triples",
            "visitor_homeruns",
            "visitor_rbi",
            "visitor_sac_hits",
            "visitor_sac_flies",
            "visitor_hbp",
            "visitor_walks",
            "visitor_intentional_walks",
            "visitor_strikeouts",
            "visitor_stolen_bases",
            "visitor_caught_stealing",
            "visitor_gidp",
            "visitor_catcher_interference",
            "visitor_left_on_base",
            "visitor_pitchers_used",
            "visitor_individual_earned_runs",
            "visitor_team_earned_runs",
            "visitor_wild_pitches",
            "visitor_balks",
            "visitor_putouts",
            "visitor_assists",
            "visitor_errors",
            "visitor_passed_balls",
            "visitor_double_plays",
            "visitor_triple_plays",
            "home_ab",
            "home_hits",
            "home_doubles",
            "home_triples",
            "home_homeruns",
            "home_rbi",
            "home_sac_hits",
            "home_sac_flies",
            "home_hbp",
            "home_walks",
            "home_intentional_walks",
            "home_strikeouts",
            "home_stolen_bases",
            "home_caught_stealing",
            "home_gidp",
            "home_catcher_interference",
            "home_left_on_base",
            "home_pitchers_used",
            "home_individual_earned_runs",
            "home_team_earned_runs",
            "home_wild_pitches",
            "home_balks",
            "home_putouts",
            "home_assists",
            "home_errors",
            "home_passed_balls",
            "home_double_plays",
            "home_triple_plays",
        ]
    }
}


impl From<game::GameLog> for GameLog {
    fn from(game_log: game::GameLog) -> Self {
        let date = chrono::NaiveDate::parse_from_str(&game_log.date, "%Y%m%d");
        GameLog {
            game_id: game_log.game_id(),
            date: date.expect("Couldn't parse Retrosheet date"),
            number_of_game: game_log.number_of_game,
            day_of_week: game_log.day_of_week,
            visitor_team: game_log.visitor_team,
            visitor_league: game_log.visitor_league,
            visitor_team_game_number: game_log.visitor_team_game_number,
            home_team: game_log.home_team,
            home_league: game_log.home_league,
            home_team_game_number: game_log.home_team_game_number,
            visitor_score: game_log.visitor_score,
            home_score: game_log.home_score,
            number_of_outs: game_log.number_of_outs,
            day_night: game_log.day_night,
            completion_info: game_log.completion_info,
            forfeit_info: game_log.forfeit_info,
            protest_info: game_log.protest_info,
            park_id: game_log.park_id,
            attendance: game_log.attendance,
            time_of_game: game_log.time_of_game,
            visitor_line_score: game_log.visitor_line_score.as_str().into(),
            home_line_score: game_log.home_line_score.as_str().into(),
            visitor_ab: game_log.visitor_ab,
            visitor_hits: game_log.visitor_hits,
            visitor_doubles: game_log.visitor_doubles.into(),
            visitor_triples: game_log.visitor_triples.into(),
            visitor_homeruns: game_log.visitor_homeruns.into(),
            visitor_rbi: game_log.visitor_rbi.into(),
            visitor_sac_hits: game_log.visitor_sac_hits,
            visitor_sac_flies: game_log.visitor_sac_flies.into(),
            visitor_hbp: game_log.visitor_hbp.into(),
            visitor_walks: game_log.visitor_walks.into(),
            visitor_intentional_walks: game_log.visitor_intentional_walks.into(),
            visitor_strikeouts: game_log.visitor_strikeouts.into(),
            visitor_stolen_bases: game_log.visitor_stolen_bases.into(),
            visitor_caught_stealing: game_log.visitor_caught_stealing.into(),
            visitor_gidp: game_log.visitor_gidp.into(),
            visitor_catcher_interference: game_log.visitor_catcher_interference.into(),
            visitor_left_on_base: game_log.visitor_left_on_base.into(),
            visitor_pitchers_used: game_log.visitor_pitchers_used,
            visitor_individual_earned_runs: game_log.visitor_individual_earned_runs.into(),
            visitor_team_earned_runs: game_log.visitor_team_earned_runs,
            visitor_wild_pitches: game_log.visitor_wild_pitches.into(),
            visitor_balks: game_log.visitor_balks,
            visitor_putouts: game_log.visitor_putouts.into(),
            visitor_assists: game_log.visitor_assists.into(),
            visitor_errors: game_log.visitor_errors.into(),
            visitor_passed_balls: game_log.visitor_passed_balls.into(),
            visitor_double_plays: game_log.visitor_double_plays.into(),
            visitor_triple_plays: game_log.visitor_triple_plays,
            home_ab: game_log.home_ab,
            home_hits: game_log.home_hits,
            home_doubles: game_log.home_doubles.into(),
            home_triples: game_log.home_triples.into(),
            home_homeruns: game_log.home_homeruns.into(),
            home_rbi: game_log.home_rbi.into(),
            home_sac_hits: game_log.home_sac_hits,
            home_sac_flies: game_log.home_sac_flies.into(),
            home_hbp: game_log.home_hbp.into(),
            home_walks: game_log.home_walks.into(),
            home_intentional_walks: game_log.home_intentional_walks.into(),
            home_strikeouts: game_log.home_strikeouts.into(),
            home_stolen_bases: game_log.home_stolen_bases.into(),
            home_caught_stealing: game_log.home_caught_stealing.into(),
            home_gidp: game_log.home_gidp.into(),
            home_catcher_interference: game_log.home_catcher_interference.into(),
            home_left_on_base: game_log.home_left_on_base.into(),
            home_pitchers_used: game_log.home_pitchers_used,
            home_individual_earned_runs: game_log.home_individual_earned_runs.into(),
            home_team_earned_runs: game_log.home_team_earned_runs,
            home_wild_pitches: game_log.home_wild_pitches.into(),
            home_balks: game_log.home_balks,
            home_putouts: game_log.home_putouts.into(),
            home_assists: game_log.home_assists.into(),
            home_errors: game_log.home_errors.into(),
            home_passed_balls: game_log.home_passed_balls.into(),
            home_double_plays: game_log.home_double_plays.into(),
            home_triple_plays: game_log.home_triple_plays,
            home_plate_umpire_name: game_log.home_plate_umpire_name,
            home_plate_umpire_id: game_log.home_plate_umpire_id,
            first_base_umpire_name: game_log.first_base_umpire_name,
            first_base_umpire_id: game_log.first_base_umpire_id,
            second_base_umpire_name: game_log.second_base_umpire_name,
            second_base_umpire_id: game_log.second_base_umpire_id,
            third_base_umpire_name: game_log.third_base_umpire_name,
            third_base_umpire_id: game_log.third_base_umpire_id,
            left_field_umpire_name: game_log.left_field_umpire_name,
            left_field_umpire_id: game_log.left_field_umpire_id,
            right_field_umpire_name: game_log.right_field_umpire_name,
            right_field_umpire_id: game_log.right_field_umpire_id,
            visitor_manager_id: game_log.visitor_manager_id,
            visitor_manager_name: game_log.visitor_manager_name,
            home_manager_id: game_log.home_manager_id,
            home_manager_name: game_log.home_manager_name,
            winning_pitcher_name: game_log.winning_pitcher_name,
            winning_pitcher_id: game_log.winning_pitcher_id,
            losing_pitcher_name: game_log.losing_pitcher_name,
            losing_pitcher_id: game_log.losing_pitcher_id,
            saving_pitcher_name: game_log.saving_pitcher_name,
            saving_pitcher_id: game_log.saving_pitcher_id,
            gwrbi_player_name: game_log.gwrbi_player_name,
            gwrbi_player_id: game_log.gwrbi_player_id,
            visitor_starter_name: game_log.visitor_starter_name,
            visitor_starter_id: game_log.visitor_starter_id,
            home_starter_name: game_log.home_starter_name,
            home_starter_id: game_log.home_starter_id,
            visitor_1_id: game_log.visitor_1_id,
            visitor_1_name: game_log.visitor_1_name,
            visitor_1_pos: game_log.visitor_1_pos,
            visitor_2_id: game_log.visitor_2_id,
            visitor_2_name: game_log.visitor_2_name,
            visitor_2_pos: game_log.visitor_2_pos,
            visitor_3_id: game_log.visitor_3_id,
            visitor_3_name: game_log.visitor_3_name,
            visitor_3_pos: game_log.visitor_3_pos,
            visitor_4_id: game_log.visitor_4_id,
            visitor_4_name: game_log.visitor_4_name,
            visitor_4_pos: game_log.visitor_4_pos,
            visitor_5_id: game_log.visitor_5_id,
            visitor_5_name: game_log.visitor_5_name,
            visitor_5_pos: game_log.visitor_5_pos,
            visitor_6_id: game_log.visitor_6_id,
            visitor_6_name: game_log.visitor_6_name,
            visitor_6_pos: game_log.visitor_6_pos,
            visitor_7_id: game_log.visitor_7_id,
            visitor_7_name: game_log.visitor_7_name,
            visitor_7_pos: game_log.visitor_7_pos,
            visitor_8_id: game_log.visitor_8_id,
            visitor_8_name: game_log.visitor_8_name,
            visitor_8_pos: game_log.visitor_8_pos,
            visitor_9_id: game_log.visitor_9_id,
            visitor_9_name: game_log.visitor_9_name,
            visitor_9_pos: game_log.visitor_9_pos,
            home_1_id: game_log.home_1_id,
            home_1_name: game_log.home_1_name,
            home_1_pos: game_log.home_1_pos,
            home_2_id: game_log.home_2_id,
            home_2_name: game_log.home_2_name,
            home_2_pos: game_log.home_2_pos,
            home_3_id: game_log.home_3_id,
            home_3_name: game_log.home_3_name,
            home_3_pos: game_log.home_3_pos,
            home_4_id: game_log.home_4_id,
            home_4_name: game_log.home_4_name,
            home_4_pos: game_log.home_4_pos,
            home_5_id: game_log.home_5_id,
            home_5_name: game_log.home_5_name,
            home_5_pos: game_log.home_5_pos,
            home_6_id: game_log.home_6_id,
            home_6_name: game_log.home_6_name,
            home_6_pos: game_log.home_6_pos,
            home_7_id: game_log.home_7_id,
            home_7_name: game_log.home_7_name,
            home_7_pos: game_log.home_7_pos,
            home_8_id: game_log.home_8_id,
            home_8_name: game_log.home_8_name,
            home_8_pos: game_log.home_8_pos,
            home_9_id: game_log.home_9_id,
            home_9_name: game_log.home_9_name,
            home_9_pos: game_log.home_9_pos,
            additional_info: game_log.additional_info,
            acquisition_info: game_log.acquisition_info,
        }
    }
}


impl From<Option<i8>> for RetrosheetOption {
    fn from(option: Option<i8>) -> Self {
        match option {
            Some(v) if v >= 0 => {
                RetrosheetOption::Some(u8::try_from(v).unwrap())
            }
            Some(_v) => RetrosheetOption::Unknown,
            None => RetrosheetOption::Unknown,
        }
    }
}


impl From<RetrosheetOption> for Value {
    fn from(retro_option: RetrosheetOption) -> Value {
        match retro_option {
            RetrosheetOption::None => Value::Null,
            RetrosheetOption::Unknown => Value::Int(-1),
            RetrosheetOption::Some(i) => Value::Int(i.into()),
        }
    }
}


impl FromSql for RetrosheetOption {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        let result = match value {
            ValueRef::Integer(v) => {
                if v >= 0 {
                    RetrosheetOption::Some(v as u8)
                }
                else {
                    RetrosheetOption::None
                }
            }
            ValueRef::Null => RetrosheetOption::Unknown,
            _ => RetrosheetOption::Unknown,
        };
        Ok(result)
    }
}


impl ToSql for RetrosheetOption {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let value = match self {
            RetrosheetOption::None => ToSqlOutput::from(-1),
            RetrosheetOption::Unknown => ToSqlOutput::from(Null),
            RetrosheetOption::Some(count) => ToSqlOutput::from(*count),
        };
        Ok(value)
    }
}


impl Linescore {
    /// Split the Retrosheet linescore into a Vec so every inning is a separate element.
    pub fn split_linescore(retrosheet_linescore: &str) -> Self {
        let mut linescore = Vec::new();
        let mut in_inning = false;
        let mut score = 0;
        for ch in retrosheet_linescore.chars() {
            match ch {
                '0'..='9' => {
                    if in_inning {
                        score *= 10;
                    }
                    score += ch.to_digit(10).expect("Failed to convert 0-9 to u32");
                    if !in_inning {
                        linescore.push(Some(u8::try_from(score).expect("Failed to convert u32 to u8")));
                        score = 0;
                    }
                }
                '(' => {
                    in_inning = true;
                }
                ')' => {
                    in_inning = false;
                    linescore.push(Some(u8::try_from(score).expect("Failed to convert u32 to u8")));
                    score = 0;
                }
                'x' => {
                    linescore.push(None);
                }
                ch => {
                    unreachable!("Unknown character: {}", ch)
                }
            }
        }
        Self {
            linescore
        }
    }

    /// Convert the split linescore back into the Retrosheet format.
    pub fn retrosheet_linescore(&self) -> String {
        let mut joined_score = String::with_capacity(self.linescore.len());
        for inning in &self.linescore {
            match inning {
                Some(score) if *score < 10 => {
                    joined_score.push_str(&score.to_string());
                }
                Some(score) => {
                    joined_score.push('(');
                    joined_score.push_str(&score.to_string());
                    joined_score.push(')');
                }
                None => {
                    joined_score.push('x');
                }
            }
        }
        joined_score
    }

    /// Convert the split linescore into a format easier to split up for the database.
    pub fn sql_linescore(&self) -> String {
        self.linescore.iter()
            .map(|score| {
                match score {
                    Some(score) => score.to_string(),
                    None => "x".to_string(),
                }
            })
            .reduce(|mut linescore, score| {
                linescore.push(',');
                linescore.push_str(&score);
                linescore
            })
            .unwrap_or(String::from(""))
    }

    /// Convert a comma-delimited representation into itself.
    fn from_sql_linescore(sql_text: &str) -> Self {
        let mut linescore = Vec::new();
        for inning in sql_text.split(',') {
            match inning {
                "x" => linescore.push(None),
                _ => {
                    let score = inning.parse::<u8>().ok();
                    linescore.push(score)
                }
            }
        }
        Self {
            linescore
        }
    }

    fn as_vec(&self) -> &Vec<Option<u8>> { &self.linescore }
}


impl From<&str> for Linescore {
    fn from(retrosheet_linescore: &str) -> Self {
        Linescore::split_linescore(retrosheet_linescore)
    }
}


impl FromSql for Linescore {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        let result = match value {
            ValueRef::Text(text) => Linescore::from_sql_linescore(str::from_utf8(text).unwrap_or("")),
            _ => Linescore::from_sql_linescore(""),
        };
        Ok(result)
    }
}


impl ToSql for Linescore {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let value = self.sql_linescore();
        Ok(ToSqlOutput::from(value))
    }
}


impl TeamGameLog {
    fn from_home_team(game: &GameLog) -> TeamGameLog {
        TeamGameLog {
            game_id: game.game_id.clone(),
            w: game.home_score > game.visitor_score,
            l: game.home_score < game.visitor_score,
            t: game.home_score == game.visitor_score,
            // 1
            date: game.date,
            number_of_game: game.number_of_game.clone(),
            day_of_week: game.day_of_week.clone(),
            team: game.home_team.clone(),
            league: game.home_league.clone(),
            team_game_number: game.home_team_game_number,
            opponent_team: game.visitor_team.clone(),
            opponent_league: game.visitor_league.clone(),
            opponent_team_game_number: game.visitor_team_game_number,
            // 10
            score: game.home_score,
            opponent_score: game.visitor_score,
            number_of_outs: game.number_of_outs,
            day_night: game.day_night.clone(),
            completion_info: game.completion_info.clone(),
            forfeit_info: game.forfeit_info.clone(),
            protest_info: game.protest_info.clone(),
            park_id: game.park_id.clone(),
            attendance: game.attendance,
            time_of_game: game.time_of_game,
            // 20
            line_score: game.home_line_score.clone(),
            opponent_line_score: game.visitor_line_score.clone(),
            ab: game.home_ab,
            hits: game.home_hits,
            doubles: game.home_doubles.clone(),
            triples: game.home_triples.clone(),
            homeruns: game.home_homeruns.clone(),
            rbi: game.home_rbi.clone(),
            sac_hits: game.home_sac_hits,
            sac_flies: game.home_sac_flies.clone(),
            // 30
            hbp: game.home_hbp.clone(),
            walks: game.home_walks.clone(),
            intentional_walks: game.home_intentional_walks.clone(),
            strikeouts: game.home_strikeouts.clone(),
            stolen_bases: game.home_stolen_bases.clone(),
            caught_stealing: game.home_caught_stealing.clone(),
            gidp: game.home_gidp.clone(),
            catcher_interference: game.home_catcher_interference.clone(),
            left_on_base: game.home_left_on_base.clone(),
            pitchers_used: game.home_pitchers_used,
            // 40
            individual_earned_runs: game.home_individual_earned_runs.clone(),
            team_earned_runs: game.home_team_earned_runs,
            wild_pitches: game.home_wild_pitches.clone(),
            balks: game.home_balks,
            putouts: game.home_putouts.clone(),
            assists: game.home_assists.clone(),
            errors: game.home_errors.clone(),
            passed_balls: game.home_passed_balls.clone(),
            double_plays: game.home_double_plays.clone(),
            triple_plays: game.home_triple_plays,
            // 50
            opponent_ab: game.visitor_ab,
            opponent_hits: game.visitor_hits,
            opponent_doubles: game.visitor_doubles.clone(),
            opponent_triples: game.visitor_triples.clone(),
            opponent_homeruns: game.visitor_homeruns.clone(),
            opponent_rbi: game.visitor_rbi.clone(),
            opponent_sac_hits: game.visitor_sac_hits,
            opponent_sac_flies: game.visitor_sac_flies.clone(),
            opponent_hbp: game.visitor_hbp.clone(),
            opponent_walks: game.visitor_walks.clone(),
            // 60
            opponent_intentional_walks: game.visitor_intentional_walks.clone(),
            opponent_strikeouts: game.visitor_strikeouts.clone(),
            opponent_stolen_bases: game.visitor_stolen_bases.clone(),
            opponent_caught_stealing: game.visitor_caught_stealing.clone(),
            opponent_gidp: game.visitor_gidp.clone(),
            opponent_catcher_interference: game.visitor_catcher_interference.clone(),
            opponent_left_on_base: game.visitor_left_on_base.clone(),
            opponent_pitchers_used: game.visitor_pitchers_used,
            opponent_individual_earned_runs: game.visitor_individual_earned_runs.clone(),
            opponent_team_earned_runs: game.visitor_team_earned_runs,
            // 70
            opponent_wild_pitches: game.visitor_wild_pitches.clone(),
            opponent_balks: game.visitor_balks,
            opponent_putouts: game.visitor_putouts.clone(),
            opponent_assists: game.visitor_assists.clone(),
            opponent_errors: game.visitor_errors.clone(),
            opponent_passed_balls: game.visitor_passed_balls.clone(),
            opponent_double_plays: game.visitor_double_plays.clone(),
            opponent_triple_plays: game.visitor_triple_plays,
            home_plate_umpire_name: game.home_plate_umpire_name.clone(),
            home_plate_umpire_id: game.home_plate_umpire_id.clone(),
            // 80
            first_base_umpire_name: game.first_base_umpire_name.clone(),
            first_base_umpire_id: game.first_base_umpire_id.clone(),
            second_base_umpire_name: game.second_base_umpire_name.clone(),
            second_base_umpire_id: game.second_base_umpire_id.clone(),
            third_base_umpire_name: game.third_base_umpire_name.clone(),
            third_base_umpire_id: game.third_base_umpire_id.clone(),
            left_field_umpire_name: game.left_field_umpire_name.clone(),
            left_field_umpire_id: game.left_field_umpire_id.clone(),
            right_field_umpire_name: game.right_field_umpire_name.clone(),
            right_field_umpire_id: game.right_field_umpire_id.clone(),
            // 90
            manager_name: game.home_manager_name.clone(),
            manager_id: game.home_manager_id.clone(),
            opponent_manager_name: game.visitor_manager_name.clone(),
            opponent_manager_id: game.visitor_manager_id.clone(),
            winning_pitcher_name: game.winning_pitcher_name.clone(),
            winning_pitcher_id: game.winning_pitcher_id.clone(),
            losing_pitcher_name: game.losing_pitcher_name.clone(),
            losing_pitcher_id: game.losing_pitcher_id.clone(),
            saving_pitcher_name: game.saving_pitcher_name.clone(),
            saving_pitcher_id: game.saving_pitcher_id.clone(),
            // 100
            gwrbi_player_name: game.gwrbi_player_name.clone(),
            gwrbi_player_id: game.gwrbi_player_id.clone(),
            starter_name: game.home_starter_name.clone(),
            starter_id: game.home_starter_id.clone(),
            opponent_starter_name: game.visitor_starter_name.clone(),
            opponent_starter_id: game.visitor_starter_id.clone(),
            lineup_1_id: game.home_1_id.clone(),
            lineup_1_name: game.home_1_name.clone(),
            lineup_1_pos: game.home_1_pos.clone(),
            lineup_2_id: game.home_2_id.clone(),
            // 110
            lineup_2_name: game.home_2_name.clone(),
            lineup_2_pos: game.home_2_pos.clone(),
            lineup_3_id: game.home_3_id.clone(),
            lineup_3_name: game.home_3_name.clone(),
            lineup_3_pos: game.home_3_pos.clone(),
            lineup_4_id: game.home_4_id.clone(),
            lineup_4_name: game.home_4_name.clone(),
            lineup_4_pos: game.home_4_pos.clone(),
            lineup_5_id: game.home_5_id.clone(),
            lineup_5_name: game.home_5_name.clone(),
            // 120
            lineup_5_pos: game.home_5_pos.clone(),
            lineup_6_id: game.home_6_id.clone(),
            lineup_6_name: game.home_6_name.clone(),
            lineup_6_pos: game.home_6_pos.clone(),
            lineup_7_id: game.home_7_id.clone(),
            lineup_7_name: game.home_7_name.clone(),
            lineup_7_pos: game.home_7_pos.clone(),
            lineup_8_id: game.home_8_id.clone(),
            lineup_8_name: game.home_8_name.clone(),
            lineup_8_pos: game.home_8_pos.clone(),
            // 130
            lineup_9_id: game.home_9_id.clone(),
            lineup_9_name: game.home_9_name.clone(),
            lineup_9_pos: game.home_9_pos.clone(),
            opponent_1_id: game.visitor_1_id.clone(),
            opponent_1_name: game.visitor_1_name.clone(),
            opponent_1_pos: game.visitor_1_pos.clone(),
            opponent_2_id: game.visitor_2_id.clone(),
            opponent_2_name: game.visitor_2_name.clone(),
            opponent_2_pos: game.visitor_2_pos.clone(),
            opponent_3_id: game.visitor_3_id.clone(),
            // 140
            opponent_3_name: game.visitor_3_name.clone(),
            opponent_3_pos: game.visitor_3_pos.clone(),
            opponent_4_id: game.visitor_4_id.clone(),
            opponent_4_name: game.visitor_4_name.clone(),
            opponent_4_pos: game.visitor_4_pos.clone(),
            opponent_5_id: game.visitor_5_id.clone(),
            opponent_5_name: game.visitor_5_name.clone(),
            opponent_5_pos: game.visitor_5_pos.clone(),
            opponent_6_id: game.visitor_6_id.clone(),
            opponent_6_name: game.visitor_6_name.clone(),
            // 150
            opponent_6_pos: game.visitor_6_pos.clone(),
            opponent_7_id: game.visitor_7_id.clone(),
            opponent_7_name: game.visitor_7_name.clone(),
            opponent_7_pos: game.visitor_7_pos.clone(),
            opponent_8_id: game.visitor_8_id.clone(),
            opponent_8_name: game.visitor_8_name.clone(),
            opponent_8_pos: game.visitor_8_pos.clone(),
            opponent_9_id: game.visitor_9_id.clone(),
            opponent_9_name: game.visitor_9_name.clone(),
            opponent_9_pos: game.visitor_9_pos.clone(),
            // 160
            additional_info: game.additional_info.clone(),
            acquisition_info: game.acquisition_info.clone(),
        }
    }

    fn from_visitor_team(game: &GameLog) -> TeamGameLog {
        TeamGameLog {
            game_id: game.game_id.clone(),
            w: game.visitor_score > game.home_score,
            l: game.visitor_score < game.home_score,
            t: game.visitor_score == game.home_score,
            // 1
            date: game.date,
            number_of_game: game.number_of_game.clone(),
            day_of_week: game.day_of_week.clone(),
            team: game.visitor_team.clone(),
            league: game.visitor_league.clone(),
            team_game_number: game.visitor_team_game_number,
            opponent_team: game.home_team.clone(),
            opponent_league: game.home_league.clone(),
            opponent_team_game_number: game.home_team_game_number,
            // 10
            score: game.visitor_score,
            opponent_score: game.home_score,
            number_of_outs: game.number_of_outs,
            day_night: game.day_night.clone(),
            completion_info: game.completion_info.clone(),
            forfeit_info: game.forfeit_info.clone(),
            protest_info: game.protest_info.clone(),
            park_id: game.park_id.clone(),
            attendance: game.attendance,
            time_of_game: game.time_of_game,
            // 20
            line_score: game.visitor_line_score.clone(),
            opponent_line_score: game.home_line_score.clone(),
            ab: game.visitor_ab,
            hits: game.visitor_hits,
            doubles: game.visitor_doubles.clone(),
            triples: game.visitor_triples.clone(),
            homeruns: game.visitor_homeruns.clone(),
            rbi: game.visitor_rbi.clone(),
            sac_hits: game.visitor_sac_hits,
            sac_flies: game.visitor_sac_flies.clone(),
            // 30
            hbp: game.visitor_hbp.clone(),
            walks: game.visitor_walks.clone(),
            intentional_walks: game.visitor_intentional_walks.clone(),
            strikeouts: game.visitor_strikeouts.clone(),
            stolen_bases: game.visitor_stolen_bases.clone(),
            caught_stealing: game.visitor_caught_stealing.clone(),
            gidp: game.visitor_gidp.clone(),
            catcher_interference: game.visitor_catcher_interference.clone(),
            left_on_base: game.visitor_left_on_base.clone(),
            pitchers_used: game.visitor_pitchers_used,
            // 40
            individual_earned_runs: game.visitor_individual_earned_runs.clone(),
            team_earned_runs: game.visitor_team_earned_runs,
            wild_pitches: game.visitor_wild_pitches.clone(),
            balks: game.visitor_balks,
            putouts: game.visitor_putouts.clone(),
            assists: game.visitor_assists.clone(),
            errors: game.visitor_errors.clone(),
            passed_balls: game.visitor_passed_balls.clone(),
            double_plays: game.visitor_double_plays.clone(),
            triple_plays: game.visitor_triple_plays,
            // 50
            opponent_ab: game.home_ab,
            opponent_hits: game.home_hits,
            opponent_doubles: game.home_doubles.clone(),
            opponent_triples: game.home_triples.clone(),
            opponent_homeruns: game.home_homeruns.clone(),
            opponent_rbi: game.home_rbi.clone(),
            opponent_sac_hits: game.home_sac_hits,
            opponent_sac_flies: game.home_sac_flies.clone(),
            opponent_hbp: game.home_hbp.clone(),
            opponent_walks: game.home_walks.clone(),
            // 60
            opponent_intentional_walks: game.home_intentional_walks.clone(),
            opponent_strikeouts: game.home_strikeouts.clone(),
            opponent_stolen_bases: game.home_stolen_bases.clone(),
            opponent_caught_stealing: game.home_caught_stealing.clone(),
            opponent_gidp: game.home_gidp.clone(),
            opponent_catcher_interference: game.home_catcher_interference.clone(),
            opponent_left_on_base: game.home_left_on_base.clone(),
            opponent_pitchers_used: game.home_pitchers_used,
            opponent_individual_earned_runs: game.home_individual_earned_runs.clone(),
            opponent_team_earned_runs: game.home_team_earned_runs,
            // 70
            opponent_wild_pitches: game.home_wild_pitches.clone(),
            opponent_balks: game.home_balks,
            opponent_putouts: game.home_putouts.clone(),
            opponent_assists: game.home_assists.clone(),
            opponent_errors: game.home_errors.clone(),
            opponent_passed_balls: game.home_passed_balls.clone(),
            opponent_double_plays: game.home_double_plays.clone(),
            opponent_triple_plays: game.home_triple_plays,
            home_plate_umpire_name: game.home_plate_umpire_name.clone(),
            home_plate_umpire_id: game.home_plate_umpire_id.clone(),
            // 80
            first_base_umpire_name: game.first_base_umpire_name.clone(),
            first_base_umpire_id: game.first_base_umpire_id.clone(),
            second_base_umpire_name: game.second_base_umpire_name.clone(),
            second_base_umpire_id: game.second_base_umpire_id.clone(),
            third_base_umpire_name: game.third_base_umpire_name.clone(),
            third_base_umpire_id: game.third_base_umpire_id.clone(),
            left_field_umpire_name: game.left_field_umpire_name.clone(),
            left_field_umpire_id: game.left_field_umpire_id.clone(),
            right_field_umpire_name: game.right_field_umpire_name.clone(),
            right_field_umpire_id: game.right_field_umpire_id.clone(),
            // 90
            manager_name: game.visitor_manager_name.clone(),
            manager_id: game.visitor_manager_id.clone(),
            opponent_manager_name: game.home_manager_name.clone(),
            opponent_manager_id: game.home_manager_id.clone(),
            winning_pitcher_name: game.winning_pitcher_name.clone(),
            winning_pitcher_id: game.winning_pitcher_id.clone(),
            losing_pitcher_name: game.losing_pitcher_name.clone(),
            losing_pitcher_id: game.losing_pitcher_id.clone(),
            saving_pitcher_name: game.saving_pitcher_name.clone(),
            saving_pitcher_id: game.saving_pitcher_id.clone(),
            // 100
            gwrbi_player_name: game.gwrbi_player_name.clone(),
            gwrbi_player_id: game.gwrbi_player_id.clone(),
            starter_name: game.visitor_starter_name.clone(),
            starter_id: game.visitor_starter_id.clone(),
            opponent_starter_name: game.home_starter_name.clone(),
            opponent_starter_id: game.home_starter_id.clone(),
            lineup_1_id: game.visitor_1_id.clone(),
            lineup_1_name: game.visitor_1_name.clone(),
            lineup_1_pos: game.visitor_1_pos.clone(),
            lineup_2_id: game.visitor_2_id.clone(),
            // 110
            lineup_2_name: game.visitor_2_name.clone(),
            lineup_2_pos: game.visitor_2_pos.clone(),
            lineup_3_id: game.visitor_3_id.clone(),
            lineup_3_name: game.visitor_3_name.clone(),
            lineup_3_pos: game.visitor_3_pos.clone(),
            lineup_4_id: game.visitor_4_id.clone(),
            lineup_4_name: game.visitor_4_name.clone(),
            lineup_4_pos: game.visitor_4_pos.clone(),
            lineup_5_id: game.visitor_5_id.clone(),
            lineup_5_name: game.visitor_5_name.clone(),
            // 120
            lineup_5_pos: game.visitor_5_pos.clone(),
            lineup_6_id: game.visitor_6_id.clone(),
            lineup_6_name: game.visitor_6_name.clone(),
            lineup_6_pos: game.visitor_6_pos.clone(),
            lineup_7_id: game.visitor_7_id.clone(),
            lineup_7_name: game.visitor_7_name.clone(),
            lineup_7_pos: game.visitor_7_pos.clone(),
            lineup_8_id: game.visitor_8_id.clone(),
            lineup_8_name: game.visitor_8_name.clone(),
            lineup_8_pos: game.visitor_8_pos.clone(),
            // 130
            lineup_9_id: game.visitor_9_id.clone(),
            lineup_9_name: game.visitor_9_name.clone(),
            lineup_9_pos: game.visitor_9_pos.clone(),
            opponent_1_id: game.home_1_id.clone(),
            opponent_1_name: game.home_1_name.clone(),
            opponent_1_pos: game.home_1_pos.clone(),
            opponent_2_id: game.home_2_id.clone(),
            opponent_2_name: game.home_2_name.clone(),
            opponent_2_pos: game.home_2_pos.clone(),
            opponent_3_id: game.home_3_id.clone(),
            // 140
            opponent_3_name: game.home_3_name.clone(),
            opponent_3_pos: game.home_3_pos.clone(),
            opponent_4_id: game.home_4_id.clone(),
            opponent_4_name: game.home_4_name.clone(),
            opponent_4_pos: game.home_4_pos.clone(),
            opponent_5_id: game.home_5_id.clone(),
            opponent_5_name: game.home_5_name.clone(),
            opponent_5_pos: game.home_5_pos.clone(),
            opponent_6_id: game.home_6_id.clone(),
            opponent_6_name: game.home_6_name.clone(),
            // 150
            opponent_6_pos: game.home_6_pos.clone(),
            opponent_7_id: game.home_7_id.clone(),
            opponent_7_name: game.home_7_name.clone(),
            opponent_7_pos: game.home_7_pos.clone(),
            opponent_8_id: game.home_8_id.clone(),
            opponent_8_name: game.home_8_name.clone(),
            opponent_8_pos: game.home_8_pos.clone(),
            opponent_9_id: game.home_9_id.clone(),
            opponent_9_name: game.home_9_name.clone(),
            opponent_9_pos: game.home_9_pos.clone(),
            // 160
            additional_info: game.additional_info.clone(),
            acquisition_info: game.acquisition_info.clone(),
        }
    }
}


impl CelEval for TeamGameLog {
    fn add_cel_variables(&self, context: &mut Context, variables: &[&str]) -> Result<(), Box<dyn Error>> {
        for name in variables {
            match *name {
                "w" => context.add_variable_from_value("w", self.w),
                "l" => context.add_variable_from_value("l", self.l),
                "t" => context.add_variable_from_value("t", self.t),
                "date" => {
                    let mut date_map: HashMap<_, Value> = HashMap::with_capacity(4);
                    date_map.insert("year", self.date.year().into());
                    date_map.insert("month", self.date.month().into());
                    date_map.insert("day", self.date.day().into());
                    context.add_variable_from_value("date", date_map)
                }
                "number_of_game" => context.add_variable_from_value("number_of_game", self.number_of_game.clone()),
                "day_of_week" => context.add_variable_from_value("day_of_week", self.day_of_week.clone()),
                "team" => context.add_variable_from_value("team", self.team.clone()),
                "league" => context.add_variable_from_value("league", self.league.clone()),
                "team_game_number" => context.add_variable_from_value("team_game_number", self.team_game_number as u64),
                "opponent_team" => context.add_variable_from_value("opponent_team", self.opponent_team.clone()),
                "opponent_league" => context.add_variable_from_value("opponent_league", self.opponent_league.clone()),
                "opponent_team_game_number" => context.add_variable("opponent_team_game_number", self.opponent_team_game_number)?,
                "score" => context.add_variable_from_value("score", self.score as u64),
                "opponent_score" => context.add_variable_from_value("opponent_score", self.opponent_score as u64),
                "number_of_outs" => context.add_variable("number_of_outs", self.number_of_outs)?,
                "day_night" => context.add_variable_from_value("day_night", self.day_night.clone()),
                "completion_info" => context.add_variable_from_value("completion_info", self.completion_info.clone()),
                "forfeit_info" => context.add_variable_from_value("forfeit_info", self.forfeit_info.clone()),
                "protest_info" => context.add_variable_from_value("protest_info", self.protest_info.clone()),
                "park_id" => context.add_variable_from_value("park_id", self.park_id.clone()),
                "attendance" => context.add_variable_from_value("attendance", self.attendance),
                "time_of_game" => context.add_variable("time_of_game", self.time_of_game)?,
                "line_score" => context.add_variable("line_score", self.line_score.as_vec())?,
                "opponent_line_score" => context.add_variable("opponent_line_score", self.opponent_line_score.as_vec())?,
                "ab" => context.add_variable("ab", self.ab)?,
                "hits" => context.add_variable("hits", self.hits)?,
                "doubles" => context.add_variable_from_value("doubles", self.doubles.clone()),
                "triples" => context.add_variable_from_value("triples", self.triples.clone()),
                "homeruns" => context.add_variable_from_value("homeruns", self.homeruns.clone()),
                "rbi" => context.add_variable_from_value("rbi", self.rbi.clone()),
                "sac_hits" => context.add_variable("sac_hits", self.sac_hits)?,
                "sac_flies" => context.add_variable_from_value("sac_flies", self.sac_flies.clone()),
                "hbp" => context.add_variable_from_value("hbp", self.hbp.clone()),
                "walks" => context.add_variable_from_value("walks", self.walks.clone()),
                "intentional_walks" => context.add_variable_from_value("intentional_walks", self.intentional_walks.clone()),
                "strikeouts" => context.add_variable_from_value("strikeouts", self.strikeouts.clone()),
                "stolen_bases" => context.add_variable_from_value("stolen_bases", self.stolen_bases.clone()),
                "caught_stealing" => context.add_variable_from_value("caught_stealing", self.caught_stealing.clone()),
                "gidp" => context.add_variable_from_value("gidp", self.gidp.clone()),
                "catcher_interference" => context.add_variable_from_value("catcher_interference", self.catcher_interference.clone()),
                "left_on_base" => context.add_variable_from_value("left_on_base", self.left_on_base.clone()),
                "pitchers_used" => context.add_variable("pitchers_used", self.pitchers_used)?,
                "individual_earned_runs" => context.add_variable_from_value("individual_earned_runs", self.individual_earned_runs.clone()),
                "team_earned_runs" => context.add_variable("team_earned_runs", self.team_earned_runs)?,
                "wild_pitches" => context.add_variable_from_value("wild_pitches", self.wild_pitches.clone()),
                "balks" => context.add_variable("balks", self.balks)?,
                "putouts" => context.add_variable_from_value("putouts", self.putouts.clone()),
                "assists" => context.add_variable_from_value("assists", self.assists.clone()),
                "errors" => context.add_variable_from_value("errors", self.errors.clone()),
                "passed_balls" => context.add_variable_from_value("passed_balls", self.passed_balls.clone()),
                "double_plays" => context.add_variable_from_value("double_plays", self.double_plays.clone()),
                "triple_plays" => context.add_variable("triple_plays", self.triple_plays)?,
                "opponent_ab" => context.add_variable("opponent_ab", self.opponent_ab)?,
                "opponent_hits" => context.add_variable("opponent_hits", self.opponent_hits)?,
                "opponent_doubles" => context.add_variable_from_value("opponent_doubles", self.opponent_doubles.clone()),
                "opponent_triples" => context.add_variable_from_value("opponent_triples", self.opponent_triples.clone()),
                "opponent_homeruns" => context.add_variable_from_value("opponent_homeruns", self.opponent_homeruns.clone()),
                "opponent_rbi" => context.add_variable_from_value("opponent_rbi", self.opponent_rbi.clone()),
                "opponent_sac_hits" => context.add_variable("opponent_sac_hits", self.opponent_sac_hits)?,
                "opponent_sac_flies" => context.add_variable_from_value("opponent_sac_flies", self.opponent_sac_flies.clone()),
                "opponent_hbp" => context.add_variable_from_value("opponent_hbp", self.opponent_hbp.clone()),
                "opponent_walks" => context.add_variable_from_value("opponent_walks", self.opponent_walks.clone()),
                "opponent_intentional_walks" => context.add_variable_from_value("opponent_intentional_walks", self.opponent_intentional_walks.clone()),
                "opponent_strikeouts" => context.add_variable_from_value("opponent_strikeouts", self.opponent_strikeouts.clone()),
                "opponent_stolen_bases" => context.add_variable_from_value("opponent_stolen_bases", self.opponent_stolen_bases.clone()),
                "opponent_caught_stealing" => context.add_variable_from_value("opponent_caught_stealing", self.opponent_caught_stealing.clone()),
                "opponent_gidp" => context.add_variable_from_value("opponent_gidp", self.opponent_gidp.clone()),
                "opponent_catcher_interference" => context.add_variable_from_value("opponent_catcher_interference", self.opponent_catcher_interference.clone()),
                "opponent_left_on_base" => context.add_variable_from_value("opponent_left_on_base", self.opponent_left_on_base.clone()),
                "opponent_pitchers_used" => context.add_variable("opponent_pitchers_used", self.opponent_pitchers_used)?,
                "opponent_individual_earned_runs" => context.add_variable_from_value("opponent_individual_earned_runs", self.opponent_individual_earned_runs.clone()),
                "opponent_team_earned_runs" => context.add_variable("opponent_team_earned_runs", self.opponent_team_earned_runs)?,
                "opponent_wild_pitches" => context.add_variable_from_value("opponent_wild_pitches", self.opponent_wild_pitches.clone()),
                "opponent_balks" => context.add_variable("opponent_balks", self.opponent_balks)?,
                "opponent_putouts" => context.add_variable_from_value("opponent_putouts", self.opponent_putouts.clone()),
                "opponent_assists" => context.add_variable_from_value("opponent_assists", self.opponent_assists.clone()),
                "opponent_errors" => context.add_variable_from_value("opponent_errors", self.opponent_errors.clone()),
                "opponent_passed_balls" => context.add_variable_from_value("opponent_passed_balls", self.opponent_passed_balls.clone()),
                "opponent_double_plays" => context.add_variable_from_value("opponent_double_plays", self.opponent_double_plays.clone()),
                "opponent_triple_plays" => context.add_variable("opponent_triple_plays", self.opponent_triple_plays)?,
                "home_plate_umpire_name" => context.add_variable_from_value("home_plate_umpire_name", self.home_plate_umpire_name.clone()),
                "home_plate_umpire_id" => context.add_variable_from_value("home_plate_umpire_id", self.home_plate_umpire_id.clone()),
                "first_base_umpire_name" => context.add_variable_from_value("first_base_umpire_name", self.first_base_umpire_name.clone()),
                "first_base_umpire_id" => context.add_variable_from_value("first_base_umpire_id", self.first_base_umpire_id.clone()),
                "second_base_umpire_name" => context.add_variable_from_value("second_base_umpire_name", self.second_base_umpire_name.clone()),
                "second_base_umpire_id" => context.add_variable_from_value("second_base_umpire_id", self.second_base_umpire_id.clone()),
                "third_base_umpire_name" => context.add_variable_from_value("third_base_umpire_name", self.third_base_umpire_name.clone()),
                "third_base_umpire_id" => context.add_variable_from_value("third_base_umpire_id", self.third_base_umpire_id.clone()),
                "left_field_umpire_name" => context.add_variable_from_value("left_field_umpire_name", self.left_field_umpire_name.clone()),
                "left_field_umpire_id" => context.add_variable_from_value("left_field_umpire_id", self.left_field_umpire_id.clone()),
                "right_field_umpire_name" => context.add_variable_from_value("right_field_umpire_name", self.right_field_umpire_name.clone()),
                "right_field_umpire_id" => context.add_variable_from_value("right_field_umpire_id", self.right_field_umpire_id.clone()),
                "manager_id" => context.add_variable_from_value("manager_id", self.manager_id.clone()),
                "manager_name" => context.add_variable_from_value("manager_name", self.manager_name.clone()),
                "opponent_manager_id" => context.add_variable_from_value("opponent_manager_id", self.opponent_manager_id.clone()),
                "opponent_manager_name" => context.add_variable_from_value("opponent_manager_name", self.opponent_manager_name.clone()),
                "winning_pitcher_name" => context.add_variable_from_value("winning_pitcher_name", self.winning_pitcher_name.clone()),
                "winning_pitcher_id" => context.add_variable_from_value("winning_pitcher_id", self.winning_pitcher_id.clone()),
                "losing_pitcher_name" => context.add_variable_from_value("losing_pitcher_name", self.losing_pitcher_name.clone()),
                "losing_pitcher_id" => context.add_variable_from_value("losing_pitcher_id", self.losing_pitcher_id.clone()),
                "saving_pitcher_name" => context.add_variable_from_value("saving_pitcher_name", self.saving_pitcher_name.clone()),
                "saving_pitcher_id" => context.add_variable_from_value("saving_pitcher_id", self.saving_pitcher_id.clone()),
                "gwrbi_player_name" => context.add_variable_from_value("gwrbi_player_name", self.gwrbi_player_name.clone()),
                "gwrbi_player_id" => context.add_variable_from_value("gwrbi_player_id", self.gwrbi_player_id.clone()),
                "starter_name" => context.add_variable_from_value("starter_name", self.starter_name.clone()),
                "starter_id" => context.add_variable_from_value("starter_id", self.starter_id.clone()),
                "opponent_starter_name" => context.add_variable_from_value("opponent_starter_name", self.opponent_starter_name.clone()),
                "opponent_starter_id" => context.add_variable_from_value("opponent_starter_id", self.opponent_starter_id.clone()),
                "lineup_1_id" => context.add_variable_from_value("lineup_1_id", self.lineup_1_id.clone()),
                "lineup_1_name" => context.add_variable_from_value("lineup_1_name", self.lineup_1_name.clone()),
                "lineup_1_pos" => context.add_variable_from_value("lineup_1_pos", self.lineup_1_pos.clone()),
                "lineup_2_id" => context.add_variable_from_value("lineup_2_id", self.lineup_2_id.clone()),
                "lineup_2_name" => context.add_variable_from_value("lineup_2_name", self.lineup_2_name.clone()),
                "lineup_2_pos" => context.add_variable_from_value("lineup_2_pos", self.lineup_2_pos.clone()),
                "lineup_3_id" => context.add_variable_from_value("lineup_3_id", self.lineup_3_id.clone()),
                "lineup_3_name" => context.add_variable_from_value("lineup_3_name", self.lineup_3_name.clone()),
                "lineup_3_pos" => context.add_variable_from_value("lineup_3_pos", self.lineup_3_pos.clone()),
                "lineup_4_id" => context.add_variable_from_value("lineup_4_id", self.lineup_4_id.clone()),
                "lineup_4_name" => context.add_variable_from_value("lineup_4_name", self.lineup_4_name.clone()),
                "lineup_4_pos" => context.add_variable_from_value("lineup_4_pos", self.lineup_4_pos.clone()),
                "lineup_5_id" => context.add_variable_from_value("lineup_5_id", self.lineup_5_id.clone()),
                "lineup_5_name" => context.add_variable_from_value("lineup_5_name", self.lineup_5_name.clone()),
                "lineup_5_pos" => context.add_variable_from_value("lineup_5_pos", self.lineup_5_pos.clone()),
                "lineup_6_id" => context.add_variable_from_value("lineup_6_id", self.lineup_6_id.clone()),
                "lineup_6_name" => context.add_variable_from_value("lineup_6_name", self.lineup_6_name.clone()),
                "lineup_6_pos" => context.add_variable_from_value("lineup_6_pos", self.lineup_6_pos.clone()),
                "lineup_7_id" => context.add_variable_from_value("lineup_7_id", self.lineup_7_id.clone()),
                "lineup_7_name" => context.add_variable_from_value("lineup_7_name", self.lineup_7_name.clone()),
                "lineup_7_pos" => context.add_variable_from_value("lineup_7_pos", self.lineup_7_pos.clone()),
                "lineup_8_id" => context.add_variable_from_value("lineup_8_id", self.lineup_8_id.clone()),
                "lineup_8_name" => context.add_variable_from_value("lineup_8_name", self.lineup_8_name.clone()),
                "lineup_8_pos" => context.add_variable_from_value("lineup_8_pos", self.lineup_8_pos.clone()),
                "lineup_9_id" => context.add_variable_from_value("lineup_9_id", self.lineup_9_id.clone()),
                "lineup_9_name" => context.add_variable_from_value("lineup_9_name", self.lineup_9_name.clone()),
                "lineup_9_pos" => context.add_variable_from_value("lineup_9_pos", self.lineup_9_pos.clone()),
                "opponent_1_id" => context.add_variable_from_value("opponent_1_id", self.opponent_1_id.clone()),
                "opponent_1_name" => context.add_variable_from_value("opponent_1_name", self.opponent_1_name.clone()),
                "opponent_1_pos" => context.add_variable_from_value("opponent_1_pos", self.opponent_1_pos.clone()),
                "opponent_2_id" => context.add_variable_from_value("opponent_2_id", self.opponent_2_id.clone()),
                "opponent_2_name" => context.add_variable_from_value("opponent_2_name", self.opponent_2_name.clone()),
                "opponent_2_pos" => context.add_variable_from_value("opponent_2_pos", self.opponent_2_pos.clone()),
                "opponent_3_id" => context.add_variable_from_value("opponent_3_id", self.opponent_3_id.clone()),
                "opponent_3_name" => context.add_variable_from_value("opponent_3_name", self.opponent_3_name.clone()),
                "opponent_3_pos" => context.add_variable_from_value("opponent_3_pos", self.opponent_3_pos.clone()),
                "opponent_4_id" => context.add_variable_from_value("opponent_4_id", self.opponent_4_id.clone()),
                "opponent_4_name" => context.add_variable_from_value("opponent_4_name", self.opponent_4_name.clone()),
                "opponent_4_pos" => context.add_variable_from_value("opponent_4_pos", self.opponent_4_pos.clone()),
                "opponent_5_id" => context.add_variable_from_value("opponent_5_id", self.opponent_5_id.clone()),
                "opponent_5_name" => context.add_variable_from_value("opponent_5_name", self.opponent_5_name.clone()),
                "opponent_5_pos" => context.add_variable_from_value("opponent_5_pos", self.opponent_5_pos.clone()),
                "opponent_6_id" => context.add_variable_from_value("opponent_6_id", self.opponent_6_id.clone()),
                "opponent_6_name" => context.add_variable_from_value("opponent_6_name", self.opponent_6_name.clone()),
                "opponent_6_pos" => context.add_variable_from_value("opponent_6_pos", self.opponent_6_pos.clone()),
                "opponent_7_id" => context.add_variable_from_value("opponent_7_id", self.opponent_7_id.clone()),
                "opponent_7_name" => context.add_variable_from_value("opponent_7_name", self.opponent_7_name.clone()),
                "opponent_7_pos" => context.add_variable_from_value("opponent_7_pos", self.opponent_7_pos.clone()),
                "opponent_8_id" => context.add_variable_from_value("opponent_8_id", self.opponent_8_id.clone()),
                "opponent_8_name" => context.add_variable_from_value("opponent_8_name", self.opponent_8_name.clone()),
                "opponent_8_pos" => context.add_variable_from_value("opponent_8_pos", self.opponent_8_pos.clone()),
                "opponent_9_id" => context.add_variable_from_value("opponent_9_id", self.opponent_9_id.clone()),
                "opponent_9_name" => context.add_variable_from_value("opponent_9_name", self.opponent_9_name.clone()),
                "opponent_9_pos" => context.add_variable_from_value("opponent_9_pos", self.opponent_9_pos.clone()),
                "additional_info" => context.add_variable_from_value("additional_info", self.additional_info.clone()),
                "acquisition_info" => context.add_variable_from_value("acquisition_info", self.acquisition_info.clone()),
                _ => {},
            }
        }

        Ok(())
    }

    fn check_cel_variables(variables: &[&str]) -> bool {
        for name in variables {
            match *name {
                "w" => {}
                "l" => {}
                "t" => {}
                "date" => {}
                "number_of_game" => {}
                "day_of_week" => {}
                "team" => {}
                "league" => {}
                "team_game_number" => {}
                "opponent_team" => {}
                "opponent_league" => {}
                "opponent_team_game_number" => {}
                "score" => {}
                "opponent_score" => {}
                "number_of_outs" => {}
                "day_night" => {}
                "completion_info" => {}
                "forfeit_info" => {}
                "protest_info" => {}
                "park_id" => {}
                "attendance" => {}
                "time_of_game" => {}
                "line_score" => {}
                "opponent_line_score" => {}
                "ab" => {}
                "hits" => {}
                "doubles" => {}
                "triples" => {}
                "homeruns" => {}
                "rbi" => {}
                "sac_hits" => {}
                "sac_flies" => {}
                "hbp" => {}
                "walks" => {}
                "intentional_walks" => {}
                "strikeouts" => {}
                "stolen_bases" => {}
                "caught_stealing" => {}
                "gidp" => {}
                "catcher_interference" => {}
                "left_on_base" => {}
                "pitchers_used" => {}
                "individual_earned_runs" => {}
                "team_earned_runs" => {}
                "wild_pitches" => {}
                "balks" => {}
                "putouts" => {}
                "assists" => {}
                "errors" => {}
                "passed_balls" => {}
                "double_plays" => {}
                "triple_plays" => {}
                "opponent_ab" => {}
                "opponent_hits" => {}
                "opponent_doubles" => {}
                "opponent_triples" => {}
                "opponent_homeruns" => {}
                "opponent_rbi" => {}
                "opponent_sac_hits" => {}
                "opponent_sac_flies" => {}
                "opponent_hbp" => {}
                "opponent_walks" => {}
                "opponent_intentional_walks" => {}
                "opponent_strikeouts" => {}
                "opponent_stolen_bases" => {}
                "opponent_caught_stealing" => {}
                "opponent_gidp" => {}
                "opponent_catcher_interference" => {}
                "opponent_left_on_base" => {}
                "opponent_pitchers_used" => {}
                "opponent_individual_earned_runs" => {}
                "opponent_team_earned_runs" => {}
                "opponent_wild_pitches" => {}
                "opponent_balks" => {}
                "opponent_putouts" => {}
                "opponent_assists" => {}
                "opponent_errors" => {}
                "opponent_passed_balls" => {}
                "opponent_double_plays" => {}
                "opponent_triple_plays" => {}
                "home_plate_umpire_name" => {}
                "home_plate_umpire_id" => {}
                "first_base_umpire_name" => {}
                "first_base_umpire_id" => {}
                "second_base_umpire_name" => {}
                "second_base_umpire_id" => {}
                "third_base_umpire_name" => {}
                "third_base_umpire_id" => {}
                "left_field_umpire_name" => {}
                "left_field_umpire_id" => {}
                "right_field_umpire_name" => {}
                "right_field_umpire_id" => {}
                "manager_id" => {}
                "manager_name" => {}
                "opponent_manager_id" => {}
                "opponent_manager_name" => {}
                "winning_pitcher_name" => {}
                "winning_pitcher_id" => {}
                "losing_pitcher_name" => {}
                "losing_pitcher_id" => {}
                "saving_pitcher_name" => {}
                "saving_pitcher_id" => {}
                "gwrbi_player_name" => {}
                "gwrbi_player_id" => {}
                "starter_name" => {}
                "starter_id" => {}
                "opponent_starter_name" => {}
                "opponent_starter_id" => {}
                "lineup_1_id" => {}
                "lineup_1_name" => {}
                "lineup_1_pos" => {}
                "lineup_2_id" => {}
                "lineup_2_name" => {}
                "lineup_2_pos" => {}
                "lineup_3_id" => {}
                "lineup_3_name" => {}
                "lineup_3_pos" => {}
                "lineup_4_id" => {}
                "lineup_4_name" => {}
                "lineup_4_pos" => {}
                "lineup_5_id" => {}
                "lineup_5_name" => {}
                "lineup_5_pos" => {}
                "lineup_6_id" => {}
                "lineup_6_name" => {}
                "lineup_6_pos" => {}
                "lineup_7_id" => {}
                "lineup_7_name" => {}
                "lineup_7_pos" => {}
                "lineup_8_id" => {}
                "lineup_8_name" => {}
                "lineup_8_pos" => {}
                "lineup_9_id" => {}
                "lineup_9_name" => {}
                "lineup_9_pos" => {}
                "opponent_1_id" => {}
                "opponent_1_name" => {}
                "opponent_1_pos" => {}
                "opponent_2_id" => {}
                "opponent_2_name" => {}
                "opponent_2_pos" => {}
                "opponent_3_id" => {}
                "opponent_3_name" => {}
                "opponent_3_pos" => {}
                "opponent_4_id" => {}
                "opponent_4_name" => {}
                "opponent_4_pos" => {}
                "opponent_5_id" => {}
                "opponent_5_name" => {}
                "opponent_5_pos" => {}
                "opponent_6_id" => {}
                "opponent_6_name" => {}
                "opponent_6_pos" => {}
                "opponent_7_id" => {}
                "opponent_7_name" => {}
                "opponent_7_pos" => {}
                "opponent_8_id" => {}
                "opponent_8_name" => {}
                "opponent_8_pos" => {}
                "opponent_9_id" => {}
                "opponent_9_name" => {}
                "opponent_9_pos" => {}
                "additional_info" => {}
                "acquisition_info" => {}
                _ => return false,
            }
        }

        true
    }
}


impl SearchKey for TeamGameLog {
    fn id(&self) -> &str { &self.game_id }

    fn subject_id(&self) -> &str { &self.team }

    fn order(&self, _career: bool) -> u16 { self.team_game_number }
}


impl TeamGameLogSmall {
    fn from_home_team(game: &GameLogSmall) -> TeamGameLogSmall {
        TeamGameLogSmall {
            game_id: game.game_id.clone(),
            w: game.home_score > game.visitor_score,
            l: game.home_score < game.visitor_score,
            t: game.home_score == game.visitor_score,
            // 1
            date: game.date,
            number_of_game: game.number_of_game.clone(),
            team: game.home_team.clone(),
            league: game.home_league.clone(),
            team_game_number: game.home_team_game_number,
            opponent_team: game.visitor_team.clone(),
            opponent_league: game.visitor_league.clone(),
            opponent_team_game_number: game.visitor_team_game_number,
            // 10
            score: game.home_score,
            opponent_score: game.visitor_score,
            number_of_outs: game.number_of_outs,
            day_night: game.day_night.clone(),
            completion_info: game.completion_info.clone(),
            forfeit_info: game.forfeit_info.clone(),
            protest_info: game.protest_info.clone(),
            park_id: game.park_id.clone(),
            attendance: game.attendance,
            time_of_game: game.time_of_game,
            // 20
            line_score: game.home_line_score.clone(),
            opponent_line_score: game.visitor_line_score.clone(),
            ab: game.home_ab,
            hits: game.home_hits,
            doubles: game.home_doubles.clone(),
            triples: game.home_triples.clone(),
            homeruns: game.home_homeruns.clone(),
            rbi: game.home_rbi.clone(),
            sac_hits: game.home_sac_hits,
            sac_flies: game.home_sac_flies.clone(),
            // 30
            hbp: game.home_hbp.clone(),
            walks: game.home_walks.clone(),
            intentional_walks: game.home_intentional_walks.clone(),
            strikeouts: game.home_strikeouts.clone(),
            stolen_bases: game.home_stolen_bases.clone(),
            caught_stealing: game.home_caught_stealing.clone(),
            gidp: game.home_gidp.clone(),
            catcher_interference: game.home_catcher_interference.clone(),
            left_on_base: game.home_left_on_base.clone(),
            pitchers_used: game.home_pitchers_used,
            // 40
            individual_earned_runs: game.home_individual_earned_runs.clone(),
            team_earned_runs: game.home_team_earned_runs,
            wild_pitches: game.home_wild_pitches.clone(),
            balks: game.home_balks,
            putouts: game.home_putouts.clone(),
            assists: game.home_assists.clone(),
            errors: game.home_errors.clone(),
            passed_balls: game.home_passed_balls.clone(),
            double_plays: game.home_double_plays.clone(),
            triple_plays: game.home_triple_plays,
            // 50
            opponent_ab: game.visitor_ab,
            opponent_hits: game.visitor_hits,
            opponent_doubles: game.visitor_doubles.clone(),
            opponent_triples: game.visitor_triples.clone(),
            opponent_homeruns: game.visitor_homeruns.clone(),
            opponent_rbi: game.visitor_rbi.clone(),
            opponent_sac_hits: game.visitor_sac_hits,
            opponent_sac_flies: game.visitor_sac_flies.clone(),
            opponent_hbp: game.visitor_hbp.clone(),
            opponent_walks: game.visitor_walks.clone(),
            // 60
            opponent_intentional_walks: game.visitor_intentional_walks.clone(),
            opponent_strikeouts: game.visitor_strikeouts.clone(),
            opponent_stolen_bases: game.visitor_stolen_bases.clone(),
            opponent_caught_stealing: game.visitor_caught_stealing.clone(),
            opponent_gidp: game.visitor_gidp.clone(),
            opponent_catcher_interference: game.visitor_catcher_interference.clone(),
            opponent_left_on_base: game.visitor_left_on_base.clone(),
            opponent_pitchers_used: game.visitor_pitchers_used,
            opponent_individual_earned_runs: game.visitor_individual_earned_runs.clone(),
            opponent_team_earned_runs: game.visitor_team_earned_runs,
            // 70
            opponent_wild_pitches: game.visitor_wild_pitches.clone(),
            opponent_balks: game.visitor_balks,
            opponent_putouts: game.visitor_putouts.clone(),
            opponent_assists: game.visitor_assists.clone(),
            opponent_errors: game.visitor_errors.clone(),
            opponent_passed_balls: game.visitor_passed_balls.clone(),
            opponent_double_plays: game.visitor_double_plays.clone(),
            opponent_triple_plays: game.visitor_triple_plays,
        }
    }

    fn from_visitor_team(game: &GameLogSmall) -> TeamGameLogSmall {
        TeamGameLogSmall {
            game_id: game.game_id.clone(),
            w: game.visitor_score > game.home_score,
            l: game.visitor_score < game.home_score,
            t: game.visitor_score == game.home_score,
            // 1
            date: game.date,
            number_of_game: game.number_of_game.clone(),
            team: game.visitor_team.clone(),
            league: game.visitor_league.clone(),
            team_game_number: game.visitor_team_game_number,
            opponent_team: game.home_team.clone(),
            opponent_league: game.home_league.clone(),
            opponent_team_game_number: game.home_team_game_number,
            // 10
            score: game.visitor_score,
            opponent_score: game.home_score,
            number_of_outs: game.number_of_outs,
            day_night: game.day_night.clone(),
            completion_info: game.completion_info.clone(),
            forfeit_info: game.forfeit_info.clone(),
            protest_info: game.protest_info.clone(),
            park_id: game.park_id.clone(),
            attendance: game.attendance,
            time_of_game: game.time_of_game,
            // 20
            line_score: game.visitor_line_score.clone(),
            opponent_line_score: game.home_line_score.clone(),
            ab: game.visitor_ab,
            hits: game.visitor_hits,
            doubles: game.visitor_doubles.clone(),
            triples: game.visitor_triples.clone(),
            homeruns: game.visitor_homeruns.clone(),
            rbi: game.visitor_rbi.clone(),
            sac_hits: game.visitor_sac_hits,
            sac_flies: game.visitor_sac_flies.clone(),
            // 30
            hbp: game.visitor_hbp.clone(),
            walks: game.visitor_walks.clone(),
            intentional_walks: game.visitor_intentional_walks.clone(),
            strikeouts: game.visitor_strikeouts.clone(),
            stolen_bases: game.visitor_stolen_bases.clone(),
            caught_stealing: game.visitor_caught_stealing.clone(),
            gidp: game.visitor_gidp.clone(),
            catcher_interference: game.visitor_catcher_interference.clone(),
            left_on_base: game.visitor_left_on_base.clone(),
            pitchers_used: game.visitor_pitchers_used,
            // 40
            individual_earned_runs: game.visitor_individual_earned_runs.clone(),
            team_earned_runs: game.visitor_team_earned_runs,
            wild_pitches: game.visitor_wild_pitches.clone(),
            balks: game.visitor_balks,
            putouts: game.visitor_putouts.clone(),
            assists: game.visitor_assists.clone(),
            errors: game.visitor_errors.clone(),
            passed_balls: game.visitor_passed_balls.clone(),
            double_plays: game.visitor_double_plays.clone(),
            triple_plays: game.visitor_triple_plays,
            // 50
            opponent_ab: game.home_ab,
            opponent_hits: game.home_hits,
            opponent_doubles: game.home_doubles.clone(),
            opponent_triples: game.home_triples.clone(),
            opponent_homeruns: game.home_homeruns.clone(),
            opponent_rbi: game.home_rbi.clone(),
            opponent_sac_hits: game.home_sac_hits,
            opponent_sac_flies: game.home_sac_flies.clone(),
            opponent_hbp: game.home_hbp.clone(),
            opponent_walks: game.home_walks.clone(),
            // 60
            opponent_intentional_walks: game.home_intentional_walks.clone(),
            opponent_strikeouts: game.home_strikeouts.clone(),
            opponent_stolen_bases: game.home_stolen_bases.clone(),
            opponent_caught_stealing: game.home_caught_stealing.clone(),
            opponent_gidp: game.home_gidp.clone(),
            opponent_catcher_interference: game.home_catcher_interference.clone(),
            opponent_left_on_base: game.home_left_on_base.clone(),
            opponent_pitchers_used: game.home_pitchers_used,
            opponent_individual_earned_runs: game.home_individual_earned_runs.clone(),
            opponent_team_earned_runs: game.home_team_earned_runs,
            // 70
            opponent_wild_pitches: game.home_wild_pitches.clone(),
            opponent_balks: game.home_balks,
            opponent_putouts: game.home_putouts.clone(),
            opponent_assists: game.home_assists.clone(),
            opponent_errors: game.home_errors.clone(),
            opponent_passed_balls: game.home_passed_balls.clone(),
            opponent_double_plays: game.home_double_plays.clone(),
            opponent_triple_plays: game.home_triple_plays,
        }
    }
}


impl CelEval for TeamGameLogSmall {
    fn add_cel_variables(&self, context: &mut Context, variables: &[&str]) -> Result<(), Box<dyn Error>> {
        for name in variables {
            match *name {
                "w" => context.add_variable_from_value("w", self.w),
                "l" => context.add_variable_from_value("l", self.l),
                "t" => context.add_variable_from_value("t", self.t),
                "date" => {
                    let mut date_map: HashMap<_, Value> = HashMap::with_capacity(4);
                    date_map.insert("year", self.date.year().into());
                    date_map.insert("month", self.date.month().into());
                    date_map.insert("day", self.date.day().into());
                    context.add_variable_from_value("date", date_map)
                }
                "number_of_game" => context.add_variable_from_value("number_of_game", self.number_of_game.clone()),
                "team" => context.add_variable_from_value("team", self.team.clone()),
                "league" => context.add_variable_from_value("league", self.league.clone()),
                "team_game_number" => context.add_variable_from_value("team_game_number", self.team_game_number as u64),
                "opponent_team" => context.add_variable_from_value("opponent_team", self.opponent_team.clone()),
                "opponent_league" => context.add_variable_from_value("opponent_league", self.opponent_league.clone()),
                "opponent_team_game_number" => context.add_variable("opponent_team_game_number", self.opponent_team_game_number)?,
                "score" => context.add_variable_from_value("score", self.score as u64),
                "opponent_score" => context.add_variable_from_value("opponent_score", self.opponent_score as u64),
                "number_of_outs" => context.add_variable("number_of_outs", self.number_of_outs)?,
                "day_night" => context.add_variable_from_value("day_night", self.day_night.clone()),
                "completion_info" => context.add_variable_from_value("completion_info", self.completion_info.clone()),
                "forfeit_info" => context.add_variable_from_value("forfeit_info", self.forfeit_info.clone()),
                "protest_info" => context.add_variable_from_value("protest_info", self.protest_info.clone()),
                "park_id" => context.add_variable_from_value("park_id", self.park_id.clone()),
                "attendance" => context.add_variable_from_value("attendance", self.attendance),
                "time_of_game" => context.add_variable("time_of_game", self.time_of_game)?,
                "line_score" => context.add_variable("line_score", self.line_score.as_vec())?,
                "opponent_line_score" => context.add_variable("opponent_line_score", self.opponent_line_score.as_vec())?,
                "ab" => context.add_variable("ab", self.ab)?,
                "hits" => context.add_variable("hits", self.hits)?,
                "doubles" => context.add_variable_from_value("doubles", self.doubles.clone()),
                "triples" => context.add_variable_from_value("triples", self.triples.clone()),
                "homeruns" => context.add_variable_from_value("homeruns", self.homeruns.clone()),
                "rbi" => context.add_variable_from_value("rbi", self.rbi.clone()),
                "sac_hits" => context.add_variable("sac_hits", self.sac_hits)?,
                "sac_flies" => context.add_variable_from_value("sac_flies", self.sac_flies.clone()),
                "hbp" => context.add_variable_from_value("hbp", self.hbp.clone()),
                "walks" => context.add_variable_from_value("walks", self.walks.clone()),
                "intentional_walks" => context.add_variable_from_value("intentional_walks", self.intentional_walks.clone()),
                "strikeouts" => context.add_variable_from_value("strikeouts", self.strikeouts.clone()),
                "stolen_bases" => context.add_variable_from_value("stolen_bases", self.stolen_bases.clone()),
                "caught_stealing" => context.add_variable_from_value("caught_stealing", self.caught_stealing.clone()),
                "gidp" => context.add_variable_from_value("gidp", self.gidp.clone()),
                "catcher_interference" => context.add_variable_from_value("catcher_interference", self.catcher_interference.clone()),
                "left_on_base" => context.add_variable_from_value("left_on_base", self.left_on_base.clone()),
                "pitchers_used" => context.add_variable("pitchers_used", self.pitchers_used)?,
                "individual_earned_runs" => context.add_variable_from_value("individual_earned_runs", self.individual_earned_runs.clone()),
                "team_earned_runs" => context.add_variable("team_earned_runs", self.team_earned_runs)?,
                "wild_pitches" => context.add_variable_from_value("wild_pitches", self.wild_pitches.clone()),
                "balks" => context.add_variable("balks", self.balks)?,
                "putouts" => context.add_variable_from_value("putouts", self.putouts.clone()),
                "assists" => context.add_variable_from_value("assists", self.assists.clone()),
                "errors" => context.add_variable_from_value("errors", self.errors.clone()),
                "passed_balls" => context.add_variable_from_value("passed_balls", self.passed_balls.clone()),
                "double_plays" => context.add_variable_from_value("double_plays", self.double_plays.clone()),
                "triple_plays" => context.add_variable("triple_plays", self.triple_plays)?,
                "opponent_ab" => context.add_variable("opponent_ab", self.opponent_ab)?,
                "opponent_hits" => context.add_variable("opponent_hits", self.opponent_hits)?,
                "opponent_doubles" => context.add_variable_from_value("opponent_doubles", self.opponent_doubles.clone()),
                "opponent_triples" => context.add_variable_from_value("opponent_triples", self.opponent_triples.clone()),
                "opponent_homeruns" => context.add_variable_from_value("opponent_homeruns", self.opponent_homeruns.clone()),
                "opponent_rbi" => context.add_variable_from_value("opponent_rbi", self.opponent_rbi.clone()),
                "opponent_sac_hits" => context.add_variable("opponent_sac_hits", self.opponent_sac_hits)?,
                "opponent_sac_flies" => context.add_variable_from_value("opponent_sac_flies", self.opponent_sac_flies.clone()),
                "opponent_hbp" => context.add_variable_from_value("opponent_hbp", self.opponent_hbp.clone()),
                "opponent_walks" => context.add_variable_from_value("opponent_walks", self.opponent_walks.clone()),
                "opponent_intentional_walks" => context.add_variable_from_value("opponent_intentional_walks", self.opponent_intentional_walks.clone()),
                "opponent_strikeouts" => context.add_variable_from_value("opponent_strikeouts", self.opponent_strikeouts.clone()),
                "opponent_stolen_bases" => context.add_variable_from_value("opponent_stolen_bases", self.opponent_stolen_bases.clone()),
                "opponent_caught_stealing" => context.add_variable_from_value("opponent_caught_stealing", self.opponent_caught_stealing.clone()),
                "opponent_gidp" => context.add_variable_from_value("opponent_gidp", self.opponent_gidp.clone()),
                "opponent_catcher_interference" => context.add_variable_from_value("opponent_catcher_interference", self.opponent_catcher_interference.clone()),
                "opponent_left_on_base" => context.add_variable_from_value("opponent_left_on_base", self.opponent_left_on_base.clone()),
                "opponent_pitchers_used" => context.add_variable("opponent_pitchers_used", self.opponent_pitchers_used)?,
                "opponent_individual_earned_runs" => context.add_variable_from_value("opponent_individual_earned_runs", self.opponent_individual_earned_runs.clone()),
                "opponent_team_earned_runs" => context.add_variable("opponent_team_earned_runs", self.opponent_team_earned_runs)?,
                "opponent_wild_pitches" => context.add_variable_from_value("opponent_wild_pitches", self.opponent_wild_pitches.clone()),
                "opponent_balks" => context.add_variable("opponent_balks", self.opponent_balks)?,
                "opponent_putouts" => context.add_variable_from_value("opponent_putouts", self.opponent_putouts.clone()),
                "opponent_assists" => context.add_variable_from_value("opponent_assists", self.opponent_assists.clone()),
                "opponent_errors" => context.add_variable_from_value("opponent_errors", self.opponent_errors.clone()),
                "opponent_passed_balls" => context.add_variable_from_value("opponent_passed_balls", self.opponent_passed_balls.clone()),
                "opponent_double_plays" => context.add_variable_from_value("opponent_double_plays", self.opponent_double_plays.clone()),
                "opponent_triple_plays" => context.add_variable("opponent_triple_plays", self.opponent_triple_plays)?,
                _ => {},
            }
        }

        Ok(())
    }

    fn check_cel_variables(variables: &[&str]) -> bool {
        for name in variables {
            match *name {
                "w" => {}
                "l" => {}
                "t" => {}
                "date" => {}
                "number_of_game" => {}
                "team" => {}
                "league" => {}
                "team_game_number" => {}
                "opponent_team" => {}
                "opponent_league" => {}
                "opponent_team_game_number" => {}
                "score" => {}
                "opponent_score" => {}
                "number_of_outs" => {}
                "day_night" => {}
                "completion_info" => {}
                "forfeit_info" => {}
                "protest_info" => {}
                "park_id" => {}
                "attendance" => {}
                "time_of_game" => {}
                "line_score" => {}
                "opponent_line_score" => {}
                "ab" => {}
                "hits" => {}
                "doubles" => {}
                "triples" => {}
                "homeruns" => {}
                "rbi" => {}
                "sac_hits" => {}
                "sac_flies" => {}
                "hbp" => {}
                "walks" => {}
                "intentional_walks" => {}
                "strikeouts" => {}
                "stolen_bases" => {}
                "caught_stealing" => {}
                "gidp" => {}
                "catcher_interference" => {}
                "left_on_base" => {}
                "pitchers_used" => {}
                "individual_earned_runs" => {}
                "team_earned_runs" => {}
                "wild_pitches" => {}
                "balks" => {}
                "putouts" => {}
                "assists" => {}
                "errors" => {}
                "passed_balls" => {}
                "double_plays" => {}
                "triple_plays" => {}
                "opponent_ab" => {}
                "opponent_hits" => {}
                "opponent_doubles" => {}
                "opponent_triples" => {}
                "opponent_homeruns" => {}
                "opponent_rbi" => {}
                "opponent_sac_hits" => {}
                "opponent_sac_flies" => {}
                "opponent_hbp" => {}
                "opponent_walks" => {}
                "opponent_intentional_walks" => {}
                "opponent_strikeouts" => {}
                "opponent_stolen_bases" => {}
                "opponent_caught_stealing" => {}
                "opponent_gidp" => {}
                "opponent_catcher_interference" => {}
                "opponent_left_on_base" => {}
                "opponent_pitchers_used" => {}
                "opponent_individual_earned_runs" => {}
                "opponent_team_earned_runs" => {}
                "opponent_wild_pitches" => {}
                "opponent_balks" => {}
                "opponent_putouts" => {}
                "opponent_assists" => {}
                "opponent_errors" => {}
                "opponent_passed_balls" => {}
                "opponent_double_plays" => {}
                "opponent_triple_plays" => {}
                _ => return false,
            }
        }

        true
    }
}


impl SearchKey for TeamGameLogSmall {
    fn id(&self) -> &str { &self.game_id }

    fn subject_id(&self) -> &str { &self.team }

    fn order(&self, _career: bool) -> u16 { self.team_game_number }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_linescore() {
        let retrosheet_linescore = "01090(10)20x";
        let linescore = Linescore::split_linescore(retrosheet_linescore);
        let expected = vec![Some(0), Some(1), Some(0), Some(9), Some(0), Some(10), Some(2), Some(0), None];
        assert_eq!(linescore.linescore, expected);
    }

    #[test]
    fn retrosheet_linescore() {
        let linescore = Linescore {
            linescore: vec![Some(0), Some(1), Some(0), Some(3), Some(0), Some(10), Some(7), Some(0), None],
        };
        let parsed = linescore.retrosheet_linescore();
        let expected = "01030(10)70x";
        assert_eq!(parsed, expected);
    }

    #[test]
    fn db_linescore() {
        let linescore = Linescore {
            linescore: vec![Some(0), Some(1), Some(0), Some(3), Some(0), Some(10), Some(7), Some(0), None],
        };
        let parsed = linescore.sql_linescore();
        let expected = "0,1,0,3,0,10,7,0,x";
        assert_eq!(parsed, expected);
    }
}
