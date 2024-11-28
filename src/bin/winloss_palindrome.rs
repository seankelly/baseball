use std::cmp::Reverse;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::path;

use clap::Parser;
use rayon::prelude::*;


const PALINDROME_LIMIT: usize = 100;


#[derive(Parser)]
struct Args {
    #[arg(short = 'n', long = "limit")]
    limit: Option<usize>,

    #[arg(long = "csv")]
    csv_file: Option<path::PathBuf>,

    #[arg(value_name = "FILE")]
    game_logs: Vec<path::PathBuf>,
}

#[derive(serde::Serialize)]
struct TeamWLPalindrome {
    year: u16,
    team: String,
    length: u8,
    palindrome: String,
    game_start: u8,
    game_end: u8,
    wins: u8,
    losses: u8,
    ties: u8,
}

impl TeamWLPalindrome {
    fn from_team_season(year: u16, team: &String, team_record: &String) -> Self {
        let (start, end) = find_longest_palindrome(&team_record);
        let length = (end - start) as u8;
        let team = team.clone();
        let palindrome = String::from(&team_record[start..end]);
        // Strings are zero-indexed but games are one-indexed. Increment by one to return the
        // correct game start as a schedule would show.
        let game_start = (start + 1) as u8;
        let game_end = end as u8;
        let mut wins = 0;
        let mut losses = 0;
        let mut ties = 0;
        for game in palindrome.chars() {
            match game {
                'W' => { wins += 1 }
                'L' => { losses += 1 }
                'T' => { ties += 1 }
                _ => {}
            }
        }

        Self {
            year,
            team,
            length,
            palindrome,
            game_start,
            game_end,
            wins,
            losses,
            ties,
        }
    }

    fn len(&self) -> u8 {
        self.length
    }
}

impl fmt::Display for TeamWLPalindrome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let game_range = format!("{}-{}", self.game_start, self.game_end);
        write!(formatter, "{}: {}: {} ({: >7}): {} ({}-{}-{})", self.year, self.team, self.length,
            game_range, self.palindrome, self.wins, self.losses, self.ties)
    }
}


fn team_game_result(score: u8, other_score: u8) -> String {
    if score > other_score {
        "W".to_string()
    }
    else if score < other_score {
        "L".to_string()
    }
    else {
        "T".to_string()
    }
}

fn process_gamelog(game_log_path: &path::Path) -> Vec<TeamWLPalindrome> {
    let mut palindromes = Vec::new();
    match parse_gamelog(game_log_path) {
        Ok(team_seasons) => {
            for (season, team, record) in &team_seasons {
                palindromes.push(TeamWLPalindrome::from_team_season(*season, team, record));
            }
        }
        Err(e) => {
            println!("failure reading {}: {}", game_log_path.display(), e);
        }
    }
    return palindromes;
}

fn parse_gamelog(gamelog: &path::Path) -> Result<Vec<(u16, String, String)>, Box<dyn Error>> {
    let mut team_games = HashMap::with_capacity(30);
    let mut season = 0;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(&gamelog)?;
    let mut raw_record = csv::StringRecord::new();
    while reader.read_record(&mut raw_record)? {
        if let Ok(game) = raw_record.deserialize(None) {
            let game: retrosheet::game::GameLogRow = game;
            if season == 0 {
                let (year, _) = game.date.split_at(4);
                season = match year.parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => {
                        0
                    },
                };
            }

            let teamid = game.home_team.to_owned();
            if !team_games.contains_key(&teamid) {
                team_games.insert(teamid.clone(), Vec::with_capacity(162));
            }
            if let Some(team) = team_games.get_mut(&teamid) {
                team.push((game.home_team_game_number, team_game_result(game.home_score, game.visitor_score)));
            }

            let teamid = game.visitor_team.to_owned();
            if !team_games.contains_key(&teamid) {
                team_games.insert(teamid.clone(), Vec::with_capacity(162));
            }
            if let Some(team) = team_games.get_mut(&teamid) {
                team.push((game.visitor_team_game_number, team_game_result(game.visitor_score, game.home_score)));
            }
        }
    }

    let mut team_seasons = Vec::with_capacity(team_games.len());
    for (team, team_results) in team_games.iter_mut() {
        // Ensure each team's games for the season are in order.
        team_results.sort_by_key(|k| k.0);
        let mut team_record = String::with_capacity(team_results.len());
        for (_, result) in team_results {
            team_record.push_str(&result);
        }
        team_seasons.push((season, team.clone(), team_record));
    }

    return Ok(team_seasons);
}

/// Find the longest palindrome using Manacher's algorithm.
fn find_longest_palindrome(string: &str) -> (usize, usize) {
    let mut padded_string = Vec::with_capacity(string.len() * 2 + 2);
    padded_string.push(' ');
    for s in string.chars() {
        padded_string.push(s);
        padded_string.push(' ');
    }
    let mut palindrome_radii = vec![0; padded_string.len()];

    let mut center = 0;
    let mut radius = 0;

    while center < padded_string.len() {
        let mut start;
        let mut end;

        if center > radius {
            start = center - (radius + 1);
            end = center + (radius + 1);

            while center >= radius && end < padded_string.len() && padded_string[start] == padded_string[end] {
                if start == 0 {
                    break;
                }
                start -= 1;
                end += 1;
                radius += 1;
            }
        }

        palindrome_radii[center] = radius;
        let old_center = center;
        let old_radius = radius;

        center += 1;
        radius = 0;

        while center <= (old_center + old_radius) {
            let mirrored_center = old_center - (center - old_center);
            let max_mirrored_radius = old_center + old_radius - center;
            if palindrome_radii[mirrored_center] < max_mirrored_radius {
                palindrome_radii[center] = palindrome_radii[mirrored_center];
                center += 1;
            }
            else if palindrome_radii[center] > max_mirrored_radius {
                palindrome_radii[center] = max_mirrored_radius;
                center += 1;
            }
            else {
                radius = max_mirrored_radius;
                break;
            }
        }
    }

    let mut max_index = 0;
    let mut max_length = 0;
    for (idx, length) in palindrome_radii.iter().enumerate() {
        if *length > max_length {
            max_index = idx;
            max_length = *length;
        }
    }

    let start = (max_index / 2) - (max_length / 2);
    let end = start + max_length;

    return (start, end);
}

/// Prune the list of palindromes to a limit while allowing to go over the limit to keep any ties.
fn prune(palindromes: &mut Vec<TeamWLPalindrome>, limit: usize) {
    palindromes.sort_by_key(|k| Reverse(k.len()));
    // There's a default limit so if the limit is zero then don't prune at all.
    if limit == 0 {
        return;
    }
    let last_index = limit - 1;
    if palindromes.len() < last_index {
        return;
    }

    let limit_length = palindromes[last_index].len();
    palindromes.retain(|p| p.len() >= limit_length);
}

fn dump_csv(palindromes: &Vec<TeamWLPalindrome>, csv_file: &path::Path) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(&csv_file)?;
    for palindrome in palindromes {
        writer.serialize(palindrome)?;
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let limit = args.limit.unwrap_or(PALINDROME_LIMIT);
    let mut palindromes = args.game_logs.par_iter()
        .flat_map(|game_log| process_gamelog(game_log))
        .collect();
    prune(&mut palindromes, limit);

    if let Some(csv_file) = args.csv_file {
        dump_csv(&palindromes, &csv_file)?;
    }
    else {
        for palindrome in &palindromes {
            println!("{}", palindrome);
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
