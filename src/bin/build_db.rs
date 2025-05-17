use std::error::Error;
use std::fs;
use std::path;

use baseball::register::Person;

use clap::Parser;
use csv::ReaderBuilder;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use rusqlite::{Connection, Result, named_params};


#[derive(Parser)]
struct DatabaseArgs {
    #[arg(short, long)]
    database: Option<path::PathBuf>,

    #[arg(short, long)]
    init: bool,

    #[arg(short, long)]
    register_dir: Option<path::PathBuf>,
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

    println!("Loaded {} entries", people.len());
}


fn run() -> Result<(), Box<dyn Error>> {
    let args = DatabaseArgs::parse();

    let database = args.database.unwrap_or(path::PathBuf::from("database.db"));
    let mut connection = Connection::open(database)?;

    if let Some(register_path) = args.register_dir {
        load_people_files(&mut connection, &register_path, args.init);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
