use std::borrow::Borrow;
use std::default::Default;
use std::error::Error;
use std::fs;
use std::path;
use std::str;

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


#[derive(Default, serde::Serialize)]
#[allow(non_snake_case)]
struct BattingGamelog {
    player_id: String,
    game_id: String,
    PA: u8,
    AB: u8,
    R: u8,
    H: u8,
    D: u8,
    T: u8,
    HR: u8,
    RBI: u8,
    RBI2out: u8,
    BB: u8,
    IBB: u8,
    SO: u8,
    GIDP: u8,
    HBP: u8,
    SH: u8,
    SF: u8,
    SB: u8,
    CS: u8,

    POS: String,
}


#[derive(Default, serde::Serialize)]
#[allow(non_snake_case)]
struct FieldingGamelog {
    player_id: String,
    game_id: String,
    POS: u8,
    O: u8,
    PO: u8,
    A: u8,
    E: u8,
    DP: u8,
    TP: u8,
    BIP: u8,
    BF: u8,
}


#[derive(Default, serde::Serialize)]
#[allow(non_snake_case)]
struct PitchingGamelog {
    player_id: String,
    game_id: String,
    GS: bool,
    CG: bool,
    SHO: bool,
    GF: bool,
    IPouts: u8,
    AB: u8,
    BF: u8,
    H: u8,
    R: u8,
    ER: u8,
    HR: u8,
    BB: u8,
    IBB: u8,
    SO: u8,
    WP: u8,
    BK: u8,
    HBP: u8,
    GB: u8,
    FB: u8,
    P: u8,
    S: u8,
    decision: String,
}


fn attribute_to_u8(attr: &quick_xml::events::attributes::Attribute) -> u8 {
    let attribute = str::from_utf8(attr.value.as_ref());
    u8::from_str_radix(attribute.unwrap_or("0"), 10).unwrap_or(0)
}


