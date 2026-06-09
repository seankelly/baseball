use std::error::Error;

use baseball::retrosheet::game;

use crate::database::Sql;

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
    pub date: chrono::NaiveDate,
    // These fields are copied straight from GameLog with some movement.
    // 1
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


impl Sql for GameLog {
    fn create_table(tx: &mut Transaction) -> Result<(), Box<dyn Error>> {
        tx.execute("DROP TABLE IF EXISTS games", ())?;
        tx.execute(include_str!("sql/create_games.sql"), ())?;
        Ok(())
    }

    fn table_name<'a>() -> &'a str { "games" }

    fn read_row(row: &Row, offset: usize) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            game_id: row.get(offset)?,
            date: row.get(offset + 1)?,
            number_of_game: row.get(offset + 2)?,
            day_of_week: row.get(offset + 3)?,
            visitor_team: row.get(offset + 4)?,
            visitor_league: row.get(offset + 5)?,
            visitor_team_game_number: row.get(offset + 6)?,
            home_team: row.get(offset + 7)?,
            home_league: row.get(offset + 8)?,
            home_team_game_number: row.get(offset + 9)?,
            visitor_score: row.get(offset + 10)?,
            home_score: row.get(offset + 0)?,
            number_of_outs: row.get(offset + 0)?,
            day_night: row.get(offset + 0)?,
            completion_info: row.get(offset + 0)?,
            forfeit_info: row.get(offset + 0)?,
            protest_info: row.get(offset + 0)?,
            park_id: row.get(offset + 0)?,
            attendance: row.get(offset + 0)?,
            time_of_game: row.get(offset + 0)?,
            visitor_line_score: row.get(offset + 0)?,
            home_line_score: row.get(offset + 0)?,
            visitor_ab: row.get(offset + 0)?,
            visitor_hits: row.get(offset + 0)?,
            visitor_doubles: row.get(offset + 0)?,
            visitor_triples: row.get(offset + 0)?,
            visitor_homeruns: row.get(offset + 0)?,
            visitor_rbi: row.get(offset + 0)?,
            visitor_sac_hits: row.get(offset + 0)?,
            visitor_sac_flies: row.get(offset + 0)?,
            visitor_hbp: row.get(offset + 0)?,
            visitor_walks: row.get(offset + 0)?,
            visitor_intentional_walks: row.get(offset + 0)?,
            visitor_strikeouts: row.get(offset + 0)?,
            visitor_stolen_bases: row.get(offset + 0)?,
            visitor_caught_stealing: row.get(offset + 0)?,
            visitor_gidp: row.get(offset + 0)?,
            visitor_catcher_interference: row.get(offset + 0)?,
            visitor_left_on_base: row.get(offset + 0)?,
            visitor_pitchers_used: row.get(offset + 0)?,
            visitor_individual_earned_runs: row.get(offset + 0)?,
            visitor_team_earned_runs: row.get(offset + 0)?,
            visitor_wild_pitches: row.get(offset + 0)?,
            visitor_balks: row.get(offset + 0)?,
            visitor_putouts: row.get(offset + 0)?,
            visitor_assists: row.get(offset + 0)?,
            visitor_errors: row.get(offset + 0)?,
            visitor_passed_balls: row.get(offset + 0)?,
            visitor_double_plays: row.get(offset + 0)?,
            visitor_triple_plays: row.get(offset + 0)?,
            home_ab: row.get(offset + 0)?,
            home_hits: row.get(offset + 0)?,
            home_doubles: row.get(offset + 0)?,
            home_triples: row.get(offset + 0)?,
            home_homeruns: row.get(offset + 0)?,
            home_rbi: row.get(offset + 0)?,
            home_sac_hits: row.get(offset + 0)?,
            home_sac_flies: row.get(offset + 0)?,
            home_hbp: row.get(offset + 0)?,
            home_walks: row.get(offset + 0)?,
            home_intentional_walks: row.get(offset + 0)?,
            home_strikeouts: row.get(offset + 0)?,
            home_stolen_bases: row.get(offset + 0)?,
            home_caught_stealing: row.get(offset + 0)?,
            home_gidp: row.get(offset + 0)?,
            home_catcher_interference: row.get(offset + 0)?,
            home_left_on_base: row.get(offset + 0)?,
            home_pitchers_used: row.get(offset + 0)?,
            home_individual_earned_runs: row.get(offset + 0)?,
            home_team_earned_runs: row.get(offset + 0)?,
            home_wild_pitches: row.get(offset + 0)?,
            home_balks: row.get(offset + 0)?,
            home_putouts: row.get(offset + 0)?,
            home_assists: row.get(offset + 0)?,
            home_errors: row.get(offset + 0)?,
            home_passed_balls: row.get(offset + 0)?,
            home_double_plays: row.get(offset + 0)?,
            home_triple_plays: row.get(offset + 0)?,
            home_plate_umpire_name: row.get(offset + 0)?,
            home_plate_umpire_id: row.get(offset + 0)?,
            first_base_umpire_name: row.get(offset + 0)?,
            first_base_umpire_id: row.get(offset + 0)?,
            second_base_umpire_name: row.get(offset + 0)?,
            second_base_umpire_id: row.get(offset + 0)?,
            third_base_umpire_name: row.get(offset + 0)?,
            third_base_umpire_id: row.get(offset + 0)?,
            left_field_umpire_name: row.get(offset + 0)?,
            left_field_umpire_id: row.get(offset + 0)?,
            right_field_umpire_name: row.get(offset + 0)?,
            right_field_umpire_id: row.get(offset + 0)?,
            visitor_manager_id: row.get(offset + 0)?,
            visitor_manager_name: row.get(offset + 0)?,
            home_manager_id: row.get(offset + 0)?,
            home_manager_name: row.get(offset + 0)?,
            winning_pitcher_name: row.get(offset + 0)?,
            winning_pitcher_id: row.get(offset + 0)?,
            losing_pitcher_name: row.get(offset + 0)?,
            losing_pitcher_id: row.get(offset + 0)?,
            saving_pitcher_name: row.get(offset + 0)?,
            saving_pitcher_id: row.get(offset + 0)?,
            gwrbi_player_name: row.get(offset + 0)?,
            gwrbi_player_id: row.get(offset + 0)?,
            visitor_starter_name: row.get(offset + 0)?,
            visitor_starter_id: row.get(offset + 0)?,
            home_starter_name: row.get(offset + 0)?,
            home_starter_id: row.get(offset + 0)?,
            visitor_1_id: row.get(offset + 0)?,
            visitor_1_name: row.get(offset + 0)?,
            visitor_1_pos: row.get(offset + 0)?,
            visitor_2_id: row.get(offset + 0)?,
            visitor_2_name: row.get(offset + 0)?,
            visitor_2_pos: row.get(offset + 0)?,
            visitor_3_id: row.get(offset + 0)?,
            visitor_3_name: row.get(offset + 0)?,
            visitor_3_pos: row.get(offset + 0)?,
            visitor_4_id: row.get(offset + 0)?,
            visitor_4_name: row.get(offset + 0)?,
            visitor_4_pos: row.get(offset + 0)?,
            visitor_5_id: row.get(offset + 0)?,
            visitor_5_name: row.get(offset + 0)?,
            visitor_5_pos: row.get(offset + 0)?,
            visitor_6_id: row.get(offset + 0)?,
            visitor_6_name: row.get(offset + 0)?,
            visitor_6_pos: row.get(offset + 0)?,
            visitor_7_id: row.get(offset + 0)?,
            visitor_7_name: row.get(offset + 0)?,
            visitor_7_pos: row.get(offset + 0)?,
            visitor_8_id: row.get(offset + 0)?,
            visitor_8_name: row.get(offset + 0)?,
            visitor_8_pos: row.get(offset + 0)?,
            visitor_9_id: row.get(offset + 0)?,
            visitor_9_name: row.get(offset + 0)?,
            visitor_9_pos: row.get(offset + 0)?,
            home_1_id: row.get(offset + 0)?,
            home_1_name: row.get(offset + 0)?,
            home_1_pos: row.get(offset + 0)?,
            home_2_id: row.get(offset + 0)?,
            home_2_name: row.get(offset + 0)?,
            home_2_pos: row.get(offset + 0)?,
            home_3_id: row.get(offset + 0)?,
            home_3_name: row.get(offset + 0)?,
            home_3_pos: row.get(offset + 0)?,
            home_4_id: row.get(offset + 0)?,
            home_4_name: row.get(offset + 0)?,
            home_4_pos: row.get(offset + 0)?,
            home_5_id: row.get(offset + 0)?,
            home_5_name: row.get(offset + 0)?,
            home_5_pos: row.get(offset + 0)?,
            home_6_id: row.get(offset + 0)?,
            home_6_name: row.get(offset + 0)?,
            home_6_pos: row.get(offset + 0)?,
            home_7_id: row.get(offset + 0)?,
            home_7_name: row.get(offset + 0)?,
            home_7_pos: row.get(offset + 0)?,
            home_8_id: row.get(offset + 0)?,
            home_8_name: row.get(offset + 0)?,
            home_8_pos: row.get(offset + 0)?,
            home_9_id: row.get(offset + 0)?,
            home_9_name: row.get(offset + 0)?,
            home_9_pos: row.get(offset + 0)?,
            additional_info: row.get(offset + 0)?,
            acquisition_info: row.get(offset + 161)?,
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
            "day_of_week",
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
            "home_plate_umpire_name",
            "home_plate_umpire_id",
            "first_base_umpire_name",
            "first_base_umpire_id",
            "second_base_umpire_name",
            "second_base_umpire_id",
            "third_base_umpire_name",
            "third_base_umpire_id",
            "left_field_umpire_name",
            "left_field_umpire_id",
            "right_field_umpire_name",
            "right_field_umpire_id",
            "visitor_manager_id",
            "visitor_manager_name",
            "home_manager_id",
            "home_manager_name",
            "winning_pitcher_name",
            "winning_pitcher_id",
            "losing_pitcher_name",
            "losing_pitcher_id",
            "saving_pitcher_name",
            "saving_pitcher_id",
            "gwrbi_player_name",
            "gwrbi_player_id",
            "visitor_starter_name",
            "visitor_starter_id",
            "home_starter_name",
            "home_starter_id",
            "visitor_1_id",
            "visitor_1_name",
            "visitor_1_pos",
            "visitor_2_id",
            "visitor_2_name",
            "visitor_2_pos",
            "visitor_3_id",
            "visitor_3_name",
            "visitor_3_pos",
            "visitor_4_id",
            "visitor_4_name",
            "visitor_4_pos",
            "visitor_5_id",
            "visitor_5_name",
            "visitor_5_pos",
            "visitor_6_id",
            "visitor_6_name",
            "visitor_6_pos",
            "visitor_7_id",
            "visitor_7_name",
            "visitor_7_pos",
            "visitor_8_id",
            "visitor_8_name",
            "visitor_8_pos",
            "visitor_9_id",
            "visitor_9_name",
            "visitor_9_pos",
            "home_1_id",
            "home_1_name",
            "home_1_pos",
            "home_2_id",
            "home_2_name",
            "home_2_pos",
            "home_3_id",
            "home_3_name",
            "home_3_pos",
            "home_4_id",
            "home_4_name",
            "home_4_pos",
            "home_5_id",
            "home_5_name",
            "home_5_pos",
            "home_6_id",
            "home_6_name",
            "home_6_pos",
            "home_7_id",
            "home_7_name",
            "home_7_pos",
            "home_8_id",
            "home_8_name",
            "home_8_pos",
            "home_9_id",
            "home_9_name",
            "home_9_pos",
            "additional_info",
            "acquisition_info",
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
