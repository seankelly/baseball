

#[derive(Debug, Deserialize)]
pub struct GameLogRow<'a> {
    // 1
    pub date: &'a str,
    pub number_of_game: &'a str,
    pub day_of_week: &'a str,
    pub visitor_team: &'a str,
    pub visitor_league: &'a str,
    pub visitor_team_game_number: u16,
    pub home_team: &'a str,
    pub home_league: &'a str,
    pub home_team_game_number: u16,
    // 10
    pub visitor_score: u8,
    pub home_score: u8,
    pub number_of_outs: Option<u8>,
    pub day_night: &'a str,
    pub completion_info: &'a str,
    pub forfeit_info: &'a str,
    pub protest_info: &'a str,
    pub park_id: &'a str,
    pub attendance: Option<i32>,
    pub time_of_game: Option<u16>,
    // 20
    pub visitor_line_score: &'a str,
    pub home_line_score: &'a str,
    pub visitor_ab: Option<u8>,
    pub visitor_hits: Option<u8>,
    pub visitor_doubles: Option<u8>,
    pub visitor_triples: Option<u8>,
    pub visitor_homeruns: Option<u8>,
    pub visitor_rbi: Option<u8>,
    pub visitor_sac_hits: Option<u8>,
    pub visitor_sac_flies: Option<u8>,
    // 30
    pub visitor_hbp: Option<u8>,
    pub visitor_walks: Option<u8>,
    pub visitor_intentional_walks: Option<u8>,
    pub visitor_strikeouts: Option<u8>,
    pub visitor_stolen_bases: Option<u8>,
    pub visitor_caught_stealing: Option<u8>,
    pub visitor_gidp: Option<u8>,
    pub visitor_catcher_interference: Option<u8>,
    pub visitor_left_on_base: Option<u8>,
    pub visitor_pitchers_used: Option<u8>,
    // 40
    pub visitor_individual_earned_runs: Option<u8>,
    pub visitor_team_earned_runs: Option<u8>,
    pub visitor_wild_pitches: Option<u8>,
    pub visitor_balks: Option<u8>,
    pub visitor_putouts: Option<u8>,
    pub visitor_assists: Option<u8>,
    pub visitor_errors: Option<u8>,
    pub visitor_passed_balls: Option<u8>,
    pub visitor_double_plays: Option<u8>,
    pub visitor_triple_plays: Option<u8>,
    // 50
    pub home_ab: Option<u8>,
    pub home_hits: Option<u8>,
    pub home_doubles: Option<u8>,
    pub home_triples: Option<u8>,
    pub home_homeruns: Option<u8>,
    pub home_rbi: Option<u8>,
    pub home_sac_hits: Option<u8>,
    pub home_sac_flies: Option<u8>,
    pub home_hbp: Option<u8>,
    pub home_walks: Option<u8>,
    // 60
    pub home_intentional_walks: Option<u8>,
    pub home_strikeouts: Option<u8>,
    pub home_stolen_bases: Option<u8>,
    pub home_caught_stealing: Option<u8>,
    pub home_gidp: Option<u8>,
    pub home_catcher_interference: Option<u8>,
    pub home_left_on_base: Option<u8>,
    pub home_pitchers_used: Option<u8>,
    pub home_individual_earned_runs: Option<u8>,
    pub home_team_earned_runs: Option<u8>,
    // 70
    pub home_wild_pitches: Option<u8>,
    pub home_balks: Option<u8>,
    pub home_putouts: Option<u8>,
    pub home_assists: Option<u8>,
    pub home_errors: Option<u8>,
    pub home_passed_balls: Option<u8>,
    pub home_double_plays: Option<u8>,
    pub home_triple_plays: Option<u8>,
    pub home_plate_umpire_name: &'a str,
    pub home_plate_umpire_id: &'a str,
    // 80
    pub first_base_umpire_name: &'a str,
    pub first_base_umpire_id: &'a str,
    pub second_base_umpire_name: &'a str,
    pub second_base_umpire_id: &'a str,
    pub third_base_umpire_name: &'a str,
    pub third_base_umpire_id: &'a str,
    pub left_field_umpire_name: &'a str,
    pub left_field_umpire_id: &'a str,
    pub right_field_umpire_name: &'a str,
    pub right_field_umpire_id: &'a str,
    // 90
    pub visitor_manager_name: &'a str,
    pub visitor_manager_id: &'a str,
    pub home_manager_name: &'a str,
    pub home_manager_id: &'a str,
    pub winning_pitcher_name: &'a str,
    pub winning_pitcher_id: &'a str,
    pub losing_pitcher_name: &'a str,
    pub losing_pitcher_id: &'a str,
    pub saving_pitcher_name: &'a str,
    pub saving_pitcher_id: &'a str,
    // 100
    pub gwrbi_player_name: &'a str,
    pub gwrbi_player_id: &'a str,
    pub visitor_starter_name: &'a str,
    pub visitor_starter_id: &'a str,
    pub home_starter_name: &'a str,
    pub home_starter_id: &'a str,
    pub visitor_1_id: &'a str,
    pub visitor_1_name: &'a str,
    pub visitor_1_pos: &'a str,
    pub visitor_2_id: &'a str,
    // 110
    pub visitor_2_name: &'a str,
    pub visitor_2_pos: &'a str,
    pub visitor_3_id: &'a str,
    pub visitor_3_name: &'a str,
    pub visitor_3_pos: &'a str,
    pub visitor_4_id: &'a str,
    pub visitor_4_name: &'a str,
    pub visitor_4_pos: &'a str,
    pub visitor_5_id: &'a str,
    pub visitor_5_name: &'a str,
    // 120
    pub visitor_5_pos: &'a str,
    pub visitor_6_id: &'a str,
    pub visitor_6_name: &'a str,
    pub visitor_6_pos: &'a str,
    pub visitor_7_id: &'a str,
    pub visitor_7_name: &'a str,
    pub visitor_7_pos: &'a str,
    pub visitor_8_id: &'a str,
    pub visitor_8_name: &'a str,
    pub visitor_8_pos: &'a str,
    // 130
    pub visitor_9_id: &'a str,
    pub visitor_9_name: &'a str,
    pub visitor_9_pos: &'a str,
    pub home_1_id: &'a str,
    pub home_1_name: &'a str,
    pub home_1_pos: &'a str,
    pub home_2_id: &'a str,
    pub home_2_name: &'a str,
    pub home_2_pos: &'a str,
    pub home_3_id: &'a str,
    // 140
    pub home_3_name: &'a str,
    pub home_3_pos: &'a str,
    pub home_4_id: &'a str,
    pub home_4_name: &'a str,
    pub home_4_pos: &'a str,
    pub home_5_id: &'a str,
    pub home_5_name: &'a str,
    pub home_5_pos: &'a str,
    pub home_6_id: &'a str,
    pub home_6_name: &'a str,
    // 150
    pub home_6_pos: &'a str,
    pub home_7_id: &'a str,
    pub home_7_name: &'a str,
    pub home_7_pos: &'a str,
    pub home_8_id: &'a str,
    pub home_8_name: &'a str,
    pub home_8_pos: &'a str,
    pub home_9_id: &'a str,
    pub home_9_name: &'a str,
    pub home_9_pos: &'a str,
    // 160
    pub additional_info: &'a str,
    pub acquistion_info: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GameLog {
    // 1
    pub date: String,
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
    pub visitor_line_score: String,
    pub home_line_score: String,
    pub visitor_ab: Option<u8>,
    pub visitor_hits: Option<u8>,
    pub visitor_doubles: Option<u8>,
    pub visitor_triples: Option<u8>,
    pub visitor_homeruns: Option<u8>,
    pub visitor_rbi: Option<u8>,
    pub visitor_sac_hits: Option<u8>,
    pub visitor_sac_flies: Option<u8>,
    // 30
    pub visitor_hbp: Option<u8>,
    pub visitor_walks: Option<u8>,
    pub visitor_intentional_walks: Option<u8>,
    pub visitor_strikeouts: Option<u8>,
    pub visitor_stolen_bases: Option<u8>,
    pub visitor_caught_stealing: Option<u8>,
    pub visitor_gidp: Option<u8>,
    pub visitor_catcher_interference: Option<u8>,
    pub visitor_left_on_base: Option<u8>,
    pub visitor_pitchers_used: Option<u8>,
    // 40
    pub visitor_individual_earned_runs: Option<u8>,
    pub visitor_team_earned_runs: Option<u8>,
    pub visitor_wild_pitches: Option<u8>,
    pub visitor_balks: Option<u8>,
    pub visitor_putouts: Option<u8>,
    pub visitor_assists: Option<u8>,
    pub visitor_errors: Option<u8>,
    pub visitor_passed_balls: Option<u8>,
    pub visitor_double_plays: Option<u8>,
    pub visitor_triple_plays: Option<u8>,
    // 50
    pub home_ab: Option<u8>,
    pub home_hits: Option<u8>,
    pub home_doubles: Option<u8>,
    pub home_triples: Option<u8>,
    pub home_homeruns: Option<u8>,
    pub home_rbi: Option<u8>,
    pub home_sac_hits: Option<u8>,
    pub home_sac_flies: Option<u8>,
    pub home_hbp: Option<u8>,
    pub home_walks: Option<u8>,
    // 60
    pub home_intentional_walks: Option<u8>,
    pub home_strikeouts: Option<u8>,
    pub home_stolen_bases: Option<u8>,
    pub home_caught_stealing: Option<u8>,
    pub home_gidp: Option<u8>,
    pub home_catcher_interference: Option<u8>,
    pub home_left_on_base: Option<u8>,
    pub home_pitchers_used: Option<u8>,
    pub home_individual_earned_runs: Option<u8>,
    pub home_team_earned_runs: Option<u8>,
    // 70
    pub home_wild_pitches: Option<u8>,
    pub home_balks: Option<u8>,
    pub home_putouts: Option<u8>,
    pub home_assists: Option<u8>,
    pub home_errors: Option<u8>,
    pub home_passed_balls: Option<u8>,
    pub home_double_plays: Option<u8>,
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
    pub visitor_manager_name: String,
    pub visitor_manager_id: String,
    pub home_manager_name: String,
    pub home_manager_id: String,
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
    pub acquistion_info: String,
}


