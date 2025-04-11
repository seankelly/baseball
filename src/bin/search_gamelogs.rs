use std::error::Error;
use std::path;

use baseball_tools::gamelogs::{BattingGamelog, FieldingGamelog, PitchingGamelog};

use cel_interpreter::{Context, Program, Value};
use clap::{Args, Parser, Subcommand, ValueEnum};
use csv::Writer;
use rayon::prelude::*;
use serde::de::DeserializeOwned;


#[derive(Parser)]
struct GamelogArgs {
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
    gamelog: GamelogType,
}


#[derive(Clone, ValueEnum)]
enum SortOrder {
    Asc,
    Desc,
}


#[derive(Subcommand)]
enum GamelogType {
    Batting(GamelogFile),
    Fielding(GamelogFile),
    Pitching(GamelogFile),
}


#[derive(Args)]
struct GamelogFile {
    #[arg(value_name = "FILE")]
    gamelogs: Vec<path::PathBuf>,
}


trait CelSearch {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>>;
}


struct Search<'a> {
    context: Context<'a>,
    filter_program: Option<Program>,
    sort_program: Option<Program>,
}


impl<'a> Search<'a> {
    fn new(filter: Option<&str>, sort: Option<&str>) -> Result<Self, Box<dyn Error>> {
        let filter_program = match filter {
            Some(source) => Some(Program::compile(source)?),
            None => None,
        };
        let sort_program = match sort {
            Some(source) => Some(Program::compile(source)?),
            None => None,
        };
        let context = Context::default();

        Ok(Self {
            context,
            filter_program,
            sort_program,
        })
    }

    fn filter<T: CelSearch>(&self, input: &mut Vec<T>) {
        /*
        match self.filter_program.as_ref() {
            Some(filter_program) => {
                input
                    .into_par_iter()
                    .filter(|element| self.filter_option(element, &filter_program)).collect()
            }
            None => input,
        }
        */
        if let Some(filter_program) = self.filter_program.as_ref() {
            input.retain(|element| self.filter_option(element, &filter_program));
        }
    }

    fn sort<T: CelSearch + Send>(&self, input: &mut Vec<T>, sort_order: &SortOrder) {
        if let Some(sort_program) = self.sort_program.as_ref() {
            input.par_sort_unstable_by(|a, b| {
                let a_res = self.sort_key(a, &sort_program);
                let b_res = self.sort_key(b, &sort_program);
                match sort_order {
                    SortOrder::Asc => { a_res.total_cmp(&b_res) }
                    SortOrder::Desc => { b_res.total_cmp(&a_res) }
                }
            });
        }
    }

