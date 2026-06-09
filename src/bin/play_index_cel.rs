use std::cmp::Reverse;
use std::collections::HashMap;
use std::error::Error;
use std::path;
use std::time::Instant;

use baseball_tools::database::Sql;
use baseball_tools::player;
use baseball_tools::search::{CelEval, CelExec, Key, StreakSpan};

use clap::{Args, Parser, Subcommand, ValueEnum};
use chrono::Datelike;
use rayon::prelude::*;
use rusqlite::{Connection, OpenFlags};
use tracing::debug;
use tracing_subscriber;


#[derive(Parser)]
struct PlayIndexCelArgs {
    #[arg(short = 'n', long)]
    limit: Option<usize>,

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
    rate: Option<String>,

    #[arg(long)]
    anchor_start: Option<String>,

    #[arg(long)]
    anchor_end: Option<String>,
}


fn load_player_games<T: player::PlayerGamelog + Sql>(conn: &Connection) -> Result<HashMap<Key, Vec<T>>, Box<dyn Error>> {
    let mut select_sql = String::with_capacity(300);
    let table_name = T::table_name();
    select_sql.push_str("SELECT games.date, ");
    for (idx, name) in T::column_names().iter().enumerate() {
        if idx > 0 {
            select_sql.push_str(", ");
        }
        select_sql.push_str("gl.");
        select_sql.push_str(name);
    }
    select_sql.push_str(" FROM ");
    select_sql.push_str(table_name);
    select_sql.push_str(" gl JOIN games ON gl.game_id = games.game_id");
    debug!(sql = select_sql, length = select_sql.len(), "SQL to select player game logs");

    let load_start = Instant::now();
    let mut players = HashMap::new();
    let mut statement = conn.prepare(&select_sql)?;
    let player_rows = statement.query_map([], |row| Ok((row.get(0), T::read_row(&row, 1))))?;
    let mut found_game_logs = 0;
    for rows in player_rows {
        let (date, gl) = rows?;
        let gl = gl?;
        let date: chrono::NaiveDate = date?;
        let key = Key { id: gl.player_id().to_string(), year: date.year() };
        let entry = players.entry(key).or_insert_with(|| Vec::new());
        entry.push(gl);
        found_game_logs += 1;
    }
    let load_end = Instant::now();
    debug!(player_seasons = players.len(), games_found = found_game_logs, duration = format!("{:?}", load_end.duration_since(load_start)), "Loaded player games");
    Ok(players)
}


fn player_game_streak<T>(streak_args: &StreakArgs, mut players: HashMap<Key, Vec<T>>) -> Result<(), Box<dyn Error>>
    where T: Send + Sync + player::PlayerGamelog + CelEval
{
    let mut exec = CelExec::new();
    exec.set_condition(&streak_args.condition)?;
    if let Some(ref program) = streak_args.count {
        exec.set_count(program)?;
    }

    let sort_start = Instant::now();
    players.par_iter_mut().for_each(|(_k, games)| games.sort_unstable_by_key(|g| g.career_game()));
    let sort_end = Instant::now();
    debug!(duration = format!("{:?}", sort_end.duration_since(sort_start)), "Sorted games");

    let eval_start = Instant::now();
    let player_streaks = exec.streak_eval(&players);
    let eval_end = Instant::now();
    debug!(duration = format!("{:?}", eval_end.duration_since(eval_start)), "Evaluated games for streaks");

    let check_start = Instant::now();
    let streaks = CelExec::find_streaks(&player_streaks);
    let check_end = Instant::now();
    debug!(duration = format!("{:?}", check_end.duration_since(check_start)), "Found streaks");

    display_streaks(streaks);

    Ok(())
}


fn batting_game_window(window_args: &WindowArgs, mut players: HashMap<Key, Vec<player::BattingGamelog>>) -> Result<(), Box<dyn Error>>
{
    Ok(())
}

fn find_player_game_log_streaks() {
}

fn display_streaks(mut streaks: Vec<StreakSpan>) {
    streaks.sort_unstable_by_key(|streak| Reverse(streak.count));
    println!("Total streaks: {}", streaks.len());
    if streaks.len() > 0 {
        println!("player ID | game start | game end | count | streak length");
        for streak in streaks.iter().take(200) {
            println!("{} | {} | {} | {} | {}", streak.id, streak.start, streak.end, streak.count, streak.length);
        }
    }
}


fn run() -> Result<(), Box<dyn Error>> {
    let args = PlayIndexCelArgs::parse();

    let connection = Connection::open_with_flags(args.database, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    match (args.table, args.mode) {
        (SearchTable::BattingGameLogs, SearchCommand::Streak(streak_args)) => {
            let batters: HashMap<_, Vec<player::BattingGamelog>> = load_player_games(&connection)?;
            player_game_streak(&streak_args, batters)?;
        }
        (SearchTable::FieldingGameLogs, SearchCommand::Streak(streak_args)) => {
            let fielders: HashMap<_, Vec<player::FieldingGamelog>> = load_player_games(&connection)?;
            player_game_streak(&streak_args, fielders)?;
        }
        (SearchTable::PitchingGameLogs, SearchCommand::Streak(streak_args)) => {
            let pitchers: HashMap<_, Vec<player::PitchingGamelog>> = load_player_games(&connection)?;
            player_game_streak(&streak_args, pitchers)?;
        }
        _ => {
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    run()
}
