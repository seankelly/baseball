use std::borrow::Borrow;
use std::default::Default;
use std::str;

use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use serde::Serialize;
use serde_derive::Deserialize;


#[derive(Default, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BattingGamelog {
    pub player_id: String,
    pub game_id: String,
    pub pa: u8,
    pub ab: u8,
    pub r: u8,
    pub h: u8,
    pub d: u8,
    pub t: u8,
    pub hr: u8,
    pub rbi: u8,
    pub rbi2out: u8,
    pub bb: u8,
    pub ibb: u8,
    pub so: u8,
    pub gidp: u8,
    pub hbp: u8,
    pub sh: u8,
    pub sf: u8,
    pub sb: u8,
    pub cs: u8,

    pub pos: String,
}


#[derive(Default, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct FieldingGamelog {
    pub player_id: String,
    pub game_id: String,
    pub pos: u8,
    pub o: u8,
    pub po: u8,
    pub a: u8,
    pub e: u8,
    pub dp: u8,
    pub tp: u8,
    pub bip: u8,
    pub bf: u8,
}


#[derive(Default, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PitchingGamelog {
    pub player_id: String,
    pub game_id: String,
    pub gs: bool,
    pub cg: bool,
    pub sho: bool,
    pub gf: bool,
    pub ipouts: u8,
    pub ab: u8,
    pub bf: u8,
    pub h: u8,
    pub r: u8,
    pub er: u8,
    pub hr: u8,
    pub bb: u8,
    pub ibb: u8,
    pub so: u8,
    pub wp: u8,
    pub bk: u8,
    pub hbp: u8,
    pub gb: u8,
    pub fb: u8,
    pub p: u8,
    pub s: u8,
    pub decision: String,
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


impl BattingGamelog {
    pub fn from_element(element: &BytesStart, game_id: &str, player_id: &str, positions: &String) -> Self {
        let game_id = game_id.to_owned();
        let player_id = player_id.to_owned();
        let pos = positions.clone();
        let mut batting = Self {
            player_id,
            game_id,
            pos,
            ..Default::default()
        };

        for attribute in element.attributes() {
            match attribute {
                Ok(attr) => {
                    match attr.key.local_name().as_ref() {
                        b"ab" => { batting.ab = attribute_to_u8(&attr); }
                        b"r" => { batting.r = attribute_to_u8(&attr); }
                        b"h" => { batting.h = attribute_to_u8(&attr); }
                        b"d" => { batting.d = attribute_to_u8(&attr); }
                        b"t" => { batting.t = attribute_to_u8(&attr); }
                        b"hr" => { batting.hr = attribute_to_u8(&attr); }
                        b"bi" => { batting.rbi = attribute_to_u8(&attr); }
                        b"bi2out" => { batting.rbi2out = attribute_to_u8(&attr); }
                        b"bb" => { batting.bb = attribute_to_u8(&attr); }
                        b"ibb" => { batting.ibb = attribute_to_u8(&attr); }
                        b"so" => { batting.so = attribute_to_u8(&attr); }
                        b"gdp" => { batting.gidp = attribute_to_u8(&attr); }
                        b"hp" => { batting.hbp = attribute_to_u8(&attr); }
                        b"sh" => { batting.sh = attribute_to_u8(&attr); }
                        b"sf" => { batting.sf = attribute_to_u8(&attr); }
                        b"sb" => { batting.sb = attribute_to_u8(&attr); }
                        b"cs" => { batting.cs = attribute_to_u8(&attr); }
                        _ => { }
                    }
                }
                Err(_e) => {}
            }
        }

        // Calculate plate appearances ahead of time for simplicity.
        batting.pa = batting.ab + batting.bb + batting.hbp + batting.sf + batting.sh;

        return batting;
    }
}