fn attribute_to_bool(attr: &quick_xml::events::attributes::Attribute) -> bool {
    let attribute = str::from_utf8(attr.value.as_ref());
    match attribute.unwrap_or("0") {
        "0" => false,
        "1" => true,
        _ => false,
    }
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


impl BattingGamelog {
    fn from_element(element: &BytesStart, player: &Player, game: &Boxscore) -> Self {
        let game_id = game.game_id.clone();
        let player_id = player.player_id.clone();
        let POS = player.positions.clone();
        let mut batting = Self {
            player_id,
            game_id,
            POS,
            ..Default::default()
        };

        for attribute in element.attributes() {
            match attribute {
                Ok(attr) => {
                    match attr.key.local_name().as_ref() {
                        b"ab" => { batting.AB = attribute_to_u8(&attr); }
                        b"r" => { batting.R = attribute_to_u8(&attr); }
                        b"h" => { batting.H = attribute_to_u8(&attr); }
                        b"d" => { batting.D = attribute_to_u8(&attr); }
                        b"t" => { batting.T = attribute_to_u8(&attr); }
                        b"hr" => { batting.HR = attribute_to_u8(&attr); }
                        b"bi" => { batting.RBI = attribute_to_u8(&attr); }
                        b"bi2out" => { batting.RBI2out = attribute_to_u8(&attr); }
                        b"bb" => { batting.BB = attribute_to_u8(&attr); }
                        b"ibb" => { batting.IBB = attribute_to_u8(&attr); }
                        b"so" => { batting.SO = attribute_to_u8(&attr); }
                        b"gdp" => { batting.GIDP = attribute_to_u8(&attr); }
                        b"hp" => { batting.HBP = attribute_to_u8(&attr); }
                        b"sh" => { batting.SH = attribute_to_u8(&attr); }
                        b"sf" => { batting.SF = attribute_to_u8(&attr); }
                        b"sb" => { batting.SB = attribute_to_u8(&attr); }
                        b"cs" => { batting.CS = attribute_to_u8(&attr); }
                        _ => { }
                    }
                }
                Err(_e) => {}
            }
        }

        // Calculate plate appearances ahead of time for simplicity.
        batting.PA = batting.AB + batting.BB + batting.HBP + batting.SF + batting.SH;

        return batting;
    }
}


impl FieldingGamelog {
    fn from_element(element: &BytesStart, player: &Player, game: &Boxscore) -> Self {
        let player_id = player.player_id.clone();
        let game_id = game.game_id.clone();
        let mut fielding = Self {
            player_id,
            game_id,
            ..Default::default()
        };

        for attribute in element.attributes() {
            match attribute {
                Ok(attr) => {
                    match attr.key.local_name().as_ref() {
                        b"pos" => { fielding.POS = attribute_to_u8(&attr); }
                        b"outs" => { fielding.O = attribute_to_u8(&attr); }
                        b"po" => { fielding.PO = attribute_to_u8(&attr); }
                        b"a" => { fielding.A = attribute_to_u8(&attr); }
                        b"e" => { fielding.E = attribute_to_u8(&attr); }
                        b"dp" => { fielding.DP = attribute_to_u8(&attr); }
                        b"tp" => { fielding.TP = attribute_to_u8(&attr); }
                        b"bip" => { fielding.BIP = attribute_to_u8(&attr); }
                        b"bf" => { fielding.BF = attribute_to_u8(&attr); }
                        _ => { }
                    }
                }
                Err(_e) => {}
            }
        }

        return fielding;
    }
}


impl PitchingGamelog {
    fn from_element(element: &BytesStart, game: &Boxscore) -> Self {
        let game_id = game.game_id.clone();
        let mut pitching = Self {
            game_id,
            ..Default::default()
        };

        for attribute in element.attributes() {
            match attribute {
                Ok(attr) => {
                    match attr.key.local_name().as_ref() {
                        b"id" => {
                            let attribute = str::from_utf8(attr.value.as_ref());
                            pitching.player_id = attribute.unwrap_or("").to_owned();
                        }
                        b"gs" => { pitching.GS = attribute_to_bool(&attr); }
                        b"cg" => { pitching.CG = attribute_to_bool(&attr); }
                        b"sho" => { pitching.SHO = attribute_to_bool(&attr); }
                        b"gf" => { pitching.GF = attribute_to_bool(&attr); }
                        b"outs" => { pitching.IPouts = attribute_to_u8(&attr); }
                        b"ab" => { pitching.AB = attribute_to_u8(&attr); }
                        b"bf" => { pitching.BF = attribute_to_u8(&attr); }
                        b"h" => { pitching.H = attribute_to_u8(&attr); }
                        b"r" => { pitching.R = attribute_to_u8(&attr); }
                        b"er" => { pitching.ER = attribute_to_u8(&attr); }
                        b"hr" => { pitching.HR = attribute_to_u8(&attr); }
                        b"bb" => { pitching.BB = attribute_to_u8(&attr); }
                        b"ibb" => { pitching.IBB = attribute_to_u8(&attr); }
                        b"so" => { pitching.SO = attribute_to_u8(&attr); }
                        b"wp" => { pitching.WP = attribute_to_u8(&attr); }
                        b"bk" => { pitching.BK = attribute_to_u8(&attr); }
                        b"hb" => { pitching.HBP = attribute_to_u8(&attr); }
                        b"gb" => { pitching.GB = attribute_to_u8(&attr); }
                        b"fb" => { pitching.FB = attribute_to_u8(&attr); }
                        b"pitch" => { pitching.P = attribute_to_u8(&attr); }
                        b"strike" => { pitching.S = attribute_to_u8(&attr); }
                        b"dec" => {
                            let attribute = str::from_utf8(attr.value.as_ref());
                            pitching.decision = attribute.unwrap_or("").to_owned();
                        }
                        _ => { }
                    }
                }
                Err(_e) => {}
            }
        }

        return pitching;
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
                        let batting = BattingGamelog::from_element(
                            &e,
                            active_player.as_ref().expect("Active player doesn't exist for Batting"),
                            active_game.as_ref().expect("Active game doesn't exist for Batting"));
                        batting_gamelogs.push(batting);
                    }
                    b"pitcher" => {
                        let mut pitcher = PitchingGamelog::from_element(
                            &e,
                            active_game.as_ref().expect("Active game doesn't exist for Pitching"));
                        // Always attempt to map the player's Retrosheet ID to Baseball-Reference.
                        if let Some(bbrefid) = register.map_retro_to_bbref(&pitcher.player_id) {
                            pitcher.player_id = bbrefid.to_owned();
                        }
                        pitching_gamelogs.push(pitcher);
                    }
                    b"fielding" => {
                        let fielding = FieldingGamelog::from_element(
                            &e,
                            active_player.as_ref().expect("Active player doesn't exist for Batting"),
                            active_game.as_ref().expect("Active game doesn't exist for Batting"));
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
