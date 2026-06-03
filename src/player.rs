use std::error::Error;

use baseball::chadwick::gamelogs;

use crate::search::CelEval;

use cel::{Context, Program, Value};
use rusqlite::{Row, Statement, named_params};
use rusqlite::types::ValueRef;


pub trait PlayerGamelog {
    fn player_id(&self) -> &str;

    fn game_id(&self) -> &str;

    fn team_id(&self) -> &str;

    fn set_team_game(&mut self, game: u16);
}


#[allow(non_snake_case)]
pub struct BattingGamelog {
    pub player_id: String,
    pub game_id: String,
    pub team_id: String,
    pub career_game: u16,
    pub season_game: u16,
    pub team_game: u16,
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

    // These are cumulative totals from beginning of season through this game.
    pub avg: f32,
    pub obp: f32,
    pub slg: f32,
    pub woba: f32,
    // BABIP covers only this game.
    pub babip: f32,

    pub pos: String,
}


#[allow(non_snake_case)]
pub struct FieldingGamelog {
    pub player_id: String,
    pub game_id: String,
    pub team_id: String,
    pub career_game: u16,
    pub season_game: u16,
    pub team_game: u16,
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


#[allow(non_snake_case)]
pub struct PitchingGamelog {
    pub player_id: String,
    pub game_id: String,
    pub team_id: String,
    pub career_game: u16,
    pub season_game: u16,
    pub team_game: u16,
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
    pub era: f32,
    pub fip: f32,
}


// Use a different method to map sqlite values to f32 because rusqlite won't map SQL NULLs to f32
// NAN.
fn map_sql_real_to_f32(value: ValueRef) -> f32 {
    match value {
        ValueRef::Null => {
            f32::NAN
        }
        ValueRef::Integer(int) => {
            int as f32
        }
        ValueRef::Real(real) => {
            real as f32
        }
        ValueRef::Text(_) => {
            f32::NAN
        }
        ValueRef::Blob(_) => {
            // This shouldn't happen based on the expected data.
            f32::NAN
        }
    }
}


impl BattingGamelog {
    /// Write the struct to the database using named parameters.
    pub fn write_row(&self, statement: &mut Statement) -> Result<usize, rusqlite::Error> {
        statement.execute(
            named_params! {
                ":player_id": &self.player_id,
                ":game_id": &self.game_id,
                ":team_id": &self.team_id,
                ":career_game": &self.career_game,
                ":season_game": &self.season_game,
                ":team_game": &self.team_game,
                ":pa": &self.pa,
                ":ab": &self.ab,
                ":r": &self.r,
                ":h": &self.h,
                ":d": &self.d,
                ":t": &self.t,
                ":hr": &self.hr,
                ":rbi": &self.rbi,
                ":rbi2out": &self.rbi2out,
                ":bb": &self.bb,
                ":ibb": &self.ibb,
                ":so": &self.so,
                ":gidp": &self.gidp,
                ":hbp": &self.hbp,
                ":sh": &self.sh,
                ":sf": &self.sf,
                ":sb": &self.sb,
                ":cs": &self.cs,
                ":avg": &self.avg,
                ":obp": &self.obp,
                ":slg": &self.slg,
                ":woba": &self.woba,
                ":babip": &self.babip,
                ":pos": &self.pos,
            }
        )
    }

    // This could maybe come from a #[derive].
    pub fn column_names<'a>() -> Vec<&'a str> {
        vec![
            "player_id",
            "game_id",
            "team_id",
            "career_game",
            "season_game",
            "team_game",
            "pa",
            "ab",
            "r",
            "h",
            "d",
            "t",
            "hr",
            "rbi",
            "rbi2out",
            "bb",
            "ibb",
            "so",
            "gidp",
            "hbp",
            "sh",
            "sf",
            "sb",
            "cs",
            "bavg",
            "obp",
            "slg",
            "woba",
            "babip",
            "pos",
        ]
    }