impl FieldingGamelog {
    pub fn from_element(element: &BytesStart, game_id: &str, player_id: &str) -> Self {
        let player_id = player_id.to_owned();
        let game_id = game_id.to_owned();
        let mut fielding = Self {
            player_id,
            game_id,
            ..Default::default()
        };

        for attribute in element.attributes() {
            match attribute {
                Ok(attr) => {
                    match attr.key.local_name().as_ref() {
                        b"pos" => { fielding.pos = attribute_to_u8(&attr); }
                        b"outs" => { fielding.o = attribute_to_u8(&attr); }
                        b"po" => { fielding.po = attribute_to_u8(&attr); }
                        b"a" => { fielding.a = attribute_to_u8(&attr); }
                        b"e" => { fielding.e = attribute_to_u8(&attr); }
                        b"dp" => { fielding.dp = attribute_to_u8(&attr); }
                        b"tp" => { fielding.tp = attribute_to_u8(&attr); }
                        b"bip" => { fielding.bip = attribute_to_u8(&attr); }
                        b"bf" => { fielding.bf = attribute_to_u8(&attr); }
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
    pub fn from_element(element: &BytesStart, game_id: &str) -> Self {
        let game_id = game_id.to_owned();
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
                        b"gs" => { pitching.gs = attribute_to_bool(&attr); }
                        b"cg" => { pitching.cg = attribute_to_bool(&attr); }
                        b"sho" => { pitching.sho = attribute_to_bool(&attr); }
                        b"gf" => { pitching.gf = attribute_to_bool(&attr); }
                        b"outs" => { pitching.ipouts = attribute_to_u8(&attr); }
                        b"ab" => { pitching.ab = attribute_to_u8(&attr); }
                        b"bf" => { pitching.bf = attribute_to_u8(&attr); }
                        b"h" => { pitching.h = attribute_to_u8(&attr); }
                        b"r" => { pitching.r = attribute_to_u8(&attr); }
                        b"er" => { pitching.er = attribute_to_u8(&attr); }
                        b"hr" => { pitching.hr = attribute_to_u8(&attr); }
                        b"bb" => { pitching.bb = attribute_to_u8(&attr); }
                        b"ibb" => { pitching.ibb = attribute_to_u8(&attr); }
                        b"so" => { pitching.so = attribute_to_u8(&attr); }
                        b"wp" => { pitching.wp = attribute_to_u8(&attr); }
                        b"bk" => { pitching.bk = attribute_to_u8(&attr); }
                        b"hb" => { pitching.hbp = attribute_to_u8(&attr); }
                        b"gb" => { pitching.gb = attribute_to_u8(&attr); }
                        b"fb" => { pitching.fb = attribute_to_u8(&attr); }
                        b"pitch" => { pitching.p = attribute_to_u8(&attr); }
                        b"strike" => { pitching.s = attribute_to_u8(&attr); }
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


fn find_game_id(element: &BytesStart) -> String {
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

    game_id
}


fn find_player_info(element: &BytesStart) -> (String, String) {
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

    (player_id, positions)
}


pub fn gamelogs_from_boxscores(boxsore_xml: &str) -> (Vec<BattingGamelog>, Vec<FieldingGamelog>, Vec<PitchingGamelog>) {
    let mut batting_gamelogs = Vec::new();
    let mut pitching_gamelogs = Vec::new();
    let mut fielding_gamelogs = Vec::new();

    let mut reader = Reader::from_str(boxsore_xml);
    reader.config_mut().trim_text(true);
    let mut buffer = Vec::new();

    let mut active_player = None;
    let mut active_player_pos = None;
    let mut active_game = None;
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"boxscore" => {
                        active_game = Some(find_game_id(&e));
                    }
                    b"player" => {
                        let (player_id, positions) = find_player_info(&e);
                        active_player = Some(player_id);
                        active_player_pos = Some(positions);
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
                        let positions = active_player_pos.as_ref().expect("Active player positions don't exist for Batting");
                        let game = active_game.as_ref().expect("Active game doesn't exist for Batting");
                        let batting = BattingGamelog::from_element(
                            &e, &game, &player, &positions);
                        batting_gamelogs.push(batting);
                    }
                    b"pitcher" => {
                        let game = active_game.as_ref().expect("Active game doesn't exist for Batting");
                        let pitcher = PitchingGamelog::from_element(
                            &e, &game);
                        pitching_gamelogs.push(pitcher);
                    }
                    b"fielding" => {
                        let player = active_player.as_ref().expect("Active player doesn't exist for Batting");
                        let game = active_game.as_ref().expect("Active game doesn't exist for Batting");
                        let fielding = FieldingGamelog::from_element(
                            &e, &game, &player);
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

    (batting_gamelogs, fielding_gamelogs, pitching_gamelogs)
}
