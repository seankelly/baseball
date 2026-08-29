use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path;
use std::process::{ChildStdout, Command, Stdio};

use baseball::register::Person;
use baseball::retrosheet::game;
use baseball::chadwick::gamelogs::{gamelogs_from_daily_stats, PlayerGameLogs};
use baseball_tools::database::Sql;
use baseball_tools::games;
use baseball_tools::player;
use baseball_tools::internals::Guts;

use clap::Parser;
use csv::ReaderBuilder;
use rusqlite::{Connection, Result, Transaction, named_params};
use tracing::debug;


#[derive(Parser)]
struct DatabaseArgs {
    #[arg(short, long)]
    database: Option<path::PathBuf>,

    #[arg(short, long)]
    init: bool,

    #[arg(short = 'R', long)]
    register_dir: Option<path::PathBuf>,

    retrosheet_dir: path::PathBuf,

    start_season: Option<u16>,
    last_season: Option<u16>,
}


enum GameFiles {
    Event,
    Deduced,
    BoxScore,
}


impl fmt::Display for GameFiles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_type = match self {
            &GameFiles::Event => "event",
            &GameFiles::Deduced => "deduced",
            &GameFiles::BoxScore => "box score",
        };
        write!(f, "{}", file_type)
    }
}


struct GameLogLoader<'a> {
    conn: &'a mut Connection,
    retrosheet_dir: path::PathBuf,
    batting_career_games: HashMap<String, u16>,
    fielding_career_games: HashMap<String, u16>,
    pitching_career_games: HashMap<String, u16>,
    game_ordering: HashMap<TeamGameLogKey, TeamGameLogValue>,
}


impl<'a> GameLogLoader<'a> {
    fn new(conn: &'a mut Connection, retrosheet_dir: path::PathBuf) -> Self {
        Self {
            conn,
            retrosheet_dir,
            batting_career_games: HashMap::new(),
            fielding_career_games: HashMap::new(),
            pitching_career_games: HashMap::new(),
            game_ordering: HashMap::new(),
        }
    }

    // SQL interaction section.
    fn create_tables(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Creating game log tables");
        let mut tx = self.conn.transaction()?;
        games::GameLog::create_table(&mut tx)?;
        player::BattingGamelog::create_table(&mut tx)?;
        player::FieldingGamelog::create_table(&mut tx)?;
        player::PitchingGamelog::create_table(&mut tx)?;
        tx.commit()?;
        Ok(())
    }

    fn create_indexes(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Creating game indexes");
        self.conn.execute_batch(
            "
            CREATE INDEX games_game_idx ON games (game_id);
            CREATE INDEX games_date_idx ON games (date);
            CREATE INDEX games_year_idx ON games (strftime('%Y', date));
            CREATE INDEX games_away_idx ON games (visitor_team);
            CREATE INDEX games_home_idx ON games (home_team);
            "
        )?;

        println!("Creating gamelog indexes");
        self.conn.execute_batch(
            "
            CREATE INDEX batting_gamelogs_player_idx ON batting_gamelogs (player_id);
            CREATE INDEX batting_gamelogs_game_idx ON batting_gamelogs (game_id);
            CREATE INDEX batting_gamelogs_team_idx ON batting_gamelogs (team_id);
            CREATE INDEX fielding_gamelogs_player_idx ON fielding_gamelogs (player_id);
            CREATE INDEX fielding_gamelogs_game_idx ON fielding_gamelogs (game_id);
            CREATE INDEX fielding_gamelogs_team_idx ON fielding_gamelogs (team_id);
            CREATE INDEX pitching_gamelogs_player_idx ON pitching_gamelogs (player_id);
            CREATE INDEX pitching_gamelogs_game_idx ON pitching_gamelogs (game_id);
            CREATE INDEX pitching_gamelogs_team_idx ON pitching_gamelogs (team_id);
            "
        )?;
        Ok(())
    }

