extern crate csv;
extern crate rustc_serialize;

use std::env;
use std::collections::BTreeMap;
use std::clone::Clone;

use csv::Reader;
use csv::Writer;

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Streak {
    team_id: String,
    year: u16,
    start_date: String,
    end_date: String,
    start_game: u16,
    end_game: u16,
    streak_type: String,
    length: u8,
}

struct FullStreak {
    team_id: String,
    year: u16,
    start_date: String,
    end_date: String,
    start_game: u16,
    end_game: u16,
    streak_type: String,
    length: u8,
    final_wins: u16,
    final_losses: u16,
    postseason: String,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct SeasonResults {
    rank: u16,
    year: u16,
    retroid: String,
    league: String,
    games: u16,
    wins: u16,
    losses: u16,
    ties: u16,
    win_loss: f32,
    pythag_win_loss: f32,
    finish: String,
    games_back: Option<f32>,
    postseason: String,
    runs: u16,
    runs_against: u16,
    attendance: String,
    batter_age: f32,
    pitcher_age: f32,
    number_batters: u8,
    number_pitchers: u8,
    top_player: String,
    managers: String,
}

#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
enum StreakType {
    Winning,
    Losing,
    Ties,
}


fn load_streaks(streaks_file: &str) -> Vec<Streak> {
    let mut csv_reader = Reader::from_file(streaks_file)
                            .expect("Couldn't open file.")
                            .has_headers(false);
    let streaks = csv_reader.decode().collect::<csv::Result<Vec<Streak>>>().unwrap();
    return streaks;
}

fn load_results(results_file: &str) -> BTreeMap<(String, u16), SeasonResults> {
    let mut csv_reader = Reader::from_file(results_file)
                            .expect("Couldn't open file.")
                            .has_headers(false);
    let results = csv_reader.decode();
    let mut results_map = BTreeMap::new();
    for row in results {
        let record: SeasonResults = row.unwrap();
        let key = (record.retroid.clone(), record.year);
        results_map.insert(key, record);
    }

    return results_map;
}

fn join_streaks_with_results(streaks: &Vec<Streak>) -> Vec<FullStreak> {
    return Vec::new();
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Arguments: streaks.csv season-results.csv");
        return;
    }

    println!("Loading streaks.");
    let streaks = load_streaks(&args[1]);
    println!("Loading season results.");
    let results = load_results(&args[2]);
}
