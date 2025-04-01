use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::path;

use baseball::lahman;

use cel_interpreter::{Context, Program, Value};
use clap::{Args, Parser, Subcommand, ValueEnum};
use csv::Writer;
use serde::de::DeserializeOwned;


#[derive(Parser)]
struct LahmanArgs {
    #[arg(long)]
    career: bool,

    #[arg(long)]
    season: bool,

    #[arg(short = 'n', long)]
    limit: Option<usize>,

    #[arg(long, value_name = "PROGRAM")]
    filter: Option<String>,

    #[arg(long, value_name = "PROGRAM")]
    sort_key: Option<String>,

    #[arg(long, value_name = "ORDER")]
    sort_order: Option<SortOrder>,

    #[arg(long = "csv")]
    csv_file: Option<path::PathBuf>,

    #[command(subcommand)]
    lahman: LahmanType,
}


#[derive(Subcommand)]
enum LahmanType {
    Batting(LahmanFile),
    Pitching(LahmanFile),
}


#[derive(Args)]
struct LahmanFile {
    #[arg(value_name = "FILE")]
    lahman_file: path::PathBuf,
}


trait Career {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>>;
}


enum LahmanSeasons {
    Batting(Vec<lahman::batting::Batting>),
    Pitching(Vec<lahman::pitching::Pitching>),
}


#[derive(Default, serde::Serialize)]
struct BattingCareer {
    player_id: String,
    games: u16,
    games_batting: u16,
    atbats: u16,
    runs: u16,
    hits: u16,
    doubles: u16,
    triples: u16,
    home_runs: u16,
    runs_batted_in: u16,
    stolen_bases: u16,
    caught_stealing: u16,
    walks: u16,
    strikeouts: u16,
    intentional_walks: u16,
    hit_by_pitches: u16,
    sacrifice_hits: u16,
    sacrifice_flies: u16,
    gidp: u16,
    games_old: u16,
}


#[derive(Default, serde::Serialize)]
struct PitchingCareer {
    player_id: String,
    wins: u16,
    losses: u16,
    games: u16,
    games_started: u16,
    complete_games: u16,
    shutouts: u8,
    saves: u16,
    ip_outs: u16,
    hits: u16,
    earned_runs: u16,
    home_runs: u16,
    walks: u16,
    strikeouts: u16,
    era: f32,
    intentional_walks: u16,
    wild_pitches: u16,
    hit_by_pitches: u16,
    balks: u8,
    batters_faced: u16,
    games_finished: u16,
    runs: u16,
    sacrifice_hits: u16,
    sacrifice_flies: u16,
    gidp: u16,
}


#[derive(Clone, ValueEnum)]
enum SortOrder {
    Asc,
    Desc,
}


impl BattingCareer {
    fn add_season(&mut self, season: &lahman::batting::Batting) {
        let season_games: u16 = season.games.into();
        self.games += season_games;
        self.atbats += season.atbats;
        let season_runs: u16 = season.runs.into();
        self.runs += season_runs;
        self.hits += season.hits;
        let season_doubles: u16 = season.doubles.into();
        self.doubles += season_doubles;
        let season_triples: u16 = season.triples.into();
        self.triples += season_triples;
        let season_home_runs: u16 = season.home_runs.into();
        self.home_runs += season_home_runs;
        let season_runs_batted_in: u16 = season.runs_batted_in.unwrap_or(0).into();
        self.runs_batted_in += season_runs_batted_in;
        let season_stolen_bases: u16 = season.stolen_bases.unwrap_or(0).into();
        self.stolen_bases += season_stolen_bases;
        let season_caught_stealing: u16 = season.caught_stealing.unwrap_or(0).into();
        self.caught_stealing += season_caught_stealing;
        let season_walks: u16 = season.walks.into();
        self.walks += season_walks;
        let season_strikeouts: u16 = season.strikeouts.unwrap_or(0).into();
        self.strikeouts += season_strikeouts;
        let season_intentional_walks: u16 = season.intentional_walks.unwrap_or(0).into();
        self.intentional_walks += season_intentional_walks;
        let season_hit_by_pitches: u16 = season.hit_by_pitches.unwrap_or(0).into();
        self.hit_by_pitches += season_hit_by_pitches;
        let season_sacrifice_hits: u16 = season.sacrifice_hits.unwrap_or(0).into();
        self.sacrifice_hits += season_sacrifice_hits;
        let season_sacrifice_flies: u16 = season.sacrifice_flies.unwrap_or(0).into();
        self.sacrifice_flies += season_sacrifice_flies;
        let season_gidp: u16 = season.gidp.unwrap_or(0).into();
        self.gidp += season_gidp;
    }
}