    fn insert_games<T: Sql>(tx: &Transaction, gamelogs: &[T]) -> Result<(), Box<dyn Error>> {
        let mut insert_sql = String::with_capacity(250);
        insert_sql.push_str("INSERT INTO ");
        insert_sql.push_str(T::table_name());
        insert_sql.push_str(" VALUES (");
        for (idx, name) in T::column_names().iter().enumerate() {
            if idx > 0 {
                insert_sql.push_str(", ");
            }
            insert_sql.push(':');
            insert_sql.push_str(name);
        }
        insert_sql.push(')');

        let mut insert = tx.prepare(&insert_sql)?;
        for game in gamelogs {
            game.write_row(&mut insert)?;
        }

        Ok(())
    }

    fn load_season_gamelog(&self, season: &str) -> Result<Vec<games::GameLog>, Box<dyn Error>> {
        let season_dir = self.retrosheet_dir.join(season);
        // Chadwick's Retrosheet seasons either have a GLYYYY.TXT or glYYYY.txt file.
        let gl_file_names = [format!("GL{}.TXT", season), format!("gl{}.txt", season)];
        let mut gamelog_file = None;
        for gl_file in gl_file_names {
            let gl_path = season_dir.join(gl_file);
            if gl_path.exists() {
                gamelog_file = Some(gl_path);
                break;
            }
        }

        let mut games = Vec::new();
        if let Some(gl_path) = gamelog_file {
            let mut reader = ReaderBuilder::new()
                .has_headers(false)
                .from_path(&gl_path)?;
            for result in reader.deserialize() {
                match result {
                    Ok(game) => {
                        let game: game::GameLog = game;
                        games.push(game.into());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
        Ok(games)
    }

    fn load_team_games(&mut self, season: &str) -> Result<HashSet<String>, Box<dyn Error>> {
        println!("Loading games from {} season", season);
        let games = self.load_season_gamelog(season)?;
        debug!(games = games.len(), "Loaded games");
        // Iterate one more time through every pitching game to calculate the league ERA and the
        // unscaled FIP values to get the FIP constant for this season. Additionally, build the
        // map of (game, team) IDs to (date, team game number) for the player game logs.
        let mut league_stats = PitcherStats::new_with_fip(0.0);
        // This tracks all of the games to check if for any missing games when loading the player
        // game logs.
        let mut game_ids = HashSet::with_capacity(games.len());
        // Purge any existing games from past seasons because they aren't necessary. Then reserve
        // space for the expected number of games.
        self.game_ordering.clear();
        self.game_ordering.reserve(games.len() * 2);
        for game in &games {
            league_stats.add_team_gamelog(game);
            game_ids.insert(game.game_id.to_owned());

            // Map home team game and team ID to the date and team game number.
            let home_team = TeamGameLogKey {
                game_id: game.game_id.to_owned(),
                team_id: game.home_team.to_owned(),
            };
            let home_team_value = TeamGameLogValue {
                date: game.date,
                team_game_number: game.home_team_game_number,
            };
            self.game_ordering.insert(home_team, home_team_value);

            // Do the same mapping for the visitor team.
            let visitor_team = TeamGameLogKey {
                game_id: game.game_id.to_owned(),
                team_id: game.visitor_team.to_owned(),
            };
            let visitor_team_value = TeamGameLogValue {
                date: game.date,
                team_game_number: game.visitor_team_game_number,
            };
            self.game_ordering.insert(visitor_team, visitor_team_value);
        }
        let league_fip_constant = league_stats.era() - league_stats.fip();
        println!("Season {} ERA: {}, FIP constant: {}", season, league_stats.era(), league_fip_constant);
        let season_numeric = season.parse::<u16>()?;
        let mut guts = Guts::new(season_numeric);
        guts.fip_constant = league_fip_constant;

        let tx = self.conn.transaction().expect("Could not create transaction");
        update_fip_constant(&tx, &guts)?;
        Self::insert_games(&tx, &games)?;
        tx.commit().expect("Failed to commit transaction");

        Ok(game_ids)
    }

    // Player section of loading game logs.
    fn load_player_daily_stats(&self, season: &str, file_type: &GameFiles) -> Result<ChildStdout, Box<dyn Error>> {
        let season_dir = self.retrosheet_dir.join(season);
        let mut cwdaily = Command::new("cwdaily");
        // Use all of the fields but not everything may make it to a game log.
        cwdaily.args(["-q", "-y", season, "-f", "0-153"]).current_dir(&season_dir);
        cwdaily.args(find_game_files(&season_dir, file_type)?);
        let command = cwdaily.stdin(Stdio::null()).stdout(Stdio::piped());
        match command.spawn() {
            Ok(mut child) => {
                let stdout = child.stdout.take().expect("cwdaily stdout handle not available");
                Ok(stdout)
            }
            Err(err) => {
                Err(Box::new(err))
            }
        }
    }

    fn dated_gamelog_cmp<T: player::PlayerGamelog>(a: &DatedPlayerGamelogs<T>, b: &DatedPlayerGamelogs<T>) -> cmp::Ordering {
        let player_cmp = a.0.player_id().cmp(b.0.player_id());
        match player_cmp {
            cmp::Ordering::Equal => {},
            _ => { return player_cmp; }
        }
        let date_cmp = a.1.cmp(&b.1);
        match date_cmp {
            cmp::Ordering::Equal => {},
            _ => { return date_cmp; }
        }
        a.0.team_id().cmp(b.0.team_id())
    }

    fn order_dated_gamelogs<T, U>(&self, season: i32, chadwick_gl: Vec<T>) -> Vec<U>
        where U: player::PlayerGamelog + std::convert::From<T>
    {
        let game_count = chadwick_gl.len();
        let default_value = TeamGameLogValue {
            date: chrono::NaiveDate::from_ymd_opt(season, 1, 1).unwrap(),
            team_game_number: 0,
        };
        let mut internal_gamelogs: Vec<DatedPlayerGamelogs<U>> = Vec::with_capacity(game_count);
        for gl in chadwick_gl.into_iter() {
            let mut new_gl: U = gl.into();
            let key = TeamGameLogKey {
                game_id: new_gl.game_id().to_string(),
                team_id: new_gl.team_id().to_string(),
            };
            // Need date and team game number.
            let value = self.game_ordering.get(&key).unwrap_or(&default_value);
            new_gl.set_team_game(value.team_game_number);
            internal_gamelogs.push((new_gl, value.date));
        }
        internal_gamelogs.sort_unstable_by(Self::dated_gamelog_cmp);
        internal_gamelogs.into_iter().map(|entry| entry.0).collect()
    }

    fn order_batting_gamelogs(mut gamelogs: Vec<player::BattingGamelog>, career_offset: &mut HashMap<String, u16>) -> Vec<player::BattingGamelog> {
        let mut player = "";
        let mut last_game = "";
        let mut slash_line = BattingSlashLine::new();
        // Start at zero because whether the current game is the same as the previous is checked
        // before setting the player's season game count.
        let mut season_game = 0;
        let mut offset = 0;
        for gl in gamelogs.iter_mut() {
            if player == gl.player_id {
                if last_game != gl.game_id {
                    season_game += 1;
                }
                slash_line.add_gamelog(gl);
                let stats = slash_line.slash_line();
                gl.season_game = season_game;
                gl.career_game = offset + season_game;
                gl.avg = stats.0;
                gl.obp = stats.1;
                gl.slg = stats.2;
            }
            else {
                // Save the new career games played for that player.
                if season_game > 0 && !player.is_empty() {
                    career_offset.insert(player.to_owned(), offset + season_game);
                }
                player = gl.player_id.as_str();
                slash_line.clear();
                slash_line.add_gamelog(gl);
                let stats = slash_line.slash_line();
                season_game = 1;
                offset = career_offset.get(player).copied().unwrap_or(0);
                gl.season_game = season_game;
                gl.career_game = offset + season_game;
                gl.avg = stats.0;
                gl.obp = stats.1;
                gl.slg = stats.2;
            }
            last_game = gl.game_id.as_str();
        }
        gamelogs
    }

    fn order_fielding_gamelogs(mut gamelogs: Vec<player::FieldingGamelog>, career_offset: &mut HashMap<String, u16>) -> Vec<player::FieldingGamelog> {
        let mut player = "";
        let mut last_game = "";
        let mut season_game = 0;
        let mut offset = 0;
        for gl in gamelogs.iter_mut() {
            if player == gl.player_id {
                if last_game != gl.game_id {
                    season_game += 1;
                }
                gl.season_game = season_game;
                gl.career_game = offset + season_game;
            }
            else {
                if season_game > 0 && player.is_empty() {
                    career_offset.insert(player.to_owned(), offset + season_game);
                }
                player = gl.player_id.as_str();
                season_game = 1;
                offset = career_offset.get(player).copied().unwrap_or(0);
                gl.season_game = season_game;
                gl.career_game = offset + season_game;
            }
            last_game = gl.game_id.as_str();
        }
        gamelogs
    }

    fn order_pitching_gamelogs(mut gamelogs: Vec<player::PitchingGamelog>, career_offset: &mut HashMap<String, u16>, fip_constant: f32) -> Vec<player::PitchingGamelog> {
        let mut player = "";
        let mut last_game = "";
        let mut pitcher_stats = PitcherStats::new_with_fip(fip_constant);
        let mut season_game = 0;
        let mut offset = 0;
        for gl in gamelogs.iter_mut() {
            if player == gl.player_id {
                if last_game != gl.game_id {
                    season_game += 1;
                }
                pitcher_stats.add_gamelog(gl);
                gl.season_game = season_game;
                gl.career_game = offset + season_game;
                gl.era = pitcher_stats.era();
                gl.fip = pitcher_stats.fip();
            }
            else {
                if season_game > 0 && player.is_empty() {
                    career_offset.insert(player.to_owned(), offset + season_game);
                }
                player = gl.player_id.as_str();
                pitcher_stats.clear();
                pitcher_stats.add_gamelog(gl);
                season_game = 1;
                offset = career_offset.get(player).copied().unwrap_or(0);
                gl.season_game = season_game;
                gl.career_game = offset + season_game;
                gl.era = pitcher_stats.era();
                gl.fip = pitcher_stats.fip();
            }
            last_game = gl.game_id.as_str();
        }
        gamelogs
    }

    fn load_player_games(&mut self, season: &str, game_ids: &HashSet<String>) -> Result<(), Box<dyn Error>> {
        let (batting_gamelogs, fielding_gamelogs, pitching_gamelogs) = self.load_player_game_logs(season, game_ids)?;

        // Transform Chadwick gamelogs into internal version for the database and sort to allow
        // marking which game number in the season this is for a player.
        let season_numeric = season.parse::<u16>()?;
        let fip_constant = get_fip_constant(self.conn, season_numeric)?.unwrap_or_default();
        let batting_gamelogs = Self::order_batting_gamelogs(
            self.order_dated_gamelogs(season_numeric.into(), batting_gamelogs),
            &mut self.batting_career_games
        );
        let fielding_gamelogs = Self::order_fielding_gamelogs(
            self.order_dated_gamelogs(season_numeric.into(), fielding_gamelogs),
            &mut self.fielding_career_games
        );
        let pitching_gamelogs = Self::order_pitching_gamelogs(
            self.order_dated_gamelogs(season_numeric.into(), pitching_gamelogs),
            &mut self.pitching_career_games,
            fip_constant
        );

        let tx = self.conn.transaction().expect("Could not create transaction");
        Self::insert_games(&tx, &batting_gamelogs)?;
        Self::insert_games(&tx, &fielding_gamelogs)?;
        Self::insert_games(&tx, &pitching_gamelogs)?;
        tx.commit().expect("Failed to commit transaction");

        Ok(())
    }

    fn load_player_game_logs(&self, season: &str, game_ids: &HashSet<String>) -> Result<PlayerGameLogs, Box<dyn Error>> {
        // Load boxscores from the event files to get more accurate data.
        let mut batting_gamelogs = Vec::new();
        let mut fielding_gamelogs = Vec::new();
        let mut pitching_gamelogs = Vec::new();
        // Initialize the missing games as all games so the first run will load as many as
        // possible. Subsequent runs will attempt to find any missed from the first loop.
        let mut missing_game_ids = game_ids.clone();
        let mut found_game_ids = HashSet::with_capacity(game_ids.len());

        println!("Loading player game logs from {} season.", season);
        for file_type in [GameFiles::Event, GameFiles::Deduced, GameFiles::BoxScore] {
            let stdout = self.load_player_daily_stats(season, &file_type)?;
            let (b_game_logs, f_game_logs, p_game_logs) = gamelogs_from_daily_stats(io::BufReader::new(stdout));

            for game_log in b_game_logs.into_iter() {
                if missing_game_ids.contains(&game_log.game_id) {
                    batting_gamelogs.push(game_log);
                }
            }

            for game_log in f_game_logs.into_iter() {
                if missing_game_ids.contains(&game_log.game_id) {
                    fielding_gamelogs.push(game_log);
                }
            }

            for game_log in p_game_logs.into_iter() {
                if missing_game_ids.contains(&game_log.game_id) {
                    pitching_gamelogs.push(game_log);
                }
            }

            // Collect all games found from loading the event files and then check the overall list
            // against what was found to detect any missing games.
            found_game_ids.clear();
            for game_log in &pitching_gamelogs {
                found_game_ids.insert(game_log.game_id.clone());
            }

            missing_game_ids.clear();
            for game_id in game_ids {
                if !found_game_ids.contains(game_id) {
                    missing_game_ids.insert(game_id.clone());
                }
            }

            if missing_game_ids.is_empty() {
                break;
            }
            else {
                println!("Missing {} games from {} files. Loading next file type.", missing_game_ids.len(), file_type);
            }
        }

        println!("Final missing games: {}", missing_game_ids.len());

        Ok((batting_gamelogs, fielding_gamelogs, pitching_gamelogs))
    }

    fn load(&mut self, season: &str) -> Result<(), Box<dyn Error>> {
        let game_ids = self.load_team_games(season)?;
        self.load_player_games(season, &game_ids)?;
        Ok(())
    }
}


type DatedPlayerGamelogs<T> = (T, chrono::NaiveDate);


struct BattingSlashLine {
    ab: u16,
    h: u16,
    tb: u16,
    bb: u8,
    hbp: u8,
    sf: u8,
}


// Value scaled so they will work for the league totals.
struct PitcherStats {
    ipouts: u32,
    er: u16,
    hr: u16,
    bb: u16,
    hbp: u16,
    so: u16,
    fip_constant: f32,
}


#[derive(Eq, Hash, PartialEq)]
struct TeamGameLogKey {
    game_id: String,
    team_id: String,
}


struct TeamGameLogValue {
    date: chrono::NaiveDate,
    team_game_number: u16,
}


impl BattingSlashLine {
    fn new() -> Self {
        Self {
            ab: 0,
            h: 0,
            tb: 0,
            bb: 0,
            hbp: 0,
            sf: 0,
        }
    }

    fn clear(&mut self) {
        self.h = 0;
        self.ab = 0;
        self.h = 0;
        self.tb = 0;
        self.bb = 0;
        self.hbp = 0;
        self.sf = 0;
    }

    fn add_gamelog(&mut self, gamelog: &player::BattingGamelog) {
        let h: u16 = gamelog.h.into();
        let ab: u16 = gamelog.ab.into();
        self.h += h;
        self.ab += ab;
        let d: u16 = gamelog.d.unwrap_or(0).into();
        let t: u16 = gamelog.t.unwrap_or(0).into();
        let hr: u16 = gamelog.hr.unwrap_or(0).into();
        // The hits field includes extra-base hits so the game total for each stat includes the
        // number of bases beyond a single.
        self.tb += h + d + t * 2 + hr * 3;
        self.bb += gamelog.bb.unwrap_or(0);
        self.hbp += gamelog.hbp.unwrap_or(0);
        self.sf += gamelog.sf.unwrap_or(0);
    }

    fn slash_line(&self) -> (f32, f32, f32) {
        let h = self.h as f32;
        let ab = self.ab as f32;
        let tb = self.tb as f32;
        let bb = self.bb as f32;
        let hbp = self.hbp as f32;
        let sf = self.sf as f32;

        let avg = if ab > 0.0 {
            h / ab
        }
        else {
            f32::NAN
        };

        let obp = if (ab + bb + hbp + sf) > 0.0 {
            (h + bb + hbp) / (ab + bb + hbp + sf)
        }
        else {
            f32::NAN
        };

        let slg = if ab > 0.0 {
            tb / ab
        }
        else {
            f32::NAN
        };

        (avg, obp, slg)
    }
}


impl PitcherStats {
    /*
    fn new() -> Self {
        Self::new_with_fip(3.20)
    }
    */

    fn new_with_fip(fip_constant: f32) -> Self {
        Self {
            ipouts: 0,
            er: 0,
            hr: 0,
            bb: 0,
            hbp: 0,
            so: 0,
            fip_constant,
        }
    }

    fn clear(&mut self) {
        self.ipouts = 0;
        self.er = 0;
        self.hr = 0;
        self.bb = 0;
        self.hbp = 0;
        self.so = 0;
    }

    fn add_team_gamelog(&mut self, gamelog: &games::GameLog) {
        macro_rules! unwrap_retro_option {
            ($field:expr) => {
                match $field {
                    games::RetrosheetOption::Some(v) => v,
                    _ => return,
                }
            }
        }

        let ipouts: u32 = match gamelog.number_of_outs {
            Some(outs) => outs.into(),
            None => return,
        };
        self.ipouts += ipouts;
        let v_i_er = unwrap_retro_option!(gamelog.visitor_individual_earned_runs);
        let h_i_er = unwrap_retro_option!(gamelog.home_individual_earned_runs);
        let er: u16 = (v_i_er + h_i_er).into();
        self.er += er;
        let v_hr = unwrap_retro_option!(gamelog.visitor_homeruns);
        let h_hr = unwrap_retro_option!(gamelog.home_homeruns);
        let hr: u16 = (v_hr + h_hr).into();
        self.hr += hr;
        let v_bb = unwrap_retro_option!(gamelog.visitor_walks);
        let h_bb = unwrap_retro_option!(gamelog.home_walks);
        let bb: u16 = (v_bb + h_bb).into();
        self.bb += bb;
        let v_hbp = unwrap_retro_option!(gamelog.visitor_hbp);
        let h_hbp = unwrap_retro_option!(gamelog.home_hbp);
        let hbp: u16 = (v_hbp + h_hbp).into();
        self.hbp += hbp;
        let v_so = unwrap_retro_option!(gamelog.visitor_strikeouts);
        let h_so = unwrap_retro_option!(gamelog.home_strikeouts);
        let so: u16 = (v_so + h_so).into();
        self.so += so;
    }

    fn add_gamelog(&mut self, gamelog: &player::PitchingGamelog) {
        let ipouts: u32 = gamelog.ipouts.into();
        self.ipouts += ipouts;
        let er: u16 = gamelog.er.unwrap_or(0).into();
        self.er += er;
        let hr: u16 = gamelog.hr.unwrap_or(0).into();
        self.hr += hr;
        let bb: u16 = gamelog.bb.unwrap_or(0).into();
        self.bb += bb;
        let hbp: u16 = gamelog.hbp.into();
        self.hbp += hbp;
        let so: u16 = gamelog.so.unwrap_or(0).into();
        self.so += so;
    }

    fn era(&self) -> f32 {
        let er = self.er as f32;
        let outs = self.ipouts as f32;
        if outs > 0.0 {
            er * 27.0 / outs
        }
        else if er > 0.0 {
            f32::INFINITY
        }
        else {
            f32::NAN
        }
    }

    fn fip(&self) -> f32 {
        let hr = self.hr as f32;
        let bb = self.bb as f32;
        let hbp = self.hbp as f32;
        let so = self.so as f32;
        let outs = self.ipouts as f32;

        if outs > 0.0 {
            (13.0 * hr + 3.0 * (bb + hbp) - 2.0 * so) / (outs / 3.0) + self.fip_constant
        }
        else if hr > 0.0 || bb > 0.0 || hbp > 0.0 || so > 0.0 {
            f32::INFINITY
        }
        else {
            f32::NAN
        }
    }
}


fn find_game_files(season_dir: &path::Path, file_type: &GameFiles) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(season_dir)? {
        let entry = entry?;
        let path = entry.path();
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        match (&file_type, extension) {
            (GameFiles::Event, "EVA" | "EVN" | "EVR") => {
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
            (GameFiles::Deduced, "EDA" | "EDN" | "EDR") => {
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
            (GameFiles::BoxScore, "EBA" | "EBN" | "EBR") => {
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
            _ => {}
        }
    }

    Ok(files)
}


fn create_internal_tables(conn: &mut Connection) {
    if let Ok(table_exists) = conn.table_exists(Some("main"), "guts") && !table_exists {
        let res = conn.execute(include_str!("../sql/create_guts.sql"), ());
        if let Err(err) = res {
            eprintln!("Creation of guts table failed: {}", err);
        }
    }
}


fn get_fip_constant(conn: &mut Connection, season: u16) -> Result<Option<f32>, Box<dyn Error>> {
    let fip = conn.query_one(
         "SELECT fip_constant FROM guts WHERE season = :season",
        &[(":season", &season)],
        |row| row.get(0)
    )?;

    Ok(fip)
}


fn update_fip_constant(tx: &Transaction, guts: &Guts) -> Result<(), Box<dyn Error>> {
    let insert_sql = String::from(
        "INSERT INTO guts (season, fip_constant) VALUES (:season, :fip_constant)
         ON CONFLICT (season) DO UPDATE SET fip_constant=:fip_constant"
    );

    let mut insert = tx.prepare(&insert_sql)?;
    insert.execute(
        named_params! {
            ":season": &guts.season,
            ":fip_constant": &guts.fip_constant,
        }
    )?;

    Ok(())
}


fn load_people_file(people_csv: &path::Path) -> Result<Vec<Person>, Box<dyn Error>> {
    // Largest people file is a bit over 32k so give some room to grow.
    let mut people = Vec::with_capacity(34000);
    let file = fs::File::open(people_csv)?;
    let mut reader = ReaderBuilder::new().from_reader(file);
    for person in reader.deserialize().flatten() {
        people.push(person);
    }
    Ok(people)
}


fn load_people_files(conn: &mut Connection, register_dir: &path::Path, initialize: bool) {
    let data_dir = register_dir.join("data");

    println!("Preparing to load register");

    if initialize {
        if let Err(err) = conn.execute("DROP TABLE IF EXISTS people", ()) {
            eprintln!("Initialize of people table failed: {}", err);
            return;
        }
        let res = conn.execute(include_str!("../sql/create_people.sql"), ());
        if let Err(err) = res {
            eprintln!("Creation of people table failed: {}", err);
            return;
        }
    }

    let mut paths = Vec::with_capacity(16);
    for entry in data_dir.read_dir().expect("Failed to read register data directory").flatten() {
        let file_name = entry.file_name().into_string();
        if let Ok(file_name) = file_name && file_name.starts_with("people") {
            paths.push(entry.path());
        }
    }

    let insert_sql = String::from(
        "INSERT INTO people VALUES ( :key_person, :key_uuid, :key_mlbam, :key_retro, :key_bbref,
        :key_bbref_minors, :key_fangraphs, :key_npb, :key_sr_nfl, :key_sr_nba, :key_sr_nhl,
        :key_wikidata, :name_last, :name_first, :name_given, :name_suffix, :name_matrilineal,
        :name_nick, :birth_year, :birth_month, :birth_day, :death_year, :death_month, :death_day,
        :pro_played_first, :pro_played_last, :mlb_played_first, :mlb_played_last,
        :col_played_first, :col_played_last, :pro_managed_first, :pro_managed_last,
        :mlb_managed_first, :mlb_managed_last, :col_managed_first, :col_managed_last,
        :pro_umpired_first, :pro_umpired_last, :mlb_umpired_first, :mlb_umpired_last)");

    let tx = conn.transaction().expect("Could not create transaction");
    let mut insert = tx.prepare(&insert_sql).expect("Could not prepare INSERT");
    let mut people_loaded = 0;
    for path in &paths {
        for person in load_people_file(path).expect("Couldn't load people CSV file") {
            insert.execute(
                named_params! {
                    ":key_person": &person.key_person,
                    ":key_uuid": &person.key_uuid,
                    ":key_mlbam": &person.key_mlbam,
                    ":key_retro": &person.key_retro,
                    ":key_bbref": &person.key_bbref,
                    ":key_bbref_minors": &person.key_bbref_minors,
                    ":key_fangraphs": &person.key_fangraphs,
                    ":key_npb": &person.key_npb,
                    ":key_sr_nfl": &person.key_sr_nfl,
                    ":key_sr_nba": &person.key_sr_nba,
                    ":key_sr_nhl": &person.key_sr_nhl,
                    ":key_wikidata": &person.key_wikidata,
                    ":name_last": &person.name_last,
                    ":name_first": &person.name_first,
                    ":name_given": &person.name_given,
                    ":name_suffix": &person.name_suffix,
                    ":name_matrilineal": &person.name_matrilineal,
                    ":name_nick": &person.name_nick,
                    ":birth_year": &person.birth_year,
                    ":birth_month": &person.birth_month,
                    ":birth_day": &person.birth_day,
                    ":death_year": &person.death_year,
                    ":death_month": &person.death_month,
                    ":death_day": &person.death_day,
                    ":pro_played_first": &person.pro_played_first,
                    ":pro_played_last": &person.pro_played_last,
                    ":mlb_played_first": &person.mlb_played_first,
                    ":mlb_played_last": &person.mlb_played_last,
                    ":col_played_first": &person.col_played_first,
                    ":col_played_last": &person.col_played_last,
                    ":pro_managed_first": &person.pro_managed_first,
                    ":pro_managed_last": &person.pro_managed_last,
                    ":mlb_managed_first": &person.mlb_managed_first,
                    ":mlb_managed_last": &person.mlb_managed_last,
                    ":col_managed_first": &person.col_managed_first,
                    ":col_managed_last": &person.col_managed_last,
                    ":pro_umpired_first": &person.pro_umpired_first,
                    ":pro_umpired_last": &person.pro_umpired_last,
                    ":mlb_umpired_first": &person.mlb_umpired_first,
                    ":mlb_umpired_last": &person.mlb_umpired_last
                }
            ).expect("Failed to insert into people table");
            people_loaded += 1;
        }
    }

    drop(insert);

    tx.commit().expect("Failed to commit transaction");


    // Create index after importing data when initializing.
    if initialize {
        conn.execute_batch(
            "
            CREATE INDEX people_retro_idx ON people (key_retro);
            CREATE INDEX people_bbref_idx ON people (key_bbref);
            CREATE INDEX people_fangraphs_idx ON people (key_fangraphs);
            "
        ).expect("Failed to create people indexes");
    }

    println!("Loaded {} register entries", people_loaded);
}


fn find_available_seasons(retrosheet_dir: &path::Path) -> Result<Vec<u16>, Box<dyn Error>> {
    let mut seasons = Vec::new();
    for entry in fs::read_dir(retrosheet_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let file_name = entry.file_name();
            if let Ok(season) = file_name.to_str().unwrap_or("").parse::<u16>() {
                seasons.push(season);
            }
        }
    }
    seasons.sort_unstable();
    Ok(seasons)
}


fn run() -> Result<(), Box<dyn Error>> {
    let args = DatabaseArgs::parse();

    let database = args.database.unwrap_or(path::PathBuf::from("database.db"));
    let mut connection = Connection::open(database)?;
    connection.pragma_update(None, "temp_store", "memory")?;

    // Start with seasons as integers to make filtering and sorting easy.
    let mut seasons = find_available_seasons(&args.retrosheet_dir)?;
    if let Some(start_season) = args.start_season {
        seasons.retain(|season| *season >= start_season);
    }
    if let Some(last_season) = args.last_season {
        seasons.retain(|season| *season <= last_season);
    }
    // Back to strings because it's treated as a path.
    let seasons: Vec<_> = seasons.iter().map(|season| season.to_string()).collect();

    if let Some(register_path) = args.register_dir {
        load_people_files(&mut connection, &register_path, args.init);
    }

    create_internal_tables(&mut connection);

    let mut game_loader = GameLogLoader::new(&mut connection, args.retrosheet_dir.to_owned());
    if args.init {
        game_loader.create_tables()?;
    }
    for season in &seasons {
        game_loader.load(season)?;
    }
    if args.init {
        game_loader.create_indexes()?;
    }

    // If initializing then tables changed and indexes were created. Run PRAGMA optimize to have
    // sqlite optimize its statistics.
    if args.init {
        println!("Optimizing database");
        connection.pragma_update(None, "optimize", "")?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
