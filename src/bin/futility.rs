#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ops::Sub;
use std::path::Path;

use clap::{Arg, App};

use retrosheet::chadwick;

enum GameResult {
    Win,
    Loss,
    Tie,
}

#[derive(Clone, Debug, Default, Serialize)]
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
    expected: f32,
}

struct FinalTeamResults {
    opponents: HashMap<String, BTreeMap<u16, OpponentResults>>,
}

impl<'a, 'b> Sub<&'a OpponentResults> for &'b SeasonSummary {
    type Output = SeasonSummary;
    fn sub(self, opponent: &'a OpponentResults) -> SeasonSummary {
        SeasonSummary {
            wins: self.wins - opponent.actual.wins,
            losses: self.losses - opponent.actual.losses,
            ties: self.ties - opponent.actual.ties,
        }
    }
}

impl OpponentResults {
    fn copy_with_expected(&self, expected: f32) -> Self {
        OpponentResults {
            actual: self.actual.clone(),
            expected: expected,
        }
    }
}

impl SeasonSummary {
    fn winning_percentage(&self) -> f32 {
        let wins = self.wins as f32;
        let losses = self.losses as f32;
        let ties = self.ties as f32;
        wins / (wins + losses + ties)
    }
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
 *         expected: W
 */

fn process_team(teams: &mut HashMap<String, BTreeMap<u16, SeasonResults>>, team_id: &str, opponent_id: &str,
                year: u16, result: GameResult) {
    let team = teams.entry(String::from(team_id)).or_insert_with(BTreeMap::new);
    let season = team.entry(year).or_insert_with(SeasonResults::default);
    let opponent = season.opponents.entry(String::from(opponent_id)).or_insert_with(OpponentResults::default);
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

fn process_games(games: Vec<chadwick::GameLog>) -> HashMap<String, BTreeMap<u16, SeasonResults>> {
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

fn process_seasons(teams: HashMap<String, BTreeMap<u16, SeasonResults>>) -> HashMap<String,
   HashMap<String, BTreeMap<u16, OpponentResults>>>
{
    let mut teams_futility = HashMap::new();
    for (team, seasons) in teams.iter() {
        let team_futility = teams_futility.entry(team.clone())
            .or_insert_with(HashMap::new);
        for (year, season) in seasons.iter() {
            for (opponent, vs_record) in season.opponents.iter() {
                let vs_opponent = team_futility.entry(opponent.clone())
                    .or_insert_with(BTreeMap::new);
                let other_record = &season.overall - vs_record;
                let expected = other_record.winning_percentage();
                let vs_season = vs_record.copy_with_expected(expected);
                vs_opponent.insert(*year, vs_season);
            }
        }
    }

    return teams_futility;
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
            let season_games = chadwick::load_file(Path::new(game_log_path));
            games.extend(season_games);
        }
        let teams = process_games(games);
        process_seasons(teams);
    }

}

fn main() {
    run()
}