impl Career for BattingCareer {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>> {
        context.add_variable("G", self.games)?;
        //context.add_variable("GB", self.games_batting)?;
        context.add_variable("AB", self.atbats)?;
        context.add_variable("R", self.runs)?;
        context.add_variable("H", self.hits)?;
        context.add_variable("H2", self.doubles)?;
        context.add_variable("H3", self.triples)?;
        context.add_variable("HR", self.home_runs)?;
        context.add_variable("RBI", self.runs_batted_in)?;
        context.add_variable("SB", self.stolen_bases)?;
        context.add_variable("CS", self.caught_stealing)?;
        context.add_variable("BB", self.walks)?;
        context.add_variable("SO", self.strikeouts)?;
        context.add_variable("IBB", self.intentional_walks)?;
        context.add_variable("HBP", self.hit_by_pitches)?;
        context.add_variable("SH", self.sacrifice_hits)?;
        context.add_variable("SF", self.sacrifice_flies)?;
        context.add_variable("GIDP", self.gidp)?;
        Ok(())
    }
}


impl PitchingCareer {
    fn add_season(&mut self, season: &lahman::pitching::Pitching) {
        let season_wins: u16 = season.wins.into();
        self.wins += season_wins;
        let season_losses: u16 = season.losses.into();
        self.losses += season_losses;
        let season_games: u16 = season.games.into();
        self.games += season_games;
        let season_games_started: u16 = season.games_started.into();
        self.games_started += season_games_started;
        let season_complete_games: u16 = season.complete_games.into();
        self.complete_games += season_complete_games;
        self.shutouts += season.shutouts;
        let season_saves: u16 = season.saves.into();
        self.saves += season_saves;
        let season_ip_outs: u16 = season.ip_outs.into();
        self.ip_outs += season_ip_outs;
        self.hits += season.hits;
        let season_earned_runs: u16 = season.earned_runs.into();
        self.earned_runs += season_earned_runs;
        let season_home_runs: u16 = season.home_runs.into();
        self.home_runs += season_home_runs;
        let season_walks: u16 = season.walks.into();
        self.walks += season_walks;
        let season_strikeouts: u16 = season.strikeouts.into();
        self.strikeouts += season_strikeouts;
        let season_intentional_walks: u16 = season.intentional_walks.unwrap_or(0).into();
        self.intentional_walks += season_intentional_walks;
        let season_wild_pitches: u16 = season.wild_pitches.into();
        self.wild_pitches += season_wild_pitches;
        let season_hit_by_pitches: u16 = season.hit_by_pitches.unwrap_or(0).into();
        self.hit_by_pitches += season_hit_by_pitches;
        self.balks += season.balks;
        self.batters_faced += season.batters_faced.unwrap_or(0);
        let season_games_finished: u16 = season.games_finished.into();
        self.games_finished += season_games_finished;
        let season_runs: u16 = season.runs.into();
        self.runs += season_runs;
        let season_sacrifice_hits: u16 = season.sacrifice_hits.unwrap_or(0).into();
        self.sacrifice_hits += season_sacrifice_hits;
        let season_sacrifice_flies: u16 = season.sacrifice_flies.unwrap_or(0).into();
        self.sacrifice_flies += season_sacrifice_flies;
        let season_gidp: u16 = season.gidp.unwrap_or(0).into();
        self.gidp += season_gidp;

        let er = self.earned_runs as f32;
        let ipouts = self.ip_outs as f32;
        self.era = er * 27.0 / ipouts;
    }
}


impl Career for PitchingCareer {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>> {
        context.add_variable("W", self.wins)?;
        context.add_variable("L", self.losses)?;
        context.add_variable("G", self.games)?;
        context.add_variable("GS", self.games_started)?;
        context.add_variable("CG", self.complete_games)?;
        context.add_variable("SHO", self.shutouts)?;
        context.add_variable("SV", self.saves)?;
        context.add_variable("IPOuts", self.ip_outs)?;
        context.add_variable("H", self.hits)?;
        context.add_variable("ER", self.earned_runs)?;
        context.add_variable("HR", self.home_runs)?;
        context.add_variable("BB", self.walks)?;
        context.add_variable("SO", self.strikeouts)?;
        context.add_variable("ERA", self.era)?;
        context.add_variable("IBB", self.intentional_walks)?;
        context.add_variable("WP", self.wild_pitches)?;
        context.add_variable("HBP", self.hit_by_pitches)?;
        context.add_variable("BK", self.balks)?;
        context.add_variable("BFP", self.batters_faced)?;
        context.add_variable("GF", self.games_finished)?;
        context.add_variable("R", self.runs)?;
        context.add_variable("SH", self.sacrifice_hits)?;
        context.add_variable("SF", self.sacrifice_flies)?;
        context.add_variable("GIDP", self.gidp)?;
        Ok(())
    }
}


