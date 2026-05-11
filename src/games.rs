use baseball::retrosheet::game;

use rusqlite::types::{ToSql, ToSqlOutput};


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


impl ToSql for RetrosheetOption {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let value = match self {
            RetrosheetOption::None => ToSqlOutput::from(-1),
            RetrosheetOption::Unknown => ToSqlOutput::from(-1),
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
                    joined_score.push_str("(");
                    joined_score.push_str(&score.to_string());
                    joined_score.push_str(")");
                }
                None => {
                    joined_score.push_str("x");
                }
            }
        }
        joined_score
    }

    /// Convert the split linescore into a format easier to split up for the database.
    pub fn sql_linescore(&self) -> String {
        let string_score = self.linescore.iter()
            .map(|score| {
                match score {
                    Some(score) => score.to_string(),
                    None => "x".to_string(),
                }
            })
            .reduce(|mut linescore, score| {
                linescore.push_str(",");
                linescore.push_str(&score);
                linescore
            })
            .unwrap_or(String::from(""));
        string_score
    }
}


impl From<&str> for Linescore {
    fn from(retrosheet_linescore: &str) -> Self {
        Linescore::split_linescore(retrosheet_linescore)
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
