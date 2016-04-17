extern crate csv;
extern crate rustc_serialize;

use std::env;

use csv::Reader;

#[derive(RustcDecodable)]
struct RetrosheetGameLog {
    // 1
    date: String,
    number_of_game: String,
    day_of_week: String,
    visitor_team: String,
    visitor_league: String,
    visitor_team_game_number: String,
    home_team: String,
    home_league: String,
    home_team_game_number: String,
    // 10
    visitor_score: String,
    home_score: String,
    number_of_outs: String,
    day_night: String,
    completion_info: String,
    forfeit_info: String,
    protest_info: String,
    park_id: String,
    attendance: String,
    time_of_game: String,
    // 20
    visitor_line_score: String,
    home_line_score: String,
    visitor_ab: String,
    visitor_hits: String,
    visitor_doubles: String,
    visitor_triples: String,
    visitor_homeruns: String,
    visitor_rbi: String,
    visitor_sac_hits: String,
    visitor_sac_flies: String,
    // 30
    visitor_hbp: String,
    visitor_walks: String,
    visitor_intentional_walks: String,
    visitor_strikeouts: String,
    visitor_stolen_bases: String,
    visitor_caught_stealing: String,
    visitor_gidp: String,
    visitor_catcher_interference: String,
    visitor_left_on_base: String,
    visitor_pitchers_used: String,
    // 40
    visitor_individual_earned_runs: String,
    visitor_team_earned_runs: String,
    visitor_wild_pitches: String,
    visitor_balks: String,
    visitor_putouts: String,
    visitor_assists: String,
    visitor_errors: String,
    visitor_passed_balls: String,
    visitor_double_plays: String,
    visitor_triple_plays: String,
    // 50
    home_ab: String,
    home_hits: String,
    home_doubles: String,
    home_triples: String,
    home_homeruns: String,
    home_rbi: String,
    home_sac_hits: String,
    home_sac_flies: String,
    home_hbp: String,
    home_walks: String,
    // 60
    home_intentional_walks: String,
    home_strikeouts: String,
    home_stolen_bases: String,
    home_caught_stealing: String,
    home_gidp: String,
    home_catcher_interference: String,
    home_left_on_base: String,
    home_pitchers_used: String,
    home_individual_earned_runs: String,
    home_team_earned_runs: String,
    // 70
    home_wild_pitches: String,
    home_balks: String,
    home_putouts: String,
    home_assists: String,
    home_errors: String,
    home_passed_balls: String,
    home_double_plays: String,
    home_triple_plays: String,
    home_plate_umpire_name: String,
    home_plate_umpire_id: String,
    // 80
    first_base_umpire_name: String,
    first_base_umpire_id: String,
    second_base_umpire_name: String,
    second_base_umpire_id: String,
    third_base_umpire_name: String,
    third_base_umpire_id: String,
    left_field_umpire_name: String,
    left_field_umpire_id: String,
    right_field_umpire_name: String,
    right_field_umpire_id: String,
    // 90
    visitor_manager_name: String,
    visitor_manager_id: String,
    home_manager_name: String,
    home_manager_id: String,
    winning_pitcher_name: String,
    winning_pitcher_id: String,
    losing_pitcher_name: String,
    losing_pitcher_id: String,
    saving_pitcher_name: String,
    saving_pitcher_id: String,
    // 100
    gwrbi_player_name: String,
    gwrbi_player_id: String,
    visitor_starter_name: String,
    visitor_starter_id: String,
    home_starter_name: String,
    home_starter_id: String,
    visitor_1_id: String,
    visitor_1_name: String,
    visitor_1_pos: String,
    visitor_2_id: String,
    // 110
    visitor_2_name: String,
    visitor_2_pos: String,
    visitor_3_id: String,
    visitor_3_name: String,
    visitor_3_pos: String,
    visitor_4_id: String,
    visitor_4_name: String,
    visitor_4_pos: String,
    visitor_5_id: String,
    visitor_5_name: String,
    // 120
    visitor_5_pos: String,
    visitor_6_id: String,
    visitor_6_name: String,
    visitor_6_pos: String,
    visitor_7_id: String,
    visitor_7_name: String,
    visitor_7_pos: String,
    visitor_8_id: String,
    visitor_8_name: String,
    visitor_8_pos: String,
    // 130
    visitor_9_id: String,
    visitor_9_name: String,
    visitor_9_pos: String,
    home_1_id: String,
    home_1_name: String,
    home_1_pos: String,
    home_2_id: String,
    home_2_name: String,
    home_2_pos: String,
    home_3_id: String,
    // 140
    home_3_name: String,
    home_3_pos: String,
    home_4_id: String,
    home_4_name: String,
    home_4_pos: String,
    home_5_id: String,
    home_5_name: String,
    home_5_pos: String,
    home_6_id: String,
    home_6_name: String,
    // 150
    home_6_pos: String,
    home_7_id: String,
    home_7_name: String,
    home_7_pos: String,
    home_8_id: String,
    home_8_name: String,
    home_8_pos: String,
    home_9_id: String,
    home_9_name: String,
    home_9_pos: String,
    // 160
    additional_info: String,
    acquistion_info: String,
}

fn main() {
    for file in env::args().skip(1) {
        let mut csv_file = Reader::from_file(&file)
                            .expect("Couldn't open file.")
                            .has_headers(false);
        let mut num_games = 0;
        for game in csv_file.records() {
            num_games += 1;
        }

        println!("{} has {} games", file, num_games);
    }
}