    /// Read one row from the database to create the full struct.
    pub fn read_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            player_id: row.get(0)?,
            game_id: row.get(1)?,
            team_id: row.get(2)?,
            career_game: row.get(3)?,
            season_game: row.get(4)?,
            team_game: row.get(5)?,
            pa: row.get(6)?,
            ab: row.get(7)?,
            r: row.get(8)?,
            h: row.get(9)?,
            d: row.get(10)?,
            t: row.get(11)?,
            hr: row.get(12)?,
            rbi: row.get(13)?,
            rbi2out: row.get(14)?,
            bb: row.get(15)?,
            ibb: row.get(16)?,
            so: row.get(17)?,
            gidp: row.get(18)?,
            hbp: row.get(19)?,
            sh: row.get(20)?,
            sf: row.get(21)?,
            sb: row.get(22)?,
            cs: row.get(23)?,
            avg: map_sql_real_to_f32(row.get_ref(24)?),
            obp: map_sql_real_to_f32(row.get_ref(25)?),
            slg: map_sql_real_to_f32(row.get_ref(26)?),
            woba: map_sql_real_to_f32(row.get_ref(27)?),
            babip: map_sql_real_to_f32(row.get_ref(28)?),
            pos: row.get(29)?,
        })
    }
}


impl PlayerGamelog for BattingGamelog {
    fn player_id(&self) -> &str { &self.player_id }

    fn game_id(&self) -> &str { &self.game_id }

    fn team_id(&self) -> &str { &self.team_id }

    fn set_team_game(&mut self, game: u16) { self.team_game = game; }
}


impl CelEval for BattingGamelog {
    fn add_cel_variables(&self, context: &mut Context, variables: &[&str]) -> Result<(), Box<dyn Error>> {
        for name in variables {
            match *name {
                "career_game" => context.add_variable("career_game", self.career_game)?,
                "season_game" => context.add_variable("season_game", self.season_game)?,
                "team_game" => context.add_variable("team_game", self.team_game)?,
                "pa" => context.add_variable("pa", self.pa)?,
                "ab" => context.add_variable("ab", self.ab)?,
                "r" => context.add_variable("r", self.r)?,
                "h" => context.add_variable("h", self.h)?,
                "d" => context.add_variable("d", self.d)?,
                "t" => context.add_variable("t", self.t)?,
                "hr" => context.add_variable("hr", self.hr)?,
                "rbi" => context.add_variable("rbi", self.rbi)?,
                "rbi2out" => context.add_variable("rbi2out", self.rbi2out)?,
                "bb" => context.add_variable("bb", self.bb)?,
                "ibb" => context.add_variable("ibb", self.ibb)?,
                "so" => context.add_variable("so", self.so)?,
                "gidp" => context.add_variable("gidp", self.gidp)?,
                "hbp" => context.add_variable("hbp", self.hbp)?,
                "sh" => context.add_variable("sh", self.sh)?,
                "sf" => context.add_variable("sf", self.sf)?,
                "sb" => context.add_variable("sb", self.sb)?,
                "cs" => context.add_variable("cs", self.cs)?,
                "avg" => context.add_variable("avg", self.avg)?,
                "obp" => context.add_variable("obp", self.obp)?,
                "slg" => context.add_variable("slg", self.slg)?,
                "woba" => context.add_variable("woba", self.woba)?,
                "babip" => context.add_variable("babip", self.babip)?,
                "pos" => context.add_variable("pos", self.pos.clone())?,
                _ => {},
            }
        }

        Ok(())
    }
}


impl From<gamelogs::BattingGamelog> for BattingGamelog {
    fn from(gamelog: gamelogs::BattingGamelog) -> Self {
        // BABIP covers only this game.
        // All of these values can safely fit within the integer part of f32.
        let h = gamelog.h as f32;
        let hr = gamelog.hr as f32;
        let ab = gamelog.ab as f32;
        let so = gamelog.so as f32;
        let sf = gamelog.sf as f32;
        let babip = (h - hr) / (ab - so - hr - sf);
        Self {
            player_id: gamelog.player_id,
            game_id: gamelog.game_id,
            team_id: gamelog.team_id,
            career_game: 0,
            season_game: 0,
            team_game: 0,
            pa: gamelog.pa,
            ab: gamelog.ab,
            r: gamelog.r,
            h: gamelog.h,
            d: gamelog.d,
            t: gamelog.t,
            hr: gamelog.hr,
            rbi: gamelog.rbi,
            rbi2out: gamelog.rbi2out,
            bb: gamelog.bb,
            ibb: gamelog.ibb,
            so: gamelog.so,
            gidp: gamelog.gidp,
            hbp: gamelog.hbp,
            sh: gamelog.sh,
            sf: gamelog.sf,
            sb: gamelog.sb,
            cs: gamelog.cs,
            // These are cumulative totals from beginning of season through this game.
            avg: 0.0,
            obp: 0.0,
            slg: 0.0,
            woba: 0.0,
            babip,
            pos: gamelog.pos,
        }
    }
}


