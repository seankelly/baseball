use std::cmp::Reverse;
use std::collections::HashMap;
use std::error::Error;
use std::path;
use std::time::Instant;

use baseball_tools::database::Sql;
use baseball_tools::games;
use baseball_tools::player;
use baseball_tools::search::{CelEval, CelExec, Key, SearchKey, StreakSpan, WindowEntry};

use clap::{Args, Parser, Subcommand, ValueEnum};
use chrono::Datelike;
use rayon::prelude::*;
use rusqlite::{Connection, OpenFlags};
use tracing::debug;


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
    TeamGames,
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
    #[arg(short = 'c', long)]
    career: bool,

    #[arg(short = 's', long, value_name = "YEAR")]
    year_start: Option<i32>,

    #[arg(short = 'e', long, value_name = "YEAR")]
    year_end: Option<i32>,

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
    #[arg(short = 'c', long)]
    career: bool,

    #[arg(short = 's', long, value_name = "YEAR")]
    year_start: Option<i32>,

    #[arg(short = 'e', long, value_name = "YEAR")]
    year_end: Option<i32>,

    #[arg(long, value_name = "PROGRAM")]
    rate: Option<String>,

    #[arg(long)]
    anchor_start: Option<String>,

    #[arg(long)]
    anchor_end: Option<String>,

    #[arg()]
    size: u8,

    #[arg()]
    count: String,
}


struct QueryArgs {
    career: bool,
    year_start: Option<i32>,
    year_end: Option<i32>,
}

impl QueryArgs {
    fn from_streak(args: &StreakArgs) -> Self {
        Self {
            career: args.career,
            year_start: args.year_start,
            year_end: args.year_end,
        }
    }

    fn from_window(args: &WindowArgs) -> Self {
        Self {
            career: args.career,
            year_start: args.year_start,
            year_end: args.year_end,
        }
    }

    fn build_game_log_query<T: Sql>(&self) -> (String, Vec<(&str, String)>) {
        let mut select_sql = String::with_capacity(300);
        let mut params = Vec::new();
        let table_name = T::table_name();

        // When loading from the team's table, no need to JOIN because the table has the game date
        // as a column.
        let loading_teams = table_name == "games";

        select_sql.push_str("SELECT ");
        // Need to do a join if not separating the games by year or if limiting the games to select
        // by a year.
        let need_where = self.year_start.is_some() || self.year_end.is_some();
        let join = !loading_teams && (!self.career || need_where);
        // This column is only necessary when needing to split up the player game logs by season.
        // It can be skipped in career mode.
        if !self.career && !loading_teams {
            select_sql.push_str("games.date, ");
        }
        for (idx, name) in T::column_names().iter().enumerate() {
            if idx > 0 {
                select_sql.push_str(", ");
            }
            if !loading_teams {
                select_sql.push_str("gl.");
            }
            select_sql.push_str(name);
        }
        select_sql.push_str(" FROM ");
        select_sql.push_str(table_name);
        // Don't rename when loading teams.
        if !loading_teams {
            select_sql.push_str(" gl");
        }
        if join {
            select_sql.push_str(" JOIN games ON gl.game_id = games.game_id");
        }

        if need_where {
            select_sql.push_str(" WHERE");
        }
        if let Some(year) = self.year_start {
            select_sql.push_str(" strftime('%Y', games.date) >= :start");
            params.push((":start", year.to_string()));
        }
        if let Some(year) = self.year_end {
            if self.year_start.is_some() {
                select_sql.push_str(" AND");
            }
            select_sql.push_str(" strftime('%Y', games.date) <= :end");
            params.push((":end", year.to_string()));
        }
        debug!(sql = select_sql, career = self.career, year_start = self.year_start, year_end = self.year_end, length = select_sql.len(), "SQL to select game logs");
        (select_sql, params)
    }
}