#[derive(Clone, Debug, Deserialize)]
pub struct TeamGameLog {
    // 1
    pub date: String,
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
    pub line_score: String,
    pub opponent_line_score: String,
    pub ab: Option<u8>,
    pub hits: Option<u8>,
    pub doubles: Option<u8>,
    pub triples: Option<u8>,
    pub homeruns: Option<u8>,
    pub rbi: Option<u8>,
    pub sac_hits: Option<u8>,
    pub sac_flies: Option<u8>,
    // 30
    pub hbp: Option<u8>,
    pub walks: Option<u8>,
    pub intentional_walks: Option<u8>,
    pub strikeouts: Option<u8>,
    pub stolen_bases: Option<u8>,
    pub caught_stealing: Option<u8>,
    pub gidp: Option<u8>,
    pub catcher_interference: Option<u8>,
    pub left_on_base: Option<u8>,
    pub pitchers_used: Option<u8>,
    // 40
    pub individual_earned_runs: Option<u8>,
    pub team_earned_runs: Option<u8>,
    pub wild_pitches: Option<u8>,
    pub balks: Option<u8>,
    pub putouts: Option<u8>,
    pub assists: Option<u8>,
    pub errors: Option<u8>,
    pub passed_balls: Option<u8>,
    pub double_plays: Option<u8>,
    pub triple_plays: Option<u8>,
    // 50
    pub opponent_ab: Option<u8>,
    pub opponent_hits: Option<u8>,
    pub opponent_doubles: Option<u8>,
    pub opponent_triples: Option<u8>,
    pub opponent_homeruns: Option<u8>,
    pub opponent_rbi: Option<u8>,
    pub opponent_sac_hits: Option<u8>,
    pub opponent_sac_flies: Option<u8>,
    pub opponent_hbp: Option<u8>,
    pub opponent_walks: Option<u8>,
    // 60
    pub opponent_intentional_walks: Option<u8>,
    pub opponent_strikeouts: Option<u8>,
    pub opponent_stolen_bases: Option<u8>,
    pub opponent_caught_stealing: Option<u8>,
    pub opponent_gidp: Option<u8>,
    pub opponent_catcher_interference: Option<u8>,
    pub opponent_left_on_base: Option<u8>,
    pub opponent_pitchers_used: Option<u8>,
    pub opponent_individual_earned_runs: Option<u8>,
    pub opponent_team_earned_runs: Option<u8>,
    // 70
    pub opponent_wild_pitches: Option<u8>,
    pub opponent_balks: Option<u8>,
    pub opponent_putouts: Option<u8>,
    pub opponent_assists: Option<u8>,
    pub opponent_errors: Option<u8>,
    pub opponent_passed_balls: Option<u8>,
    pub opponent_double_plays: Option<u8>,
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
    pub manager_name: String,
    pub manager_id: String,
    pub opponent_manager_name: String,
    pub opponent_manager_id: String,
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
    pub acquistion_info: String,
}