impl FieldingGamelog {
    /// Write the struct to the database using named parameters.
    pub fn write_row(&self, statement: &mut Statement) -> Result<usize, rusqlite::Error> {
        statement.execute(
            named_params! {
                ":player_id": &self.player_id,
                ":game_id": &self.game_id,
                ":team_id": &self.team_id,
                ":career_game": &self.career_game,
                ":season_game": &self.season_game,
                ":team_game": &self.team_game,
                ":pos": &self.pos,
                ":o": &self.o,
                ":po": &self.po,
                ":a": &self.a,
                ":e": &self.e,
                ":dp": &self.dp,
                ":tp": &self.tp,
                ":bip": &self.bip,
                ":bf": &self.bf,
            }
        )
    }
}


impl PlayerGamelog for FieldingGamelog {
    fn player_id(&self) -> &str { &self.player_id }

    fn game_id(&self) -> &str { &self.game_id }

    fn team_id(&self) -> &str { &self.team_id }

    fn set_team_game(&mut self, game: u16) { self.team_game = game; }
}


impl From<gamelogs::FieldingGamelog> for FieldingGamelog {
    fn from(gamelog: gamelogs::FieldingGamelog) -> Self {
        Self {
            player_id: gamelog.player_id,
            game_id: gamelog.game_id,
            team_id: gamelog.team_id,
            career_game: 0,
            season_game: 0,
            team_game: 0,
            pos: gamelog.pos,
            o: gamelog.o,
            po: gamelog.po,
            a: gamelog.a,
            e: gamelog.e,
            dp: gamelog.dp,
            tp: gamelog.tp,
            bip: gamelog.bip,
            bf: gamelog.bf,
        }
    }
}


impl PitchingGamelog {
    /// Write the struct to the database using named parameters.
    pub fn write_row(&self, statement: &mut Statement) -> Result<usize, rusqlite::Error> {
        statement.execute(
            named_params! {
                ":player_id": &self.player_id,
                ":game_id": &self.game_id,
                ":team_id": &self.team_id,
                ":career_game": &self.career_game,
                ":season_game": &self.season_game,
                ":team_game": &self.team_game,
                ":gs": &self.gs,
                ":cg": &self.cg,
                ":sho": &self.sho,
                ":gf": &self.gf,
                ":ipouts": &self.ipouts,
                ":ab": &self.ab,
                ":bf": &self.bf,
                ":h": &self.h,
                ":r": &self.r,
                ":er": &self.er,
                ":hr": &self.hr,
                ":bb": &self.bb,
                ":ibb": &self.ibb,
                ":so": &self.so,
                ":wp": &self.wp,
                ":bk": &self.bk,
                ":hbp": &self.hbp,
                ":gb": &self.gb,
                ":fb": &self.fb,
                ":p": &self.p,
                ":s": &self.s,
                ":decision": &self.decision,
                ":era": &self.era,
                ":fip": &self.fip,
            }
        )
    }
}


impl PlayerGamelog for PitchingGamelog {
    fn player_id(&self) -> &str { &self.player_id }

    fn game_id(&self) -> &str { &self.game_id }

    fn team_id(&self) -> &str { &self.team_id }

    fn set_team_game(&mut self, game: u16) { self.team_game = game; }
}


impl From<gamelogs::PitchingGamelog> for PitchingGamelog {
    fn from(gamelog: gamelogs::PitchingGamelog) -> Self {
        Self {
            player_id: gamelog.player_id,
            game_id: gamelog.game_id,
            team_id: gamelog.team_id,
            career_game: 0,
            season_game: 0,
            team_game: 0,
            gs: gamelog.gs,
            cg: gamelog.cg,
            sho: gamelog.sho,
            gf: gamelog.gf,
            ipouts: gamelog.ipouts,
            ab: gamelog.ab,
            bf: gamelog.bf,
            h: gamelog.h,
            r: gamelog.r,
            er: gamelog.er,
            hr: gamelog.hr,
            bb: gamelog.bb,
            ibb: gamelog.ibb,
            so: gamelog.so,
            wp: gamelog.wp,
            bk: gamelog.bk,
            hbp: gamelog.hbp,
            gb: gamelog.gb,
            fb: gamelog.fb,
            p: gamelog.p,
            s: gamelog.s,
            decision: gamelog.decision,
            era: 0.0,
            fip: 0.0,
        }
    }
}
