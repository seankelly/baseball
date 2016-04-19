extern crate csv;
extern crate rustc_serialize;

use std::env;
use std::collections::BTreeMap;
use std::clone::Clone;

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

#[derive(PartialEq)]
enum StreakType {
    Winning,
    Losing,
    Ties,
}

impl Streak {
    fn from_game(game: &retrosheet::TeamGameLog) -> Streak {
        let team_id = game.team.clone();

        let (game_number, team_score, other_score) = (game.team_game_number, game.score, game.opponent_score);

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

fn order_season(games: Vec<retrosheet::RetrosheetGameLog>) -> BTreeMap<String, Vec<retrosheet::TeamGameLog>> {
    let mut season = BTreeMap::new();

    for game in games {
        // Check home team first and then the visiting team.
        let (home_game, visitor_game) = game.each_team_game();
        {
            let mut team = season.entry(game.home_team).or_insert(Vec::new());
            team.push(home_game);
        }
        {
            let mut team = season.entry(game.visitor_team).or_insert(Vec::new());
            team.push(visitor_game);
        }
    }

    // Now that every team has every game it played, they need to be sorted. This is complicated
    // because the ordering is in one of two variables.
    for (_team_id, team_season) in season.iter_mut() {
        team_season.sort_by(|a, b| {
            let a_game = a.team_game_number;
            let b_game = b.team_game_number;
            a_game.cmp(&b_game)
        });
    }

    return season;
}

fn process_season_streaks(season: BTreeMap<String, Vec<retrosheet::TeamGameLog>>) -> Vec<Streak> {
    let mut streaks = Vec::new();

    for (_team_id, team_season) in &season {
        // If no games in this season, skip trying to process it.
        if team_season.len() == 0 {
            continue;
        }

        let mut season_streaks = team_season.iter().map(Streak::from_game).collect::<Vec<Streak>>();
        let mut active_streak = season_streaks.pop().expect("season_streaks shouldn't be empty.");
        loop {
            if season_streaks.len() == 0 {
                break;
            }

            let next_streak = season_streaks.pop().expect("season_streaks shouldn't be empty.");
            if active_streak.end_game + 1 == next_streak.start_game && active_streak.streak_type == next_streak.streak_type {
                active_streak.end_date = next_streak.end_date.clone();
                active_streak.end_game = next_streak.end_game.clone();
                active_streak.length += 1;
            }
            else {
                streaks.push(active_streak);
                active_streak = next_streak;
            }
        }

        streaks.push(active_streak);
    }

    return streaks;
}

fn main() {
    for file in env::args().skip(1) {
        let games = season_games(&file);
        let num_games = games.len();

        println!("{} has {} games", file, num_games);
        let team_seasons = order_season(games);
        let streaks = process_season_streaks(team_seasons);
    }
}
