#[macro_use]
extern crate serde_derive;

extern crate baseball;
extern crate clap;
extern crate csv;
extern crate serde;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::Path;

use clap::{Arg, App};

use baseball::retrosheet;

enum GameResult {
    Win,
    Loss,
    Tie,
}

#[derive(Debug, Default, Serialize)]
struct SeasonSummary {
    wins: u8,
    losses: u8,
    ties: u8,
}

#[derive(Debug, Default)]
struct SeasonResults {
    year: u16,
    overall: SeasonSummary,
    opponents: HashMap<String, OpponentResults>,
}

#[derive(Debug, Default)]
struct OpponentResults {
    actual: SeasonSummary,
}

struct FinalTeamResults {
    opponents: HashMap<String, BTreeMap<u16, OpponentResults>>,
}

/*
 * input data:
 * team:
 *   year:
 *     wins: X
 *     losses: Y
 *     ties: Z
 *     opponent:
 *       actual:
 *         wins: X
 *         losses: Y
 *         ties: Z
 *
 * processed data:
 * team:
 *   opponent:
 *     year:
 *       actual:
 *         wins: X
 *         losses: Y
 *         ties: Z
 */

fn process_team(teams: &mut HashMap<String, BTreeMap<u16, SeasonResults>>, team_id: &str, opponent_id: &str,
                year: u16, result: GameResult) {
    let team = teams.entry(String::from(team_id)).or_insert(BTreeMap::new());
    let season = team.entry(year).or_insert(SeasonResults::default());
    let opponent = season.opponents.entry(String::from(opponent_id)).or_insert(OpponentResults::default());
    if season.year == 0 {
        season.year = year;
    }
    match result {
        GameResult::Win => {
            season.overall.wins += 1;
            opponent.actual.wins += 1;
        }
        GameResult::Loss => {
            season.overall.losses += 1;
            opponent.actual.losses += 1;
        }
        GameResult::Tie => {
            season.overall.ties += 1;
            opponent.actual.ties += 1;
        }
    }
}

fn process_games(games: Vec<retrosheet::games::RetrosheetGameLog>) -> HashMap<String, BTreeMap<u16, SeasonResults>> {
    let mut teams = HashMap::new();
    for game in &games {
        // The date field is in the format "yyyymmdd";
        let year: u16 = game.date[..4].parse().expect("Couldn't parse year");
        let home_id = &game.home_team;
        let away_id = &game.visitor_team;
        let (home_result, away_result) = if game.home_score > game.visitor_score {
            (GameResult::Win, GameResult::Loss)
        }
        else if game.home_score < game.visitor_score {
            (GameResult::Loss, GameResult::Win)
        }
        else {
            (GameResult::Tie, GameResult::Tie)
        };
        process_team(&mut teams, &home_id, &away_id, year, home_result);
        process_team(&mut teams, &away_id, &home_id, year, away_result);
    }
    return teams;
}

fn run() {
    let matches = App::new("Team Futility")
        .about("Find stetches of team futility against another team.")
        .arg(Arg::with_name("game-log")
             .value_name("FILE")
             .help("Retrosheet game log file(s)")
             .multiple(true))
        .get_matches();

    if let Some(game_logs) = matches.values_of("game-log") {
        let mut games = Vec::new();
        for game_log_path in game_logs {
            let season_games = retrosheet::games::RetrosheetGameLog::load_game_logs(Path::new(game_log_path));
            games.extend(season_games);
        }
        process_games(games);
    }

}

fn main() {
    run()
}
