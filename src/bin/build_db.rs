use std::error::Error;
use std::fs;
use std::io;
use std::path;
use std::process::Command;

use baseball::register::Person;
use baseball::chadwick::gamelogs::{gamelogs_from_boxscores, BattingGamelog, FieldingGamelog, PitchingGamelog};
use baseball::chadwick::games;

use clap::Parser;
use csv::ReaderBuilder;
use rusqlite::{Connection, Result, Transaction, named_params};


#[derive(Parser)]
struct DatabaseArgs {
    #[arg(short, long)]
    database: Option<path::PathBuf>,

    #[arg(short, long)]
    init: bool,

    #[arg(short = 'G', long)]
    gamelogs: bool,

    #[arg(short, long)]
    games: bool,

    #[arg(short = 'R', long)]
    register_dir: Option<path::PathBuf>,

    #[arg(short = 'r', long)]
    retrosheet_dir: Option<path::PathBuf>,

    seasons: Vec<String>,
}


struct GameLoader<'a> {
    conn: &'a mut Connection,
    retrosheet_dir: path::PathBuf,
}


impl<'a> GameLoader<'a> {
    fn new(conn: &'a mut Connection, retrosheet_dir: path::PathBuf) -> Self {
        Self {
            conn,
            retrosheet_dir
        }
    }

    // SQL interaction section.
    fn create_games_table(&mut self) -> Result<(), Box<dyn Error>> {
        self.conn.execute("DROP TABLE IF EXISTS games", ())?;
        self.conn.execute(
            "CREATE TABLE games (
                game_id TEXT NOT NULL,
                date TEXT,
                start_time TEXT,
                game_number INTEGER,
                dh_used INTEGER,
                day_night TEXT,
                away_team TEXT,
                home_team TEXT,
                park TEXT,
                attendance INTEGER,
                pitch_info INTEGER,
                temperature INTEGER,
                wind_direction INTEGER,
                wind_speed INTEGER,
                field_condition INTEGER,
                precipitation INTEGER,
                sky INTEGER,
                game_time INTEGER,
                innings INTEGER,
                game_type TEXT
            )",
            ()
        )?;

        Ok(())
    }

    fn insert_games(tx: &Transaction, games: &Vec<games::Game>) -> Result<(), Box<dyn Error>> {
        let insert_sql = String::from(
            "INSERT INTO games VALUES (
                :game_id, :date, :start_time, :game_number, :dh_used, :day_night, :away_team,
                :home_team, :park, :attendance, :pitch_info, :temperature, :wind_direction,
                :wind_speed, :field_condition, :precipitation, :sky, :game_time, :innings,
                :game_type)");

        let mut insert = tx.prepare(&insert_sql)?;
        for game in games {
            insert.execute(
                named_params! {
                    ":game_id": &game.game_id,
                    ":date": &game.date,
                    ":start_time": &game.start_time,
                    ":game_number": &game.game_number,
                    ":dh_used": &game.dh_used,
                    ":day_night": &game.day_night,
                    ":away_team": &game.away_team,
                    ":home_team": &game.home_team,
                    ":park": &game.park,
                    ":attendance": &game.attendance,
                    ":pitch_info": &game.pitch_info,
                    ":temperature": &game.temperature,
                    ":wind_direction": &game.wind_direction,
                    ":wind_speed": &game.wind_speed,
                    ":field_condition": &game.field_condition,
                    ":precipitation": &game.precipitation,
                    ":sky": &game.sky,
                    ":game_time": &game.game_time,
                    ":innings": &game.innings,
                    ":game_type": &game.game_type,
                }
            )?;
        }

        Ok(())
    }
    fn load_season_games(&self, season: &String) -> Result<Vec<games::Game>, Box<dyn Error>> {
        let season_dir = self.retrosheet_dir.join(season);
        let mut cwgame = Command::new("cwgame");
        cwgame.args(["-q", "-y", season, "-f", "0-2,4-9,18,25-33,84",]).current_dir(&season_dir);
        cwgame.args(find_boxscore_files(&season_dir)?);
        match cwgame.output() {
            Ok(result) => {
                let game_buffer = io::Cursor::new(result.stdout);
                Ok(games::Game::load(game_buffer))
            }
            Err(err) => {
                Err(Box::new(err))
            }
        }
    }

    fn load(&mut self, seasons: &Vec<String>, initialize: bool) -> Result<(), Box<dyn Error>> {
        if initialize {
            println!("Creating games tables");
            self.create_games_table()?;
        }

        for season in seasons {
            println!("Loading games from {} season", season);
            let games = self.load_season_games(season)?;
            println!("Found {} games", games.len());

            let tx = self.conn.transaction().expect("Could not create transaction");
            Self::insert_games(&tx, &games)?;
            tx.commit().expect("Failed to commit transaction");
        }

        if initialize {
            println!("Creating game indexes");
            self.conn.execute_batch(
                "
                CREATE INDEX games_game_idx ON games (game_id);
                CREATE INDEX games_away_idx ON games (away_team);
                CREATE INDEX games_home_idx ON games (home_team);
                CREATE INDEX games_type_idx ON games (game_type);
                "
            )?;
        }

        Ok(())
    }
}