    fn filter_option<T: CelSearch>(&self, element: &T, program: &Program) -> bool {
        let mut player_ctx = self.context.new_inner_scope();
        if let Err(_) = element.add_cel_variables(&mut player_ctx) {
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

    fn sort_key<T: CelSearch>(&self, career: &T, program: &Program) -> f64 {
        let mut player_ctx = self.context.new_inner_scope();
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
}


impl CelSearch for BattingGamelog {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>> {
        context.add_variable("AB", self.AB)?;
        context.add_variable("PA", self.PA)?;
        context.add_variable("R", self.R)?;
        context.add_variable("H", self.H)?;
        context.add_variable("D", self.D)?;
        context.add_variable("T", self.T)?;
        context.add_variable("HR", self.HR)?;
        context.add_variable("RBI", self.RBI)?;
        context.add_variable("SB", self.SB)?;
        context.add_variable("CS", self.CS)?;
        context.add_variable("BB", self.BB)?;
        context.add_variable("SO", self.SO)?;
        context.add_variable("IBB", self.IBB)?;
        context.add_variable("HBP", self.HBP)?;
        context.add_variable("SH", self.SH)?;
        context.add_variable("SF", self.SF)?;
        context.add_variable("GIDP", self.GIDP)?;
        context.add_variable("pos", self.POS.clone())?;
        Ok(())
    }
}


impl CelSearch for FieldingGamelog {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>> {
        context.add_variable("POS", self.POS)?;
        context.add_variable("O", self.O)?;
        context.add_variable("PO", self.PO)?;
        context.add_variable("A", self.A)?;
        context.add_variable("E", self.E)?;
        context.add_variable("DP", self.DP)?;
        context.add_variable("TP", self.TP)?;
        context.add_variable("BIP", self.BIP)?;
        context.add_variable("BF", self.BF)?;
        Ok(())
    }
}


impl CelSearch for PitchingGamelog {
    fn add_cel_variables(&self, context: &mut Context) -> Result<(), Box<dyn Error>> {
        context.add_variable("GS", self.GS)?;
        context.add_variable("GF", self.GF)?;
        context.add_variable("CG", self.CG)?;
        context.add_variable("SHO", self.SHO)?;
        context.add_variable("IPOuts", self.IPouts)?;
        context.add_variable("H", self.H)?;
        context.add_variable("R", self.R)?;
        context.add_variable("ER", self.ER)?;
        context.add_variable("HR", self.HR)?;
        context.add_variable("BB", self.BB)?;
        context.add_variable("SO", self.SO)?;
        context.add_variable("IBB", self.IBB)?;
        context.add_variable("WP", self.WP)?;
        context.add_variable("HBP", self.HBP)?;
        context.add_variable("BK", self.BK)?;
        context.add_variable("BF", self.BF)?;
        context.add_variable("GF", self.GF)?;
        context.add_variable("P", self.P)?;
        context.add_variable("S", self.S)?;
        context.add_variable("decision", self.decision.clone())?;
        Ok(())
    }
}


fn load_gamelog_file<T: DeserializeOwned>(file: &path::Path) -> Result<Vec<T>, Box<dyn Error>>
{
    let mut seasons = Vec::new();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(&file)?;
    for result in reader.deserialize() {
        if let Ok(season) = result {
            seasons.push(season);
        }
        /*
        match result {
            Ok(season) => {
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


fn dump_csv<T: serde::Serialize, F: std::io::Write>(gamelog: &Vec<T>, csv_file: &mut F) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(csv_file);
    for row in gamelog {
        writer.serialize(row)?;
    }
    Ok(())
}


fn results<T: serde::Serialize>(games: &Vec<T>, limit: Option<usize>) -> Result<String, Box<dyn Error>> {
    let mut wtr = Writer::from_writer(vec![]);
    let limit = limit.unwrap_or(games.len());
    for game in games.iter().take(limit) {
        wtr.serialize(game)?;
    }

    Ok(String::from_utf8(wtr.into_inner()?)?)
}


fn run() -> Result<(), Box<dyn Error>> {
    let args = GamelogArgs::parse();
    let search = Search::new(args.filter.as_deref(), args.sort_key.as_deref())?;

    let sort_order = args.sort_order.as_ref().unwrap_or(&SortOrder::Asc);

    let results = match args.gamelog {
        GamelogType::Batting(files) => {
            let mut gamelogs = Vec::new();
            for file in &files.gamelogs {
                let mut gamelog: Vec<BattingGamelog> = load_gamelog_file(file)?;
                search.filter(&mut gamelog);
                gamelogs.par_extend(gamelog);
            }
            search.sort(&mut gamelogs, sort_order);
            results(&gamelogs, args.limit)?
        }
        GamelogType::Fielding(files) => {
            let mut gamelogs = Vec::new();
            for file in &files.gamelogs {
                let mut gamelog: Vec<FieldingGamelog> = load_gamelog_file(file)?;
                search.filter(&mut gamelog);
                gamelogs.par_extend(gamelog);
            }
            search.sort(&mut gamelogs, sort_order);
            results(&gamelogs, args.limit)?
        }
        GamelogType::Pitching(files) => {
            let mut gamelogs = Vec::new();
            for file in &files.gamelogs {
                let mut gamelog: Vec<PitchingGamelog> = load_gamelog_file(file)?;
                search.filter(&mut gamelog);
                gamelogs.par_extend(gamelog);
            }
            search.sort(&mut gamelogs, sort_order);
            results(&gamelogs, args.limit)?
        }
    };

    println!("{}", results);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
