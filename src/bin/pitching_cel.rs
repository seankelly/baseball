use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::path;

use baseball::lahman;

use cel_interpreter::{Context, Program, Value};
use clap::Parser;
use csv::Writer;


#[derive(Parser)]
struct Args {
    #[arg(long)]
    career: bool,

    #[arg(long)]
    season: bool,

    #[arg(short = 'n', long)]
    limit: Option<usize>,

    #[arg(long, value_name = "PROGRAM")]
    filter: Option<String>,

    #[arg(long = "csv")]
    csv_file: Option<path::PathBuf>,

    #[arg(value_name = "FILE")]
    pitching_file: path::PathBuf,
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
        let season_hits: u16 = season.hits.into();
        self.hits += season_hits;
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
        let season_hit_by_pitches: u16 = season.hit_by_pitches.into();
        self.hit_by_pitches += season_hit_by_pitches;
        self.balks += season.balks;
        let season_batters_faced: u16 = season.batters_faced.into();
        self.batters_faced += season_batters_faced;
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
    }

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


fn load_pitching(pitching_file: &path::Path) -> Result<Vec<lahman::pitching::Pitching>, Box<dyn Error>> {
    let mut seasons = Vec::new();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(&pitching_file)?;
    for result in reader.deserialize() {
        if let Ok(season) = result {
            let season: lahman::pitching::Pitching = season;
            seasons.push(season);
        }
    }

    Ok(seasons)
}


fn parse_career(seasons: &[lahman::pitching::Pitching]) -> HashMap<String, PitchingCareer> {
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

    return career;
}

fn filter_option(career: &PitchingCareer, context: &Context, program: &Program) -> bool {
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


fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let seasons = load_pitching(&args.pitching_file)?;

    if args.career {
        let careers_map = parse_career(&seasons);
        let context = Context::default();

        let careers: Vec<&PitchingCareer> = match args.filter {
            Some(filter_prog) => {
                let program = Program::compile(&filter_prog)?;
                careers_map.values().filter(|career| filter_option(career, &context, &program) ).collect()
            }
            None => {
                careers_map.values().collect()
            }
        };

        println!("found {} matches", careers.len());

        let mut wtr = Writer::from_writer(vec![]);
        for career in &careers {
            wtr.serialize(career)?;
        }
        print!("{}", String::from_utf8(wtr.into_inner()?)?);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
