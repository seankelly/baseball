use std::cmp::Reverse;
use std::error::Error;
use std::fmt;
use std::path;

use clap::Parser;
use rayon::prelude::*;


const OUTPUT_LIMIT: usize = 100;


#[derive(Parser)]
struct Args {
    #[arg(long = "team")]
    team: bool,

    #[arg(long = "game")]
    game: bool,

    #[arg(short = 'n', long = "limit")]
    limit: Option<usize>,

    #[arg(long = "csv")]
    csv_file: Option<path::PathBuf>,

    #[arg(value_name = "FILE")]
    game_logs: Vec<path::PathBuf>,
}

#[derive(serde::Serialize)]
struct GameEarnedRunDiff {
    year: u16,
    date: String,
    home_team: String,
    home_r: u8,
    home_er: u8,
    away_team: String,
    away_r: u8,
    away_er: u8,
}

enum UnearnedRunMode {
    Team,
    Game,
}

impl GameEarnedRunDiff {
    /// Return the total number of unearned runs in the game from both teams.
    fn total_diff(&self) -> u8 {
        let home_diff = self.home_r - self.home_er;
        let away_diff = self.away_r - self.away_er;
        home_diff + away_diff
    }

    /// Return the highest unearned run difference of both teams.
    fn team_diff(&self) -> u8 {
        let home_diff = self.home_r - self.home_er;
        let away_diff = self.away_r - self.away_er;
        if home_diff > away_diff {
            home_diff
        }
        else {
            away_diff
        }
    }
}

impl fmt::Display for GameEarnedRunDiff {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let home_diff = self.home_r - self.home_er;
        let away_diff = self.away_r - self.away_er;
        let total_diff = home_diff + away_diff;
        write!(formatter, "{}: total: {}, home ({}): {}, away ({}): {}", self.date, total_diff, self.home_team, home_diff, self.away_team, away_diff)
    }
}

fn process_gamelog(game_log_path: &path::Path) -> Vec<GameEarnedRunDiff> {
    let mut games = Vec::new();
    match parse_gamelog(game_log_path) {
        Ok(game_diffs) => {
            games = game_diffs;
        }
        Err(e) => {
            println!("failure reading {}: {}", game_log_path.display(), e);
        }
    }
    return games;
}

fn parse_gamelog(gamelog: &path::Path) -> Result<Vec<GameEarnedRunDiff>, Box<dyn Error>> {
    let mut games = Vec::with_capacity(162);
    let mut season = 0;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(&gamelog)?;
    for result in reader.deserialize() {
        if let Ok(game) = result {
            let game: baseball::retrosheet::game::GameLog = game;
            if season == 0 {
                let (year, _) = game.date.split_at(4);
                season = match year.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => {
                        0
                    },
                };
            }

            if game.home_individual_earned_runs.is_none() {
                continue;
            }
            if game.visitor_individual_earned_runs.is_none() {
                continue;
            }
            let home_er = game.home_individual_earned_runs.unwrap();
            let away_er = game.visitor_individual_earned_runs.unwrap();

            let home_team = game.home_team;
            let away_team = game.visitor_team;
            let home_r = game.visitor_score;
            let away_r = game.home_score;
            if home_er > home_r || away_er > away_r {
                println!("More earned runs than runs found on {} for {} vs {}", game.date,
                    away_team, home_team);
                continue;
            }
            let game_diff = GameEarnedRunDiff {
                year: season,
                date: game.date.clone(),
                home_team,
                home_r,
                home_er,
                away_team,
                away_r,
                away_er,
            };
            games.push(game_diff);
        }
    }

    return Ok(games);
}

fn prune(games: &mut Vec<GameEarnedRunDiff>, limit: usize, search_mode: UnearnedRunMode) {
    let sort_fn = match search_mode {
        UnearnedRunMode::Team => |g: &GameEarnedRunDiff| Reverse(g.total_diff()),
        UnearnedRunMode::Game => |g: &GameEarnedRunDiff| Reverse(g.team_diff()),
    };
    games.sort_by_key(sort_fn);
    // There's a default limit so if the limit is zero then don't prune at all.
    if limit == 0 {
        return;
    }
    let last_index = limit - 1;
    if games.len() < last_index {
        return;
    }

    match search_mode {
        UnearnedRunMode::Team => {
            let limit_minimum = games[last_index].total_diff();
            games.retain(|g: &GameEarnedRunDiff| g.total_diff() >= limit_minimum);
        },
        UnearnedRunMode::Game => {
            let limit_minimum = games[last_index].team_diff();
            games.retain(|g: &GameEarnedRunDiff| g.team_diff() >= limit_minimum);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let team = args.team;
    let game = args.game;
    let search_mode = match (team, game) {
        (false, false) => UnearnedRunMode::Team,
        (true, false) => UnearnedRunMode::Team,
        (false, true) => UnearnedRunMode::Game,
        (true, true) => {
            eprintln!("Team and game modes selected, picking only team");
            UnearnedRunMode::Team
        }
    };
    let limit = args.limit.unwrap_or(OUTPUT_LIMIT);
    let mut games = args.game_logs.par_iter()
        .flat_map(|game_log| process_gamelog(game_log))
        .collect();
    prune(&mut games, limit, search_mode);

    if let Some(csv_file) = args.csv_file {
        //dump_csv(&palindromes, &csv_file)?;
    }
    else {
        for game in &games {
            println!("{}", game);
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
