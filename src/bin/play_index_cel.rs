use std::error::Error;
use std::collections::HashMap;
use std::path;
use std::time::Instant;

use baseball_tools::database::Sql;
use baseball_tools::player;
use baseball_tools::search::{CelExec, StreakSpan};

use clap::{Args, Parser, Subcommand, ValueEnum};
use rayon::prelude::*;
use rusqlite::{Connection, OpenFlags};


#[derive(Parser)]
struct PlayIndexCelArgs {
    #[arg(short = 'n', long)]
    limit: Option<usize>,

    #[arg(long, value_name = "PROGRAM")]
    pre_filter: Option<String>,

    #[arg(long, value_name = "PROGRAM")]
    condition: Option<String>,

    #[arg(long, value_name = "PROGRAM")]
    sort_key: Option<String>,

    database: path::PathBuf,
    table: SearchTable,
    #[command(subcommand)]
    mode: SearchCommand,
}


#[derive(Clone, ValueEnum)]
enum SearchTable {
    BattingGameLogs,
    FieldingGameLogs,
    PitchingGameLogs,
    Games,
}

#[derive(Clone, Subcommand)]
enum SearchCommand {
    #[command(arg_required_else_help = true)]
    Search(SearchArgs),
    Streak(StreakArgs),
    Window(WindowArgs),
}

#[derive(Clone, Args)]
struct SearchArgs {
    #[arg(long, value_name = "PROGRAM")]
    filter: Option<String>,

    #[arg(long, value_name = "PROGRAM")]
    sort_key: Option<String>,
    // sort order
}

#[derive(Clone, Args)]
struct StreakArgs {
    #[arg(long, value_name = "PROGRAM")]
    pre_filter: Option<String>,

    #[arg(long, value_name = "PROGRAM")]
    count: Option<String>,

    #[arg(long)]
    anchor_start: Option<String>,

    #[arg(long)]
    anchor_end: Option<String>,

    #[arg()]
    condition: String,
}

#[derive(Clone, Args)]
struct WindowArgs {
    #[arg(long, value_name = "PROGRAM")]
    condition: Option<String>,

    #[arg(long, value_name = "PROGRAM")]
    rate: Option<String>,

    #[arg(long)]
    anchor_start: Option<String>,

    #[arg(long)]
    anchor_end: Option<String>,
}


fn load_player_batting(conn: &Connection) -> Result<HashMap<String, Vec<player::BattingGamelog>>, Box<dyn Error>> {
    let mut select_sql = String::with_capacity(250);
    select_sql.push_str("SELECT ");
    for (idx, name) in player::BattingGamelog::column_names().iter().enumerate() {
        if idx > 0 {
            select_sql.push_str(", ");
        }
        select_sql.push_str(name);
    }
    select_sql.push_str(" FROM batting_gamelogs");

    let load_start = Instant::now();
    let mut players = HashMap::new();
    let mut statement = conn.prepare(&select_sql)?;
    let player_rows = statement.query_map([], |row| player::BattingGamelog::read_row(&row))?;
    let mut found_game_logs = 0;
    for gl in player_rows {
        let gl = gl?;
        let entry = players.entry(gl.player_id.clone()).or_insert_with(|| Vec::new());
        entry.push(gl);
        found_game_logs += 1;
    }
    let load_end = Instant::now();
    println!("Loaded {} players ({} games) in {:?}", players.len(), found_game_logs, load_end.duration_since(load_start));
    Ok(players)
}


fn player_batting_streak(conn: &Connection, streak_args: &StreakArgs) -> Result<(), Box<dyn Error>> {
    let mut exec = CelExec::new();
    exec.set_condition(&streak_args.condition)?;

    let mut players = load_player_batting(conn)?;

    let sort_start = Instant::now();
    players.par_iter_mut().for_each(|(_k, games)| games.sort_unstable_by_key(|g| g.career_game));
    let sort_end = Instant::now();
    println!("Sorted all games in {:?}", sort_end.duration_since(sort_start));

    println!("Running program");
    let eval_start = Instant::now();
    let player_streaks = exec.streak_eval(&players);
    let eval_end = Instant::now();
    println!("Evaluated in {:?}", eval_end.duration_since(eval_start));

    println!("Checking results");
    let check_start = Instant::now();
    let streaks = CelExec::find_streaks(&player_streaks);
    let check_end = Instant::now();
    println!("Checked in {:?}", check_end.duration_since(check_start));

    display_streaks(streaks);

    Ok(())
}

fn find_player_game_log_streaks() {
}

fn display_streaks<T: std::fmt::Display>(streaks: Vec<StreakSpan<T>>) {
    println!("Total streaks: {}", streaks.len());
    if streaks.len() < 200 {
        println!("player ID | game start | game end | streak length");
        for streak in &streaks {
            println!("{} | {} | {} | {}", streak.key, streak.start, streak.end, streak.length);
        }
    }
}


fn run() -> Result<(), Box<dyn Error>> {
    let args = PlayIndexCelArgs::parse();

    let connection = Connection::open_with_flags(args.database, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    match (args.table, args.mode) {
        (SearchTable::BattingGameLogs, SearchCommand::Streak(streak_args)) => {
            player_batting_streak(&connection, &streak_args)?;
        }
        _ => {
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
