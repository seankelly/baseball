extern crate csv;
extern crate rustc_serialize;

use std::env;
use std::collections::BTreeMap;
use std::clone::Clone;
use std::cmp::Ordering;

use csv::Reader;

#[derive(Clone, Debug, RustcDecodable)]
struct RetrosheetGameLog {
    // 1
    date: String,
    number_of_game: String,
    day_of_week: String,
    visitor_team: String,
    visitor_league: String,
    visitor_team_game_number: u16,
    home_team: String,
    home_league: String,
    home_team_game_number: u16,
    // 10
    visitor_score: u16,
    home_score: u16,
    number_of_outs: Option<u8>,
    day_night: String,
    completion_info: String,
    forfeit_info: String,
    protest_info: String,
    park_id: String,
    attendance: Option<u32>,
    time_of_game: Option<u16>,
    // 20
    visitor_line_score: String,
    home_line_score: String,
    visitor_ab: Option<u8>,
    visitor_hits: Option<u8>,
    visitor_doubles: Option<u8>,
    visitor_triples: Option<u8>,
    visitor_homeruns: Option<u8>,
    visitor_rbi: Option<u8>,
    visitor_sac_hits: Option<u8>,
    visitor_sac_flies: Option<u8>,
    // 30
    visitor_hbp: Option<u8>,
    visitor_walks: Option<u8>,
    visitor_intentional_walks: Option<u8>,
    visitor_strikeouts: Option<u8>,
    visitor_stolen_bases: Option<u8>,
    visitor_caught_stealing: Option<u8>,
    visitor_gidp: Option<u8>,
    visitor_catcher_interference: Option<u8>,
    visitor_left_on_base: Option<u8>,
    visitor_pitchers_used: Option<u8>,
    // 40
    visitor_individual_earned_runs: Option<u8>,
    visitor_team_earned_runs: Option<u8>,
    visitor_wild_pitches: Option<u8>,
    visitor_balks: Option<u8>,
    visitor_putouts: Option<u8>,
    visitor_assists: Option<u8>,
    visitor_errors: Option<u8>,
    visitor_passed_balls: Option<u8>,
    visitor_double_plays: Option<u8>,
    visitor_triple_plays: Option<u8>,
    // 50
    home_ab: Option<u8>,
    home_hits: Option<u8>,
    home_doubles: Option<u8>,
    home_triples: Option<u8>,
    home_homeruns: Option<u8>,
    home_rbi: Option<u8>,
    home_sac_hits: Option<u8>,
    home_sac_flies: Option<u8>,
    home_hbp: Option<u8>,
    home_walks: Option<u8>,
    // 60
    home_intentional_walks: Option<u8>,
    home_strikeouts: Option<u8>,
    home_stolen_bases: Option<u8>,
    home_caught_stealing: Option<u8>,
    home_gidp: Option<u8>,
    home_catcher_interference: Option<u8>,
    home_left_on_base: Option<u8>,
    home_pitchers_used: Option<u8>,
    home_individual_earned_runs: Option<u8>,
    home_team_earned_runs: Option<u8>,
    // 70
    home_wild_pitches: Option<u8>,
    home_balks: Option<u8>,
    home_putouts: Option<u8>,
    home_assists: Option<u8>,
    home_errors: Option<u8>,
    home_passed_balls: Option<u8>,
    home_double_plays: Option<u8>,
    home_triple_plays: Option<u8>,
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

struct Streak {
    team_id: String,
    year: String,
    start: String,
    end: String,
    streak_type: StreakType,
    length: u8,
    final_wins: u8,
    final_losses: u8,
    made_postseason: bool,
}

enum StreakType {
    Winning,
    Losing,
    Ties,
}

impl Streak {
    fn from_game(game: &RetrosheetGameLog, home_team: bool) -> Streak {
        let team_id = if home_team {
            game.home_team.clone()
        }
        else {
            game.visitor_team.clone()
        };

        let (team_score, other_score) = if home_team {
            (game.home_score, game.visitor_score)
        }
        else {
            (game.visitor_score, game.home_score)
        };

        let streak_type = if team_score > other_score {
            StreakType::Winning
        }
        else if team_score < other_score {
            StreakType::Losing
        }
        else {
            StreakType::Ties
        };

        Streak {
            team_id: team_id,
            year: String::from(""),
            start: game.date.clone(),
            end: game.date.clone(),
            streak_type: streak_type,
            length: 1,
            final_wins: 1,
            final_losses: 1,
            made_postseason: false,
        }
    }
}

fn season_games(file: &str) -> Vec<RetrosheetGameLog> {
    let mut csv_reader = Reader::from_file(file)
                            .expect("Couldn't open file.")
                            .has_headers(false);
    let games = csv_reader.decode().collect::<csv::Result<Vec<RetrosheetGameLog>>>().unwrap();
    return games;
}

fn order_season(games: Vec<RetrosheetGameLog>) -> BTreeMap<String, Vec<RetrosheetGameLog>> {
    let mut season = BTreeMap::new();

    for game in games {
        // Check home team first and then the visiting team.
        let home_game = game.clone();
        let visitor_team = game.clone();
        {
            let mut team = season.entry(game.home_team).or_insert(Vec::new());
            team.push(home_game);
        }
        {
            let mut team = season.entry(game.visitor_team).or_insert(Vec::new());
            team.push(visitor_team);
        }
    }

    // Now that every team has every game it played, they need to be sorted. This is complicated
    // because the ordering is in one of two variables.
    for (team_id, team_season) in season.iter_mut() {
        team_season.sort_by(|a, b| {
            let a_game = if *team_id == a.home_team {
                a.home_team_game_number
            }
            else {
                a.visitor_team_game_number
            };
            let b_game = if *team_id == b.home_team {
                b.home_team_game_number
            }
            else {
                b.visitor_team_game_number
            };

            a_game.cmp(&b_game)
        });
    }

    return season;
}

fn process_season_streaks(season: BTreeMap<String, Vec<RetrosheetGameLog>>) -> Vec<Streak> {
    let streaks = Vec::new();
    return streaks;
}

fn main() {
    for file in env::args().skip(1) {
        let games = season_games(&file);
        let num_games = games.len();

        println!("{} has {} games", file, num_games);
        let team_seasons = order_season(games);
    }
}
