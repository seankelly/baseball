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

macro_rules! u8_into_u16 {
    ($attr:expr) => {
        {
            let v: u16 = $attr.into();
            v
        }
    };
}

macro_rules! u8_option_into_u16 {
    ($attr:expr) => {
        {
            let v: u16 = $attr.unwrap_or(0).into();
            v
        }
    };
}


impl BattingCareer {
    fn add_season(&mut self, season: &lahman::batting::Batting) {
        self.games += u8_into_u16!(season.games);
        self.atbats += season.atbats;
        self.runs += u8_into_u16!(season.runs);
        self.hits += season.hits;
        self.doubles += u8_into_u16!(season.doubles);
        self.triples += u8_into_u16!(season.triples);
        self.home_runs += u8_into_u16!(season.home_runs);
        self.runs_batted_in += u8_option_into_u16!(season.runs_batted_in);
        self.stolen_bases += u8_option_into_u16!(season.stolen_bases);
        self.caught_stealing += u8_option_into_u16!(season.caught_stealing);
        self.walks += u8_into_u16!(season.walks);
        self.strikeouts += u8_option_into_u16!(season.strikeouts);
        self.intentional_walks += u8_option_into_u16!(season.intentional_walks);
        self.hit_by_pitches += u8_option_into_u16!(season.hit_by_pitches);
        self.sacrifice_hits += u8_option_into_u16!(season.sacrifice_hits);
        self.sacrifice_flies += u8_option_into_u16!(season.sacrifice_flies);
        self.gidp += u8_option_into_u16!(season.gidp);
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
        self.wins += u8_into_u16!(season.wins);
        self.losses += u8_into_u16!(season.losses);
        self.games += u8_into_u16!(season.games);
        self.games_started += u8_into_u16!(season.games_started);
        self.complete_games += u8_into_u16!(season.complete_games);
        self.shutouts += season.shutouts;
        self.saves += u8_into_u16!(season.saves);
        self.ip_outs += u8_into_u16!(season.ip_outs);
        self.hits += season.hits;
        self.earned_runs += u8_into_u16!(season.earned_runs);
        self.home_runs += u8_into_u16!(season.home_runs);
        self.walks += u8_into_u16!(season.walks);
        self.strikeouts += u8_into_u16!(season.strikeouts);
        self.intentional_walks += u8_option_into_u16!(season.intentional_walks);
        self.wild_pitches += u8_into_u16!(season.wild_pitches);
        self.hit_by_pitches += u8_option_into_u16!(season.hit_by_pitches);
        self.balks += season.balks;
        self.batters_faced += season.batters_faced.unwrap_or(0);
        self.games_finished += u8_into_u16!(season.games_finished);
        self.runs += u8_into_u16!(season.runs);
        self.sacrifice_hits += u8_option_into_u16!(season.sacrifice_hits);
        self.sacrifice_flies += u8_option_into_u16!(season.sacrifice_flies);
        self.gidp += u8_option_into_u16!(season.gidp);

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
