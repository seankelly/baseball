use std::convert::From;
use std::error::Error;
use std::path;

use baseball::chadwick::gamelogs::{BattingGamelog, FieldingGamelog, PitchingGamelog};
use baseball_tools::search::{Search, SortOrder};

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
    sort_order: Option<CliSortOrder>,

    #[arg(long = "csv")]
    csv_file: Option<path::PathBuf>,

    #[command(subcommand)]
    gamelog: GamelogType,
}


#[derive(Clone, ValueEnum)]
enum CliSortOrder {
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


impl From<CliSortOrder> for SortOrder {
    fn from(item: CliSortOrder) -> Self {
        match item {
            CliSortOrder::Asc => Self::Asc,
            CliSortOrder::Desc => Self::Desc,
        }
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


fn dump_csv<T: serde::Serialize>(gamelog: &Vec<T>, csv_file: &path::Path) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(&csv_file)?;
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

    let sort_order = args.sort_order.unwrap_or(CliSortOrder::Asc).into();

    let results = match args.gamelog {
        GamelogType::Batting(files) => {
            let mut gamelogs = Vec::new();
            for file in &files.gamelogs {
                let mut gamelog: Vec<BattingGamelog> = load_gamelog_file(file)?;
                search.filter(&mut gamelog);
                gamelogs.par_extend(gamelog);
            }
            search.sort(&mut gamelogs, &sort_order);
            if let Some(csv_file) = args.csv_file {
                dump_csv(&gamelogs, &csv_file)?;
            }
            results(&gamelogs, args.limit)?
        }
        GamelogType::Fielding(files) => {
            let mut gamelogs = Vec::new();
            for file in &files.gamelogs {
                let mut gamelog: Vec<FieldingGamelog> = load_gamelog_file(file)?;
                search.filter(&mut gamelog);
                gamelogs.par_extend(gamelog);
            }
            search.sort(&mut gamelogs, &sort_order);
            if let Some(csv_file) = args.csv_file {
                dump_csv(&gamelogs, &csv_file)?;
            }
            results(&gamelogs, args.limit)?
        }
        GamelogType::Pitching(files) => {
            let mut gamelogs = Vec::new();
            for file in &files.gamelogs {
                let mut gamelog: Vec<PitchingGamelog> = load_gamelog_file(file)?;
                search.filter(&mut gamelog);
                gamelogs.par_extend(gamelog);
            }
            search.sort(&mut gamelogs, &sort_order);
            if let Some(csv_file) = args.csv_file {
                dump_csv(&gamelogs, &csv_file)?;
            }
            results(&gamelogs, args.limit)?
        }
    };

    println!("{}", results);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