impl GameLog {
    pub fn each_team_game(&self) -> (TeamGameLog, TeamGameLog) {
        (TeamGameLog::from_home_team(&self),
         TeamGameLog::from_visitor_team(&self))
    }
}

impl TeamGameLog {
    fn from_home_team(game: &GameLog) -> TeamGameLog {
        TeamGameLog {
            // 1
            date: game.date.clone(),
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
            doubles: game.home_doubles,
            triples: game.home_triples,
            homeruns: game.home_homeruns,
            rbi: game.home_rbi,
            sac_hits: game.home_sac_hits,
            sac_flies: game.home_sac_flies,
            // 30
            hbp: game.home_hbp,
            walks: game.home_walks,
            intentional_walks: game.home_intentional_walks,
            strikeouts: game.home_strikeouts,
            stolen_bases: game.home_stolen_bases,
            caught_stealing: game.home_caught_stealing,
            gidp: game.home_gidp,
            catcher_interference: game.home_catcher_interference,
            left_on_base: game.home_left_on_base,
            pitchers_used: game.home_pitchers_used,
            // 40
            individual_earned_runs: game.home_individual_earned_runs,
            team_earned_runs: game.home_team_earned_runs,
            wild_pitches: game.home_wild_pitches,
            balks: game.home_balks,
            putouts: game.home_putouts,
            assists: game.home_assists,
            errors: game.home_errors,
            passed_balls: game.home_passed_balls,
            double_plays: game.home_double_plays,
            triple_plays: game.home_triple_plays,
            // 50
            opponent_ab: game.visitor_ab,
            opponent_hits: game.visitor_hits,
            opponent_doubles: game.visitor_doubles,
            opponent_triples: game.visitor_triples,
            opponent_homeruns: game.visitor_homeruns,
            opponent_rbi: game.visitor_rbi,
            opponent_sac_hits: game.visitor_sac_hits,
            opponent_sac_flies: game.visitor_sac_flies,
            opponent_hbp: game.visitor_hbp,
            opponent_walks: game.visitor_walks,
            // 60
            opponent_intentional_walks: game.visitor_intentional_walks,
            opponent_strikeouts: game.visitor_strikeouts,
            opponent_stolen_bases: game.visitor_stolen_bases,
            opponent_caught_stealing: game.visitor_caught_stealing,
            opponent_gidp: game.visitor_gidp,
            opponent_catcher_interference: game.visitor_catcher_interference,
            opponent_left_on_base: game.visitor_left_on_base,
            opponent_pitchers_used: game.visitor_pitchers_used,
            opponent_individual_earned_runs: game.visitor_individual_earned_runs,
            opponent_team_earned_runs: game.visitor_team_earned_runs,
            // 70
            opponent_wild_pitches: game.visitor_wild_pitches,
            opponent_balks: game.visitor_balks,
            opponent_putouts: game.visitor_putouts,
            opponent_assists: game.visitor_assists,
            opponent_errors: game.visitor_errors,
            opponent_passed_balls: game.visitor_passed_balls,
            opponent_double_plays: game.visitor_double_plays,
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
            acquistion_info: game.acquistion_info.clone(),
        }
    }

