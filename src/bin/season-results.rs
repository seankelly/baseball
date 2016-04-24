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

#[derive(Debug, RustcDecodable, RustcEncodable)]
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


impl FullStreak {
    fn from_streak(streak: &Streak, season: &SeasonResults) -> Self {
        FullStreak {
            team_id: streak.team_id.clone(),
            year: streak.year,
            start_date: streak.start_date.clone(),
            end_date: streak.end_date.clone(),
            start_game: streak.start_game,
            end_game: streak.end_game,
            streak_type: streak.streak_type.clone(),
            length: streak.length,
            final_wins: season.wins,
            final_losses: season.losses,
            postseason: season.postseason.clone(),
        }
    }
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

fn join_streaks_with_results(streaks: &Vec<Streak>,
                             results: &BTreeMap<(String, u16), SeasonResults>)
                            -> Vec<FullStreak> {
    let mut full_streaks = Vec::new();

    for streak in streaks {
        let key = (streak.team_id.clone(), streak.year);
        let maybe_season = results.get(&key);
        if let Some(season) = maybe_season {
            full_streaks.push(FullStreak::from_streak(&streak, &season));
        }
    }

    return full_streaks;
}

fn dump_full_streaks(full_streaks: &Vec<FullStreak>) {
    let mut writer = Writer::from_memory();
    for record in full_streaks.into_iter() {
        let result = writer.encode(record).expect("Encoded streak into CSV.");
    }
    print!("{}", writer.as_string());
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
    println!("Joining streaks with season results.");
    let full_streaks = join_streaks_with_results(&streaks, &results);
    dump_full_streaks(&full_streaks);
}
