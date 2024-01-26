use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::fmt;

use clap::{Arg, App};


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
        let wins = 0;
        let losses = 0;
        let ties = 0;

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
        write!(formatter, "{}: {}: {} {}-{}: {}", self.year, self.team, self.length,
            self.game_start, self.game_end, self.palindrome)
    }
}


fn team_game_result(score: u16, other_score: u16) -> String {
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

fn parse_gamelog(gamelog: &Path) -> Result<Vec<(u16, String, String)>, Box<dyn Error>> {
    let mut team_games = HashMap::with_capacity(30);
    let mut season = 0;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(&gamelog)?;
    for result in reader.deserialize() {
        let game: retrosheet::game::GameLog = result?;
        if season == 0 {
            let (year, _) = game.date.split_at(4);
            season = match year.parse::<u16>() {
                Ok(n) => n,
                Err(_) => {
                    0
                },
            };
        }

        let teamid = game.home_team;
        if !team_games.contains_key(&teamid) {
            team_games.insert(teamid.clone(), Vec::with_capacity(162));
        }
        if let Some(team) = team_games.get_mut(&teamid) {
            team.push((game.home_team_game_number, team_game_result(game.home_score, game.visitor_score)));
        }

        let teamid = game.visitor_team;
        if !team_games.contains_key(&teamid) {
            team_games.insert(teamid.clone(), Vec::with_capacity(162));
        }
        if let Some(team) = team_games.get_mut(&teamid) {
            team.push((game.visitor_team_game_number, team_game_result(game.visitor_score, game.home_score)));
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

fn run() {
    let matches = App::new("Team win/loss palindrome")
        .about("Find longest win/loss/tie palindromes.")
        .arg(Arg::with_name("game-log")
             .value_name("FILE")
             .help("Retrosheet game log file(s)")
             .multiple(true))
        .get_matches();

    if let Some(game_logs) = matches.values_of("game-log") {
        //let mut games = Vec::new();
        for game_log_path in game_logs {
            match parse_gamelog(Path::new(game_log_path)) {
                Ok(team_seasons) => {
                    let mut palindromes = Vec::new();
                    for (season, team, record) in &team_seasons {
                        palindromes.push(TeamWLPalindrome::from_team_season(*season, team, record));
                    }
                    palindromes.sort_by_key(|k| k.len());
                    for palindrome in palindromes.iter().rev() {
                        println!("{}", palindrome);
                    }
                }
                Err(e) => {
                    println!("failure: {}", e);
                }
            }
        }
    }

}

fn main() {
    run()
}
