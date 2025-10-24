use std::borrow::Borrow;
use std::default::Default;
use std::error::Error;
use std::fs;
use std::path;

use baseball::chadwick::gamelogs::{BattingGamelog, FieldingGamelog, PitchingGamelog};
use baseball::register::Register;

use clap::Parser;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;


#[derive(Parser)]
struct GamelogArgs {
    #[arg(short, long)]
    register: Option<path::PathBuf>,

    #[arg(short, long)]
    batting: Option<path::PathBuf>,

    #[arg(short, long)]
    fielding: Option<path::PathBuf>,

    #[arg(short, long)]
    pitching: Option<path::PathBuf>,

    xml_files: Vec<path::PathBuf>,
}


struct Boxscore {
    game_id: String,
}


struct Player {
    player_id: String,
    positions: String,
}


impl Boxscore {
    fn from_element(element: &BytesStart) -> Self {
        let mut game_id = String::new();

        for attribute in element.attributes() {
            match attribute {
                Ok(attr) => {
                    match attr.key.local_name().as_ref() {
                        b"game_id" => {
                            game_id.push_str(String::from_utf8_lossy(attr.value.as_ref()).borrow());
                        }
                        _ => {
                        }
                    }
                }
                Err(_e) => {}
            }
        }

        Self {
            game_id,
        }
    }
}


impl Player {
    fn from_element(element: &BytesStart) -> Self {
        let mut player_id = String::new();
        let mut positions = String::new();

        for attribute in element.attributes() {
            match attribute {
                Ok(attr) => {
                    match attr.key.local_name().as_ref() {
                        b"id" => {
                            player_id.push_str(String::from_utf8_lossy(attr.value.as_ref()).borrow());
                        }
                        b"pos" => {
                            positions.push_str(String::from_utf8_lossy(attr.value.as_ref()).borrow());
                        }
                        _ => {
                        }
                    }
                }
                Err(_e) => {}
            }
        }

        Self {
            player_id,
            positions,
        }
    }
}


fn process_boxscore_xml(xml_file: &path::PathBuf, register: &Register) -> Result<(Vec<BattingGamelog>, Vec<FieldingGamelog>, Vec<PitchingGamelog>), Box<dyn Error>> {
    let mut reader = Reader::from_file(xml_file)?;
    reader.config_mut().trim_text(true);

    let mut buffer = Vec::new();

    let mut active_player = None;
    let mut active_game = None;
    let mut batting_gamelogs = Vec::new();
    let mut pitching_gamelogs = Vec::new();
    let mut fielding_gamelogs = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"boxscore" => {
                        active_game = Some(Boxscore::from_element(&e));
                    }
                    b"player" => {
                        let mut player = Player::from_element(&e);
                        // Always attempt to map the player's Retrosheet ID to Baseball-Reference.
                        if let Some(bbrefid) = register.map_retro_to_bbref(&player.player_id) {
                            player.player_id = bbrefid.to_owned();
                        }
                        active_player = Some(player);
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"boxscore" => {
                        active_game = None;
                    }
                    b"player" => {
                        active_player = None;
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(e)) => {
                match e.name().as_ref() {
                    b"batting" => {
                        let player = active_player.as_ref().expect("Active player doesn't exist for Batting");
                        let game = active_game.as_ref().expect("Active game doesn't exist for Batting");
                        let batting = BattingGamelog::from_element(
                            &e,
                            &player.player_id,
                            &player.positions,
                            &game.game_id);
                        batting_gamelogs.push(batting);
                    }
                    b"pitcher" => {
                        let game = active_game.as_ref().expect("Active game doesn't exist for Batting");
                        let mut pitcher = PitchingGamelog::from_element(
                            &e,
                            &game.game_id);
                        // Always attempt to map the player's Retrosheet ID to Baseball-Reference.
                        if let Some(bbrefid) = register.map_retro_to_bbref(&pitcher.player_id) {
                            pitcher.player_id = bbrefid.to_owned();
                        }
                        pitching_gamelogs.push(pitcher);
                    }
                    b"fielding" => {
                        let player = active_player.as_ref().expect("Active player doesn't exist for Batting");
                        let game = active_game.as_ref().expect("Active game doesn't exist for Batting");
                        let fielding = FieldingGamelog::from_element(
                            &e,
                            &player.player_id,
                            &game.game_id);
                        fielding_gamelogs.push(fielding);
                    }
                    _ => {}
                }
            }
            Err(e) => {
                eprintln!("Error at position {}: {:?}", reader.error_position(), e);
                break;
            }
            _ => {}
        }

        buffer.clear();
    }

    Ok((batting_gamelogs, fielding_gamelogs, pitching_gamelogs))
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


fn run() -> Result<(), Box<dyn Error>> {
    let args = GamelogArgs::parse();

    let mut register = Register::default();
    if let Some(register_path) = args.register {
        let register_file = fs::File::open(register_path)?;
        register.load(register_file);
        register.build_retro_map();
    }

    let batting_gamelog_path = args.batting.unwrap_or(path::PathBuf::from("batting_gamelogs.csv"));
    let mut batting_file = fs::File::create(&batting_gamelog_path)?;
    let fielding_gamelog_path = args.fielding.unwrap_or(path::PathBuf::from("fielding_gamelogs.csv"));
    let mut fielding_file = fs::File::create(&fielding_gamelog_path)?;
    let pitching_gamelog_path = args.pitching.unwrap_or(path::PathBuf::from("pitching_gamelogs.csv"));
    let mut pitching_file = fs::File::create(&pitching_gamelog_path)?;

    for xml_file in args.xml_files {
        let (batting_gamelogs, fielding_gamelogs, pitching_gamelogs) = process_boxscore_xml(&xml_file, &register)?;
        dump_csv(&batting_gamelogs, &mut batting_file)?;
        dump_csv(&fielding_gamelogs, &mut fielding_file)?;
        dump_csv(&pitching_gamelogs, &mut pitching_file)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
