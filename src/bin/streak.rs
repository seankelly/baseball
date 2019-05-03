#[macro_use]
extern crate serde_derive;

extern crate csv;
extern crate serde;

use std::env;
use std::collections::BTreeMap;
use std::clone::Clone;
use std::io;
use std::path::Path;

use csv::WriterBuilder;

use retrosheet::chadwick;

#[derive(Debug, Deserialize, Serialize)]
struct Streak {
    team_id: String,
    year: String,
    start_date: String,
    end_date: String,
    start_game: u16,
    end_game: u16,
    streak_type: StreakType,
    length: u8,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum StreakType {
    Winning,
    Losing,
    Ties,
}

impl Streak {
    fn from_game(game: &chadwick::TeamGameLog) -> Streak {
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
            year: game.date[0..4].to_string(),
            start_date: game.date.clone(),
            end_date: game.date.clone(),
            start_game: game_number,
            end_game: game_number,
            streak_type: streak_type,
            length: 1,
        }
    }
}

fn order_season(games: Vec<chadwick::GameLog>) -> BTreeMap<String, Vec<chadwick::TeamGameLog>> {
    let mut season = BTreeMap::new();

    for game in games {
        // Check home team first and then the visiting team.
        let (home_game, visitor_game) = game.each_team_game();
        let team = season.entry(game.home_team).or_insert(Vec::new());
        team.push(home_game);
        let team = season.entry(game.visitor_team).or_insert(Vec::new());
        team.push(visitor_game);
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

fn process_season_streaks(season: BTreeMap<String, Vec<chadwick::TeamGameLog>>) -> Vec<Streak> {
    let mut streaks = Vec::new();

    for (_team_id, team_season) in &season {
        // If no games in this season, skip trying to process it.
        if team_season.len() == 0 {
            continue;
        }

        let mut season_streaks = team_season.iter().map(Streak::from_game).collect::<Vec<Streak>>();
        // Reverse the Vec because pop returns the last element but need the first element.
        season_streaks.reverse();
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

fn dump_season_streaks(streaks: &Vec<Streak>) {
    let mut writer = WriterBuilder::new().from_writer(io::stdout());
    for record in streaks.into_iter() {
        writer.serialize(record).expect("Encoded streak into CSV.");
    }
    writer.flush().expect("Failed flushing to stdout");
}

fn main() {
    for file in env::args().skip(1) {
        let path = Path::new(&file);
        let games = chadwick::load_file(&path);

        let team_seasons = order_season(games);
        let streaks = process_season_streaks(team_seasons);
        dump_season_streaks(&streaks);
    }
}
