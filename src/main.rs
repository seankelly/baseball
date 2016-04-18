extern crate csv;
extern crate rustc_serialize;

use std::env;
use std::collections::BTreeMap;
use std::clone::Clone;
use std::cmp::Ordering;

use csv::Reader;

mod retrosheet;

struct Streak {
    team_id: String,
    year: String,
    start_date: String,
    end_date: String,
    start_game: u16,
    end_game: u16,
    streak_type: StreakType,
    length: u8,
    final_wins: u16,
    final_losses: u16,
    made_postseason: bool,
}

enum StreakType {
    Winning,
    Losing,
    Ties,
}

impl Streak {
    fn from_game(game: &retrosheet::RetrosheetGameLog, home_team: bool) -> Streak {
        let team_id = if home_team {
            game.home_team.clone()
        }
        else {
            game.visitor_team.clone()
        };

        let (game_number, team_score, other_score) = if home_team {
            (game.home_team_game_number, game.home_score, game.visitor_score)
        }
        else {
            (game.visitor_team_game_number, game.visitor_score, game.home_score)
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
            start_date: game.date.clone(),
            end_date: game.date.clone(),
            start_game: game_number,
            end_game: game_number,
            streak_type: streak_type,
            length: 1,
            final_wins: 1,
            final_losses: 1,
            made_postseason: false,
        }
    }
}

fn season_games(file: &str) -> Vec<retrosheet::RetrosheetGameLog> {
    let mut csv_reader = Reader::from_file(file)
                            .expect("Couldn't open file.")
                            .has_headers(false);
    let games = csv_reader.decode().collect::<csv::Result<Vec<retrosheet::RetrosheetGameLog>>>().unwrap();
    return games;
}

fn order_season(games: Vec<retrosheet::RetrosheetGameLog>) -> BTreeMap<String, Vec<retrosheet::RetrosheetGameLog>> {
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

fn process_season_streaks(season: BTreeMap<String, Vec<retrosheet::RetrosheetGameLog>>) -> Vec<Streak> {
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