fn load_lahman_file<T: DeserializeOwned>(lahman_file: &path::Path) -> Result<Vec<T>, Box<dyn Error>>
{
    let mut seasons = Vec::new();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(&lahman_file)?;
    for result in reader.deserialize() {
        if let Ok(season) = result {
            let season: T = season;
            seasons.push(season);
        }
        /*
        match result {
            Ok(season) => {
                let season: T = season;
                seasons.push(season);
            }
            Err(e) => {
                eprintln!("error: {}", e);
                break;
            }
        }
        */
    }

    Ok(seasons)
}


fn collect_batting_careers(seasons: &[lahman::batting::Batting]) -> Vec<BattingCareer> {
    let mut career = HashMap::new();

    for season in seasons {
        let player_id = season.player_id.clone();
        if !career.contains_key(&player_id) {
            let player_career = BattingCareer {
                player_id: player_id.clone(),
                ..Default::default()
            };
            career.insert(player_id.clone(), player_career);
        }
        if let Some(player_career) = career.get_mut(&player_id) {
            player_career.add_season(season);
        }
    }

    return career.into_values().collect();
}


fn collect_pitching_careers(seasons: &[lahman::pitching::Pitching]) -> Vec<PitchingCareer> {
    let mut career = HashMap::new();

    for season in seasons {
        let player_id = season.player_id.clone();
        if !career.contains_key(&player_id) {
            let player_career = PitchingCareer {
                player_id: player_id.clone(),
                ..Default::default()
            };
            career.insert(player_id.clone(), player_career);
        }
        if let Some(player_career) = career.get_mut(&player_id) {
            player_career.add_season(season);
        }
    }

    return career.into_values().collect();
}


fn filter_option<T: Career>(career: &T, context: &Context, program: &Program) -> bool {
    let mut player_ctx = context.new_inner_scope();
    if let Err(_) = career.add_cel_variables(&mut player_ctx) {
        return false;
    }
    match program.execute(&player_ctx) {
        Ok(Value::Bool(true)) => true,
        Ok(_) => false,
        Err(error) => {
            eprintln!("error evaluating: {error}");
            false
        }
    }
}


fn process_careers_generic<T: Career + serde::Serialize>(args: &LahmanArgs, context: &Context, careers: &mut Vec<T>) -> Result<String, Box<dyn Error>> {
    if let Some(filter_prog) = &args.filter {
        let program = Program::compile(filter_prog)?;
        careers.retain(|career| filter_option(career, &context, &program) );
    }

    if let Some(sort_prog) = &args.sort_key {
        let program = Program::compile(&sort_prog)?;
        let sort_order = args.sort_order.as_ref().unwrap_or(&SortOrder::Asc);
        careers.sort_unstable_by(|a, b| {
            let a_res = sort_key(a, &context, &program);
            let b_res = sort_key(b, &context, &program);
            match sort_order {
                SortOrder::Asc => { a_res.total_cmp(&b_res) }
                SortOrder::Desc => { b_res.total_cmp(&a_res) }
            }
        });
    }

    let mut wtr = Writer::from_writer(vec![]);
    let limit = args.limit.unwrap_or(careers.len());
    for career in careers.iter().take(limit) {
        wtr.serialize(career)?;
    }

    Ok(String::from_utf8(wtr.into_inner()?)?)
}


fn process_careers(args: &LahmanArgs, seasons: &LahmanSeasons) -> Result<(), Box<dyn Error>> {
    let mut context = Context::default();
    context.add_function("abs", |a: f64| a.abs());

    let results = match seasons {
        LahmanSeasons::Batting(s) => {
            let mut careers = collect_batting_careers(s);
            process_careers_generic(args, &context, &mut careers)?
        }
        LahmanSeasons::Pitching(s) => {
            let mut careers = collect_pitching_careers(s);
            process_careers_generic(args, &context, &mut careers)?
        }
    };

    print!("{}", results);

    Ok(())
}


fn sort_key<T: Career>(career: &T, context: &Context, program: &Program) -> f64 {
    let mut player_ctx = context.new_inner_scope();
    if let Err(_) = career.add_cel_variables(&mut player_ctx) {
        return f64::NEG_INFINITY;
    }

    let result = program.execute(&player_ctx);
    let f_result = match result {
        Ok(Value::Int(i)) => { i as f64 }
        Ok(Value::UInt(u)) => { u as f64 }
        Ok(Value::Float(f)) => { f }
        _ => f64::INFINITY
    };
    f_result
}


fn run() -> Result<(), Box<dyn Error>> {
    let args = LahmanArgs::parse();

    let seasons = match &args.lahman {
        LahmanType::Batting(path) => {
            LahmanSeasons::Batting(load_lahman_file(&path.lahman_file)?)
        }
        LahmanType::Pitching(path) => {
            LahmanSeasons::Pitching(load_lahman_file(&path.lahman_file)?)
        }
    };

    let mut context = Context::default();
    context.add_function("abs", |a: f64| a.abs());

    if args.career {
        process_careers(&args, &seasons)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