fn load_player_games<T: SearchKey + Sql>(conn: &Connection, args: &QueryArgs) -> Result<HashMap<Key, Vec<T>>, Box<dyn Error>> {
    let (select_sql, params) = args.build_game_log_query::<T>();
    let load_start = Instant::now();
    let mut players = HashMap::new();
    let mut statement = conn.prepare(&select_sql)?;
    let mut found_game_logs = 0;
    let mut rows = statement.query(&params[0..])?;
    while let Some(row) = rows.next()? {
        let gl;
        let key = if args.career {
            gl = T::read_row(row, 0)?;
            Key { id: gl.subject_id().to_string(), year: 0 }
        }
        else {
            let date: chrono::NaiveDate = row.get(0)?;
            gl = T::read_row(row, 1)?;
            Key { id: gl.subject_id().to_string(), year: date.year() }
        };
        let entry = players.entry(key).or_insert_with(|| Vec::new());
        entry.push(gl);
        found_game_logs += 1;
    }
    let load_end = Instant::now();
    debug!(player_seasons = players.len(), games_found = found_game_logs, duration = format!("{:?}", load_end.duration_since(load_start)), "Loaded player games");
    Ok(players)
}


fn find_game_streaks<T>(streak_args: &StreakArgs, mut games: HashMap<Key, Vec<T>>) -> Result<(), Box<dyn Error>>
    where T: Send + Sync + SearchKey + CelEval
{
    let mut exec = CelExec::default();
    exec.set_condition(&streak_args.condition)?;
    if let Some(ref program) = streak_args.count {
        exec.set_count(program)?;
    }

    let sort_start = Instant::now();
    games.par_iter_mut().for_each(|(_k, games)| games.sort_unstable_by_key(|g| g.order()));
    let sort_end = Instant::now();
    debug!(duration = format!("{:?}", sort_end.duration_since(sort_start)), "Sorted games");

    let eval_start = Instant::now();
    let streak_map = exec.streak_eval(&games);
    let eval_end = Instant::now();
    debug!(duration = format!("{:?}", eval_end.duration_since(eval_start)), "Evaluated games for streaks");

    let check_start = Instant::now();
    let streaks = CelExec::find_streaks(&streak_map);
    let check_end = Instant::now();
    debug!(duration = format!("{:?}", check_end.duration_since(check_start)), "Found streaks");

    display_streaks(streaks);

    Ok(())
}


fn load_team_games(conn: &Connection, args: &QueryArgs) -> Result<HashMap<Key, Vec<games::TeamGameLog>>, Box<dyn Error>> {
    let (select_sql, params) = args.build_game_log_query::<games::GameLog>();
    let load_start = Instant::now();
    let mut team_seasons = HashMap::new();
    let mut statement = conn.prepare(&select_sql)?;
    let mut found_game_logs = 0;
    let mut rows = statement.query(&params[0..])?;
    while let Some(row) = rows.next()? {
        let gl = games::GameLog::read_row(row, 0)?;
        let (home, visitor) = gl.each_team_game();
        {
            let home_key = Key { id: home.subject_id().to_string(), year: home.date.year() };
            let entry = team_seasons.entry(home_key).or_insert_with(|| Vec::new());
            entry.push(home);
        }
        {
            let visitor_key = Key { id: visitor.subject_id().to_string(), year: visitor.date.year() };
            let entry = team_seasons.entry(visitor_key).or_insert_with(|| Vec::new());
            entry.push(visitor);
        }
        found_game_logs += 1;
    }
    let load_end = Instant::now();
    debug!(team_seasons = team_seasons.len(), games_found = found_game_logs, duration = format!("{:?}", load_end.duration_since(load_start)), "Loaded team games");
    Ok(team_seasons)
}


fn display_streaks(mut streaks: Vec<StreakSpan>) {
    streaks.sort_unstable_by_key(|streak| Reverse(streak.count));
    println!("Total streaks: {}", streaks.len());
    if !streaks.is_empty() {
        println!("subject ID | game start | game end | count | streak length");
        for streak in streaks.iter().take(200) {
            println!("{} | {} | {} | {} | {}", streak.id, streak.start, streak.end, streak.count, streak.length);
        }
    }
}