struct PlayerGamelogs<'a> {
    conn: &'a mut Connection,
    retrosheet_dir: path::PathBuf,
}


impl<'a> PlayerGamelogs<'a> {
    fn new(conn: &'a mut Connection, retrosheet_dir: path::PathBuf) -> Self {
        Self {
            conn,
            retrosheet_dir
        }
    }

    fn create_batting_gamelogs_table(&mut self) -> Result<(), Box<dyn Error>> {
        self.conn.execute("DROP TABLE IF EXISTS batting_gamelogs", ())?;
        self.conn.execute(
            "CREATE TABLE batting_gamelogs (
                player_id TEXT NOT NULL,
                game_id TEXT NOT NULL,
                pa INTEGER,
                ab INTEGER,
                r INTEGER,
                h INTEGER,
                d INTEGER,
                t INTEGER,
                hr INTEGER,
                rbi INTEGER,
                rbi2out INTEGER,
                bb INTEGER,
                ibb INTEGER,
                so INTEGER,
                gidp INTEGER,
                hbp INTEGER,
                sh INTEGER,
                sf INTEGER,
                sb INTEGER,
                cs INTEGER,
                pos TEXT
            )",
            ()
        )?;

        Ok(())
    }

    fn create_fielding_gamelogs_table(&mut self) -> Result<(), Box<dyn Error>> {
        self.conn.execute("DROP TABLE IF EXISTS fielding_gamelogs", ())?;
        self.conn.execute(
            "CREATE TABLE fielding_gamelogs (
                player_id TEXT NOT NULL,
                game_id TEXT NOT NULL,
                pos INTEGER,
                o INTEGER,
                po INTEGER,
                a INTEGER,
                e INTEGER,
                dp INTEGER,
                tp INTEGER,
                bip INTEGER,
                bf INTEGER
            )",
            ()
        )?;

        Ok(())
    }

    fn create_pitching_gamelogs_table(&mut self) -> Result<(), Box<dyn Error>> {
        self.conn.execute("DROP TABLE IF EXISTS pitching_gamelogs", ())?;
        self.conn.execute(
            "CREATE TABLE pitching_gamelogs (
                player_id TEXT NOT NULL,
                game_id TEXT NOT NULL,
                gs INTEGER,
                cg INTEGER,
                sho INTEGER,
                gf INTEGER,
                ipouts INTEGER,
                ab INTEGER,
                bf INTEGER,
                h INTEGER,
                r INTEGER,
                er INTEGER,
                hr INTEGER,
                bb INTEGER,
                ibb INTEGER,
                so INTEGER,
                wp INTEGER,
                bk INTEGER,
                hbp INTEGER,
                gb INTEGER,
                fb INTEGER,
                p INTEGER,
                s INTEGER,
                decision TEXT
            )",
            ()
        )?;

        Ok(())
    }

    fn insert_batting_gamelogs(tx: &Transaction, gamelogs: &Vec<BattingGamelog>) -> Result<(), Box<dyn Error>> {
        let insert_sql = String::from(
            "INSERT INTO batting_gamelogs VALUES (
                :player_id, :game_id, :pa, :ab, :r, :h, :d, :t, :hr, :rbi, :rbi2out, :bb, :ibb,
                :so, :gidp, :hbp, :sh, :sf, :sb, :cs, :pos)");

        let mut insert = tx.prepare(&insert_sql)?;
        for game in gamelogs {
            insert.execute(
                named_params! {
                    ":player_id": &game.player_id,
                    ":game_id": &game.game_id,
                    ":pa": &game.pa,
                    ":ab": &game.ab,
                    ":r": &game.r,
                    ":h": &game.h,
                    ":d": &game.d,
                    ":t": &game.t,
                    ":hr": &game.hr,
                    ":rbi": &game.rbi,
                    ":rbi2out": &game.rbi2out,
                    ":bb": &game.bb,
                    ":ibb": &game.ibb,
                    ":so": &game.so,
                    ":gidp": &game.gidp,
                    ":hbp": &game.hbp,
                    ":sh": &game.sh,
                    ":sf": &game.sf,
                    ":sb": &game.sb,
                    ":cs": &game.cs,
                    ":pos": &game.pos,
                }
            )?;
        }

        Ok(())
    }

    fn insert_fielding_gamelogs(tx: &Transaction, gamelogs: &Vec<FieldingGamelog>) -> Result<(), Box<dyn Error>> {
        let insert_sql = String::from(
            "INSERT INTO fielding_gamelogs VALUES (
                :player_id, :game_id, :pos, :o, :po, :a, :e, :dp, :tp, :bip, :bf)");

        let mut insert = tx.prepare(&insert_sql)?;
        for game in gamelogs {
            insert.execute(
                named_params! {
                    ":player_id": &game.player_id,
                    ":game_id": &game.game_id,
                    ":pos": &game.pos,
                    ":o": &game.o,
                    ":po": &game.po,
                    ":a": &game.a,
                    ":e": &game.e,
                    ":dp": &game.dp,
                    ":tp": &game.tp,
                    ":bip": &game.bip,
                    ":bf": &game.bf,
                }
            )?;
        }

        Ok(())
    }

    fn insert_pitching_gamelogs(tx: &Transaction, gamelogs: &Vec<PitchingGamelog>) -> Result<(), Box<dyn Error>> {
        let insert_sql = String::from(
            "INSERT INTO pitching_gamelogs VALUES (
                :player_id, :game_id, :gs, :cg, :sho, :gf, :ipouts, :ab, :bf, :h, :r, :er, :hr,
                :bb, :ibb, :so, :wp, :bk, :hbp, :gb, :fb, :p, :s, :decision)");

        let mut insert = tx.prepare(&insert_sql)?;
        for game in gamelogs {
            insert.execute(
                named_params! {
                    ":player_id": &game.player_id,
                    ":game_id": &game.game_id,
                    ":gs": &game.gs,
                    ":cg": &game.cg,
                    ":sho": &game.sho,
                    ":gf": &game.gf,
                    ":ipouts": &game.ipouts,
                    ":ab": &game.ab,
                    ":bf": &game.bf,
                    ":h": &game.h,
                    ":r": &game.r,
                    ":er": &game.er,
                    ":hr": &game.hr,
                    ":bb": &game.bb,
                    ":ibb": &game.ibb,
                    ":so": &game.so,
                    ":wp": &game.wp,
                    ":bk": &game.bk,
                    ":hbp": &game.hbp,
                    ":gb": &game.gb,
                    ":fb": &game.fb,
                    ":p": &game.p,
                    ":s": &game.s,
                    ":decision": &game.decision,
                }
            )?;
        }

        Ok(())
    }

    fn load_season_boxscores(&self, season: &String) -> Result<String, Box<dyn Error>> {
        let season_dir = self.retrosheet_dir.join(season);
        let mut cwbox = Command::new("cwbox");
        cwbox.args(["-q", "-y", season, "-X"]).current_dir(&season_dir);
        cwbox.args(find_boxscore_files(&season_dir)?);
        match cwbox.output() {
            Ok(result) => {
                Ok(String::from_utf8(result.stdout)?)
            }
            Err(err) => {
                Err(Box::new(err))
            }
        }
    }

    fn load(&mut self, seasons: &Vec<String>, initialize: bool) -> Result<(), Box<dyn Error>> {
        if initialize {
            println!("Creating gamelog tables");
            self.create_batting_gamelogs_table()?;
            self.create_fielding_gamelogs_table()?;
            self.create_pitching_gamelogs_table()?;
        }

        for season in seasons {
            println!("Loading player game logs from {} season", season);
            let xml = self.load_season_boxscores(&season)?;
            let (batting_gamelogs, fielding_gamelogs, pitching_gamelogs) = gamelogs_from_boxscores(&xml);

            let tx = self.conn.transaction().expect("Could not create transaction");
            Self::insert_batting_gamelogs(&tx, &batting_gamelogs)?;
            Self::insert_fielding_gamelogs(&tx, &fielding_gamelogs)?;
            Self::insert_pitching_gamelogs(&tx, &pitching_gamelogs)?;
            tx.commit().expect("Failed to commit transaction");
        }

        if initialize {
            println!("Creating gamelog indexes");
            self.conn.execute_batch(
                "
                CREATE INDEX batting_gamelogs_player_idx ON batting_gamelogs (player_id);
                CREATE INDEX batting_gamelogs_game_idx ON batting_gamelogs (game_id);
                CREATE INDEX fielding_gamelogs_player_idx ON fielding_gamelogs (player_id);
                CREATE INDEX fielding_gamelogs_game_idx ON fielding_gamelogs (game_id);
                CREATE INDEX pitching_gamelogs_player_idx ON pitching_gamelogs (player_id);
                CREATE INDEX pitching_gamelogs_game_idx ON pitching_gamelogs (game_id);
                "
            )?;
        }

        Ok(())
    }
}


