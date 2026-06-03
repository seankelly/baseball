use std::error::Error;
use std::collections::HashMap;
use std::path;
use std::time::Instant;

use baseball_tools::player;
use baseball_tools::search::{CelEval, CelExec};

use cel::{Context, Program, Value};
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


struct StreakSpan<T> {
    key: T,
    start: String,
    end: String,
    length: u32,
}

struct StreakEntry {
    game_id: String,
    result: bool,
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
    let context = Context::default();
    let program = Program::compile(&streak_args.condition)?;
    let references = program.references();
    let variables = references.variables();
    let player_streaks: HashMap<_, _> = players.par_iter().map(|kv| {
        let (key, value) = kv;
        let bool_value: Vec<_> = value.iter().map(|e| {
            let mut ctx = context.new_inner_scope();
            let result;
            if e.add_cel_variables(&mut ctx, &variables).is_err() {
                result = false;
            }
            else {
                result = match program.execute(&ctx) {
                    Ok(Value::Bool(true)) => true,
                    Ok(_) => false,
                    Err(error) => {
                        eprintln!("error evaluating: {error}");
                        false
                    }
                };
            }
            let entry = StreakEntry {
                game_id: e.game_id.clone(),
                result,
            };
            entry
        }).collect();
        (key, bool_value)
    }).collect();
    let eval_end = Instant::now();
    println!("Evaluated in {:?}", eval_end.duration_since(eval_start));

    println!("Checking results");
    let check_start = Instant::now();
    let mut streaks = Vec::with_capacity(150);
    let mut streak_minimum = 2;
    for (player_name, games) in player_streaks.iter() {
        let mut streak_start = None;
        let mut streak_end = None;
        let mut streak_length = 0;
        for game_entry in games {
            if game_entry.result {
                streak_length += 1;
                if streak_start.is_none() {
                    streak_start = Some(&game_entry.game_id);
                }
                streak_end = Some(&game_entry.game_id);
            }
            else {
                if let (Some(start), Some(end)) = (streak_start, streak_end) {
                    if streak_length >= streak_minimum {
                        let span = StreakSpan {
                            key: player_name,
                            start: start.clone(),
                            end: end.clone(),
                            length: streak_length,
                        };
                        streaks.push(span);
                    }
                }
                streak_start = None;
                streak_end = None;
                streak_length = 0;
            }
        }

        // Check for streaks that end with the player's final game.
        if let (Some(start), Some(end)) = (streak_start, streak_end) {
            if streak_length >= streak_minimum {
                let span = StreakSpan {
                    key: player_name,
                    start: start.clone(),
                    end: end.clone(),
                    length: streak_length,
                };
                streaks.push(span);
            }
        }

        // Sort the spans and check the 100th entry to see if the streak minimum length should
        // increase. If so, prune the list to only spans meeting the new minimum.
        streaks.sort_unstable_by(|a, b| b.length.cmp(&a.length));
        let mut prune_streaks = false;
        if let Some(span) = streaks.get(100) {
            if span.length > streak_minimum {
                prune_streaks = true;
                streak_minimum = span.length;
            }
        }
        if prune_streaks {
            streaks.retain(|span| span.length >= streak_minimum);
        }
    }
    let check_end = Instant::now();
    println!("Checked in {:?}", check_end.duration_since(check_start));

    println!("Total streaks: {}", streaks.len());
    if streaks.len() < 200 {
        println!("player ID | game start | game end | streak length");
        for streak in &streaks {
            println!("{} | {} | {} | {}", streak.key, streak.start, streak.end, streak.length);
        }
    }

    Ok(())
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