    fn from_visitor_team(game: &GameLog) -> TeamGameLog {
        TeamGameLog {
            // 1
            date: game.date.clone(),
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
            doubles: game.visitor_doubles,
            triples: game.visitor_triples,
            homeruns: game.visitor_homeruns,
            rbi: game.visitor_rbi,
            sac_hits: game.visitor_sac_hits,
            sac_flies: game.visitor_sac_flies,
            // 30
            hbp: game.visitor_hbp,
            walks: game.visitor_walks,
            intentional_walks: game.visitor_intentional_walks,
            strikeouts: game.visitor_strikeouts,
            stolen_bases: game.visitor_stolen_bases,
            caught_stealing: game.visitor_caught_stealing,
            gidp: game.visitor_gidp,
            catcher_interference: game.visitor_catcher_interference,
            left_on_base: game.visitor_left_on_base,
            pitchers_used: game.visitor_pitchers_used,
            // 40
            individual_earned_runs: game.visitor_individual_earned_runs,
            team_earned_runs: game.visitor_team_earned_runs,
            wild_pitches: game.visitor_wild_pitches,
            balks: game.visitor_balks,
            putouts: game.visitor_putouts,
            assists: game.visitor_assists,
            errors: game.visitor_errors,
            passed_balls: game.visitor_passed_balls,
            double_plays: game.visitor_double_plays,
            triple_plays: game.visitor_triple_plays,
            // 50
            opponent_ab: game.home_ab,
            opponent_hits: game.home_hits,
            opponent_doubles: game.home_doubles,
            opponent_triples: game.home_triples,
            opponent_homeruns: game.home_homeruns,
            opponent_rbi: game.home_rbi,
            opponent_sac_hits: game.home_sac_hits,
            opponent_sac_flies: game.home_sac_flies,
            opponent_hbp: game.home_hbp,
            opponent_walks: game.home_walks,
            // 60
            opponent_intentional_walks: game.home_intentional_walks,
            opponent_strikeouts: game.home_strikeouts,
            opponent_stolen_bases: game.home_stolen_bases,
            opponent_caught_stealing: game.home_caught_stealing,
            opponent_gidp: game.home_gidp,
            opponent_catcher_interference: game.home_catcher_interference,
            opponent_left_on_base: game.home_left_on_base,
            opponent_pitchers_used: game.home_pitchers_used,
            opponent_individual_earned_runs: game.home_individual_earned_runs,
            opponent_team_earned_runs: game.home_team_earned_runs,
            // 70
            opponent_wild_pitches: game.home_wild_pitches,
            opponent_balks: game.home_balks,
            opponent_putouts: game.home_putouts,
            opponent_assists: game.home_assists,
            opponent_errors: game.home_errors,
            opponent_passed_balls: game.home_passed_balls,
            opponent_double_plays: game.home_double_plays,
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
            acquistion_info: game.acquistion_info.clone(),
        }
    }
}