fn find_boxscore_files(season_dir: &path::Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(season_dir)? {
        let entry = entry?;
        let path = entry.path();
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        match extension {
            // Include only the boxscore files because they're simpler and enough to get the stats
            // for every player.
            "EBA" | "EBN" | "EBR" => {
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
            _ => {}
        }
    }

    Ok(files)
}


fn load_people_file(people_csv: &path::Path) -> Result<Vec<Person>, Box<dyn Error>> {
    let mut people = Vec::new();

    let file = fs::File::open(people_csv)?;
    let mut reader = ReaderBuilder::new().from_reader(file);
    for result in reader.deserialize() {
        if let Ok(person) = result {
            people.push(person);
        }
    }

    return Ok(people);
}

fn load_people_files(conn: &mut Connection, register_dir: &path::Path, initialize: bool) {
    let mut people = Vec::new();
    let data_dir = register_dir.join("data");

    println!("Preparing to load register");

    for entry in data_dir.read_dir().expect("Failed to read register data directory") {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().into_string();
            if let Ok(file_name) = file_name {
                if !file_name.starts_with("people") {
                    continue;
                }
            }
            else {
                continue;
            }
            match load_people_file(&entry.path()) {
                Ok(more_people) => {
                    people.extend(more_people);
                }
                Err(_) => {
                }
            }
        }
    }

    if initialize {
        if let Err(err) = conn.execute("DROP TABLE IF EXISTS people", ()) {
            eprintln!("Initialize of people table failed: {}", err);
            return;
        }
        let res = conn.execute(
            "CREATE TABLE people (
                key_person TEXT NOT NULL,
                key_uuid TEXT NOT NULL,
                key_mlbam INTEGER,
                key_retro TEXT,
                key_bbref TEXT,
                key_bbref_minors TEXT,
                key_fangraphs TEXT,
                key_npb TEXT,
                key_sr_nfl TEXT,
                key_sr_nba TEXT,
                key_sr_nhl TEXT,
                key_wikidata TEXT,
                name_last TEXT,
                name_first TEXT,
                name_given TEXT,
                name_suffix TEXT,
                name_matrilineal TEXT,
                name_nick TEXT,
                birth_year INTEGER,
                birth_month INTEGER,
                birth_day INTEGER,
                death_year INTEGER,
                death_month INTEGER,
                death_day INTEGER,
                pro_played_first INTEGER,
                pro_played_last INTEGER,
                mlb_played_first INTEGER,
                mlb_played_last INTEGER,
                col_played_first INTEGER,
                col_played_last INTEGER,
                pro_managed_first INTEGER,
                pro_managed_last INTEGER,
                mlb_managed_first INTEGER,
                mlb_managed_last INTEGER,
                col_managed_first INTEGER,
                col_managed_last INTEGER,
                pro_umpired_first INTEGER,
                pro_umpired_last INTEGER,
                mlb_umpired_first INTEGER,
                mlb_umpired_last INTEGER
            )",
            ()
        );
        if let Err(err) = res {
            eprintln!("Creation of people table failed: {}", err);
            return;
        }
    }

    let insert_sql = String::from(
        "INSERT INTO people VALUES (
            :key_person,
            :key_uuid,
            :key_mlbam,
            :key_retro,
            :key_bbref,
            :key_bbref_minors,
            :key_fangraphs,
            :key_npb,
            :key_sr_nfl,
            :key_sr_nba,
            :key_sr_nhl,
            :key_wikidata,
            :name_last,
            :name_first,
            :name_given,
            :name_suffix,
            :name_matrilineal,
            :name_nick,
            :birth_year,
            :birth_month,
            :birth_day,
            :death_year,
            :death_month,
            :death_day,
            :pro_played_first,
            :pro_played_last,
            :mlb_played_first,
            :mlb_played_last,
            :col_played_first,
            :col_played_last,
            :pro_managed_first,
            :pro_managed_last,
            :mlb_managed_first,
            :mlb_managed_last,
            :col_managed_first,
            :col_managed_last,
            :pro_umpired_first,
            :pro_umpired_last,
            :mlb_umpired_first,
            :mlb_umpired_last)");

    let tx = conn.transaction().expect("Could not create transaction");
    let mut insert = tx.prepare(&insert_sql).expect("Could not prepare INSERT");
    for person in &people {
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

    println!("Loaded {} register entries", people.len());
}


fn run() -> Result<(), Box<dyn Error>> {
    let args = DatabaseArgs::parse();

    let database = args.database.unwrap_or(path::PathBuf::from("database.db"));
    let mut connection = Connection::open(database)?;

    let seasons = args.seasons;

    if let Some(register_path) = args.register_dir {
        load_people_files(&mut connection, &register_path, args.init);
    }

    if args.games {
        if let Some(ref retrosheet_dir) = args.retrosheet_dir {
            let mut game_loader = GameLoader::new(&mut connection, retrosheet_dir.to_owned());
            game_loader.load(&seasons, args.init)?;
        }
        else {
            eprintln!("Cannot load games without retrosheet directory.");
        }
    }

    if args.gamelogs {
        if let Some(ref retrosheet_dir) = args.retrosheet_dir {
            let mut gamelogs = PlayerGamelogs::new(&mut connection, retrosheet_dir.to_owned());
            gamelogs.load(&seasons, args.init)?;
        }
        else {
            eprintln!("Cannot load gamelogs without retrosheet directory.");
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