fn player_game_window<T>(window_args: &WindowArgs, mut players: HashMap<Key, Vec<T>>) -> Result<(), Box<dyn Error>>
    where T: Send + Sync + SearchKey + CelEval
{
    let mut exec = CelExec::default();
    exec.set_count(&window_args.count)?;

    let sort_start = Instant::now();
    players.par_iter_mut().for_each(|(_k, games)| games.sort_unstable_by_key(|g| g.order()));
    let sort_end = Instant::now();
    debug!(duration = format!("{:?}", sort_end.duration_since(sort_start)), "Sorted games");

    let eval_start = Instant::now();
    let player_windows = exec.window_eval(&players, window_args.size as usize);
    let eval_end = Instant::now();
    debug!(size = window_args.size, duration = format!("{:?}", eval_end.duration_since(eval_start)), "Evaluated game windows");

    let check_start = Instant::now();
    let windows = exec.sort_windows(&player_windows);
    let check_end = Instant::now();
    debug!(duration = format!("{:?}", check_end.duration_since(check_start)), "Sorted windows");

    display_windows(windows);

    Ok(())
}


fn display_windows(windows: Vec<&WindowEntry>) {
    println!("Total windows: {}", windows.len());
    if !windows.is_empty() {
        println!("player ID | game start | game end | count");
        for window in windows.iter().take(200) {
            println!("{} | {} | {} | {}", window.id, window.start, window.end, window.count);
        }
    }
}


fn find_player_game_log_streaks<T>(connection: &Connection, streak_args: &StreakArgs) -> Result<(), Box<dyn Error>>
    where T: Send + Sync + CelEval + SearchKey + Sql
{
    let query_args = QueryArgs::from_streak(&streak_args);
    let batters: HashMap<_, Vec<T>> = load_player_games(connection, &query_args)?;
    find_game_streaks(streak_args, batters)?;
    Ok(())
}


fn find_team_game_streaks(connection: &Connection, streak_args: &StreakArgs) -> Result<(), Box<dyn Error>>
{
    let query_args = QueryArgs::from_streak(&streak_args);
    let team_seasons: HashMap<_, Vec<games::TeamGameLog>> = load_team_games(connection, &query_args)?;
    find_game_streaks(streak_args, team_seasons)?;
    Ok(())
}


fn run() -> Result<(), Box<dyn Error>> {
    let args = PlayIndexCelArgs::parse();

    let connection = Connection::open_with_flags(args.database, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    match (args.table, args.mode) {
        (SearchTable::BattingGameLogs, SearchCommand::Streak(streak_args)) => {
            find_player_game_log_streaks::<player::BattingGamelog>(&connection, &streak_args)?;
        }
        (SearchTable::FieldingGameLogs, SearchCommand::Streak(streak_args)) => {
            find_player_game_log_streaks::<player::FieldingGamelog>(&connection, &streak_args)?;
        }
        (SearchTable::PitchingGameLogs, SearchCommand::Streak(streak_args)) => {
            find_player_game_log_streaks::<player::PitchingGamelog>(&connection, &streak_args)?;
        }
        (SearchTable::TeamGames, SearchCommand::Streak(streak_args)) => {
            find_team_game_streaks(&connection, &streak_args)?;
        }
        (SearchTable::BattingGameLogs, SearchCommand::Window(window_args)) => {
            let query_args = QueryArgs::from_window(&window_args);
            let batters: HashMap<_, Vec<player::BattingGamelog>> = load_player_games(&connection, &query_args)?;
            player_game_window(&window_args, batters)?;
        }
        (SearchTable::FieldingGameLogs, SearchCommand::Window(window_args)) => {
            let query_args = QueryArgs::from_window(&window_args);
            let fielders: HashMap<_, Vec<player::FieldingGamelog>> = load_player_games(&connection, &query_args)?;
            player_game_window(&window_args, fielders)?;
        }
        (SearchTable::PitchingGameLogs, SearchCommand::Window(window_args)) => {
            let query_args = QueryArgs::from_window(&window_args);
            let pitchers: HashMap<_, Vec<player::PitchingGamelog>> = load_player_games(&connection, &query_args)?;
            player_game_window(&window_args, pitchers)?;
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
