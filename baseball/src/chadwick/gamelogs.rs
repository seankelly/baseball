use std::default::Default;
use std::str;

use quick_xml::events::BytesStart;
use serde::Serialize;
use serde_derive::Deserialize;


#[derive(Default, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BattingGamelog {
    pub player_id: String,
    pub game_id: String,
    pub PA: u8,
    pub AB: u8,
    pub R: u8,
    pub H: u8,
    pub D: u8,
    pub T: u8,
    pub HR: u8,
    pub RBI: u8,
    pub RBI2out: u8,
    pub BB: u8,
    pub IBB: u8,
    pub SO: u8,
    pub GIDP: u8,
    pub HBP: u8,
    pub SH: u8,
    pub SF: u8,
    pub SB: u8,
    pub CS: u8,

    pub POS: String,
}


#[derive(Default, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct FieldingGamelog {
    pub player_id: String,
    pub game_id: String,
    pub POS: u8,
    pub O: u8,
    pub PO: u8,
    pub A: u8,
    pub E: u8,
    pub DP: u8,
    pub TP: u8,
    pub BIP: u8,
    pub BF: u8,
}


#[derive(Default, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PitchingGamelog {
    pub player_id: String,
    pub game_id: String,
    pub GS: bool,
    pub CG: bool,
    pub SHO: bool,
    pub GF: bool,
    pub IPouts: u8,
    pub AB: u8,
    pub BF: u8,
    pub H: u8,
    pub R: u8,
    pub ER: u8,
    pub HR: u8,
    pub BB: u8,
    pub IBB: u8,
    pub SO: u8,
    pub WP: u8,
    pub BK: u8,
    pub HBP: u8,
    pub GB: u8,
    pub FB: u8,
    pub P: u8,
    pub S: u8,
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
        let POS = positions.clone();
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
