use std::error::Error;

use baseball::chadwick::gamelogs;

use crate::database::Sql;
use crate::search::{CelEval, SearchKey};

use cel::Context;
use rusqlite::{Row, Statement, Transaction, named_params};
use rusqlite::types::ValueRef;


pub trait PlayerGamelog {
    fn player_id(&self) -> &str;

    fn game_id(&self) -> &str;

    fn team_id(&self) -> &str;

    fn career_game(&self) -> u16;

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


impl PlayerGamelog for BattingGamelog {
    fn player_id(&self) -> &str { &self.player_id }

    fn game_id(&self) -> &str { &self.game_id }

    fn team_id(&self) -> &str { &self.team_id }

    fn career_game(&self) -> u16 { self.career_game }

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

    fn check_cel_variables(variables: &[&str]) -> bool {
        for name in variables {
            match *name {
                "career_game" => {},
                "season_game" => {},
                "team_game" => {},
                "pa" => {},
                "ab" => {},
                "r" => {},
                "h" => {},
                "d" => {},
                "t" => {},
                "hr" => {},
                "rbi" => {},
                "rbi2out" => {},
                "bb" => {},
                "ibb" => {},
                "so" => {},
                "gidp" => {},
                "hbp" => {},
                "sh" => {},
                "sf" => {},
                "sb" => {},
                "cs" => {},
                "avg" => {},
                "obp" => {},
                "slg" => {},
                "woba" => {},
                "babip" => {},
                "pos" => {},
                _ => return false,
            }
        }

        true
    }
}


impl SearchKey for BattingGamelog {
    fn id(&self) -> &str { &self.game_id }

    fn subject_id(&self) -> &str { &self.player_id }

    fn order(&self, career: bool) -> u16 {
        if career {
            self.career_game
        }
        else {
            self.team_game
        }
    }
}


impl Sql for BattingGamelog {
    fn create_table(tx: &mut Transaction) -> Result<(), Box<dyn Error>> {
        tx.execute("DROP TABLE IF EXISTS batting_gamelogs", ())?;
        tx.execute(include_str!("sql/create_batting_gamelogs.sql"), ())?;
        Ok(())
    }

    fn table_name<'a>() -> &'a str { "batting_gamelogs" }

    /// Read one row from the database to create the full struct.
    fn read_row(row: &Row, offset: usize) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            player_id: row.get(offset)?,
            game_id: row.get(offset + 1)?,
            team_id: row.get(offset + 2)?,
            career_game: row.get(offset + 3)?,
            season_game: row.get(offset + 4)?,
            team_game: row.get(offset + 5)?,
            pa: row.get(offset + 6)?,
            ab: row.get(offset + 7)?,
            r: row.get(offset + 8)?,
            h: row.get(offset + 9)?,
            d: row.get(offset + 10)?,
            t: row.get(offset + 11)?,
            hr: row.get(offset + 12)?,
            rbi: row.get(offset + 13)?,
            rbi2out: row.get(offset + 14)?,
            bb: row.get(offset + 15)?,
            ibb: row.get(offset + 16)?,
            so: row.get(offset + 17)?,
            gidp: row.get(offset + 18)?,
            hbp: row.get(offset + 19)?,
            sh: row.get(offset + 20)?,
            sf: row.get(offset + 21)?,
            sb: row.get(offset + 22)?,
            cs: row.get(offset + 23)?,
            avg: map_sql_real_to_f32(row.get_ref(offset + 24)?),
            obp: map_sql_real_to_f32(row.get_ref(offset + 25)?),
            slg: map_sql_real_to_f32(row.get_ref(offset + 26)?),
            woba: map_sql_real_to_f32(row.get_ref(offset + 27)?),
            babip: map_sql_real_to_f32(row.get_ref(offset + 28)?),
            pos: row.get(offset + 29)?,
        })
    }

    /// Write the struct to the database using named parameters.
    fn write_row(&self, statement: &mut Statement) -> Result<usize, rusqlite::Error> {
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
                // "avg" is an invalid column name so it's named "bavg" in the SQL.
                ":bavg": &self.avg,
                ":obp": &self.obp,
                ":slg": &self.slg,
                ":woba": &self.woba,
                ":babip": &self.babip,
                ":pos": &self.pos,
            }
        )
    }

    // This could maybe come from a #[derive].
    fn column_names<'a>() -> Vec<&'a str> {
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


impl PlayerGamelog for FieldingGamelog {
    fn player_id(&self) -> &str { &self.player_id }

    fn game_id(&self) -> &str { &self.game_id }

    fn team_id(&self) -> &str { &self.team_id }

    fn career_game(&self) -> u16 { self.career_game }

    fn set_team_game(&mut self, game: u16) { self.team_game = game; }
}


impl CelEval for FieldingGamelog {
    fn add_cel_variables(&self, context: &mut Context, variables: &[&str]) -> Result<(), Box<dyn Error>> {
        for name in variables {
            match *name {
                "career_game" => context.add_variable("career_game", self.career_game)?,
                "season_game" => context.add_variable("season_game", self.season_game)?,
                "team_game" => context.add_variable("team_game", self.team_game)?,
                "pos" => context.add_variable("pos", self.pos)?,
                "o" => context.add_variable("o", self.o)?,
                "po" => context.add_variable("po", self.po)?,
                "a" => context.add_variable("a", self.a)?,
                "e" => context.add_variable("e", self.e)?,
                "dp" => context.add_variable("dp", self.dp)?,
                "tp" => context.add_variable("tp", self.tp)?,
                "bip" => context.add_variable("bip", self.bip)?,
                "bf" => context.add_variable("bf", self.bf)?,
                _ => {},
            }
        }

        Ok(())
    }

    fn check_cel_variables(variables: &[&str]) -> bool {
        for name in variables {
            match *name {
                "career_game" => {},
                "season_game" => {},
                "team_game" => {},
                "pos" => {},
                "o" => {},
                "po" => {},
                "a" => {},
                "e" => {},
                "dp" => {},
                "tp" => {},
                "bip" => {},
                "bf" => {},
                _ => return false,
            }
        }

        true
    }
}


impl SearchKey for FieldingGamelog {
    fn id(&self) -> &str { &self.game_id }

    fn subject_id(&self) -> &str { &self.player_id }

    fn order(&self, career: bool) -> u16 {
        if career {
            self.career_game
        }
        else {
            self.team_game
        }
    }
}


impl Sql for FieldingGamelog {
    fn create_table(tx: &mut Transaction) -> Result<(), Box<dyn Error>> {
        tx.execute("DROP TABLE IF EXISTS fielding_gamelogs", ())?;
        tx.execute(include_str!("sql/create_fielding_gamelogs.sql"), ())?;
        Ok(())
    }

    fn table_name<'a>() -> &'a str { "fielding_gamelogs" }

    fn read_row(row: &Row, offset: usize) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            player_id: row.get(offset)?,
            game_id: row.get(offset + 1)?,
            team_id: row.get(offset + 2)?,
            career_game: row.get(offset + 3)?,
            season_game: row.get(offset + 4)?,
            team_game: row.get(offset + 5)?,
            pos: row.get(offset + 6)?,
            o: row.get(offset + 7)?,
            po: row.get(offset + 8)?,
            a: row.get(offset + 9)?,
            e: row.get(offset + 10)?,
            dp: row.get(offset + 11)?,
            tp: row.get(offset + 12)?,
            bip: row.get(offset + 13)?,
            bf: row.get(offset + 14)?,
        })
    }

    /// Write the struct to the database using named parameters.
    fn write_row(&self, statement: &mut Statement) -> Result<usize, rusqlite::Error> {
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

    fn column_names<'a>() -> Vec<&'a str> {
        vec![
            "player_id",
            "game_id",
            "team_id",
            "career_game",
            "season_game",
            "team_game",
            "pos",
            "o",
            "po",
            "a",
            "e",
            "dp",
            "tp",
            "bip",
            "bf",
        ]
    }
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


impl PlayerGamelog for PitchingGamelog {
    fn player_id(&self) -> &str { &self.player_id }

    fn game_id(&self) -> &str { &self.game_id }

    fn team_id(&self) -> &str { &self.team_id }

    fn career_game(&self) -> u16 { self.career_game }

    fn set_team_game(&mut self, game: u16) { self.team_game = game; }
}


impl CelEval for PitchingGamelog {
    fn add_cel_variables(&self, context: &mut Context, variables: &[&str]) -> Result<(), Box<dyn Error>> {
        for name in variables {
            match *name {
                "career_game" => context.add_variable("career_game", self.career_game)?,
                "season_game" => context.add_variable("season_game", self.season_game)?,
                "team_game" => context.add_variable("team_game", self.team_game)?,
                "gs" => context.add_variable("gs", self.gs)?,
                "cg" => context.add_variable("cg", self.cg)?,
                "sho" => context.add_variable("sho", self.sho)?,
                "gf" => context.add_variable("gf", self.gf)?,
                "ipouts" => context.add_variable("ipouts", self.ipouts)?,
                "ab" => context.add_variable("ab", self.ab)?,
                "bf" => context.add_variable("bf", self.bf)?,
                "h" => context.add_variable("h", self.h)?,
                "r" => context.add_variable("r", self.r)?,
                "er" => context.add_variable("er", self.er)?,
                "hr" => context.add_variable("hr", self.hr)?,
                "bb" => context.add_variable("bb", self.bb)?,
                "ibb" => context.add_variable("ibb", self.ibb)?,
                "so" => context.add_variable("so", self.so)?,
                "wp" => context.add_variable("wp", self.wp)?,
                "bk" => context.add_variable("bk", self.bk)?,
                "hbp" => context.add_variable("hbp", self.hbp)?,
                "gb" => context.add_variable("gb", self.gb)?,
                "fb" => context.add_variable("fb", self.fb)?,
                "p" => context.add_variable("p", self.p)?,
                "s" => context.add_variable("s", self.s)?,
                "decision" => context.add_variable("decision", self.decision.clone())?,
                "era" => context.add_variable("era", self.era)?,
                "fip" => context.add_variable("fip", self.fip)?,
                _ => {},
            }
        }

        Ok(())
    }

    fn check_cel_variables(variables: &[&str]) -> bool {
        for name in variables {
            match *name {
                "career_game" => {},
                "season_game" => {},
                "team_game" => {},
                "gs" => {},
                "cg" => {},
                "sho" => {},
                "gf" => {},
                "ipouts" => {},
                "ab" => {},
                "bf" => {},
                "h" => {},
                "r" => {},
                "er" => {},
                "hr" => {},
                "bb" => {},
                "ibb" => {},
                "so" => {},
                "wp" => {},
                "bk" => {},
                "hbp" => {},
                "gb" => {},
                "fb" => {},
                "p" => {},
                "s" => {},
                "decision" => {},
                "era" => {},
                "fip" => {},
                _ => return false,
            }
        }

        true
    }
}


impl SearchKey for PitchingGamelog {
    fn id(&self) -> &str { &self.game_id }

    fn subject_id(&self) -> &str { &self.player_id }

    fn order(&self, career: bool) -> u16 {
        if career {
            self.career_game
        }
        else {
            self.team_game
        }
    }
}


impl Sql for PitchingGamelog {
    fn create_table(tx: &mut Transaction) -> Result<(), Box<dyn Error>> {
        tx.execute("DROP TABLE IF EXISTS pitching_gamelogs", ())?;
        tx.execute(include_str!("sql/create_pitching_gamelogs.sql"), ())?;
        Ok(())
    }

    fn table_name<'a>() -> &'a str { "pitching_gamelogs" }

    fn read_row(row: &Row, offset: usize) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            player_id: row.get(offset)?,
            game_id: row.get(offset + 1)?,
            team_id: row.get(offset + 2)?,
            career_game: row.get(offset + 3)?,
            season_game: row.get(offset + 4)?,
            team_game: row.get(offset + 5)?,
            gs: row.get(offset + 6)?,
            cg: row.get(offset + 7)?,
            sho: row.get(offset + 8)?,
            gf: row.get(offset + 9)?,
            ipouts: row.get(offset + 10)?,
            ab: row.get(offset + 11)?,
            bf: row.get(offset + 12)?,
            h: row.get(offset + 13)?,
            r: row.get(offset + 14)?,
            er: row.get(offset + 15)?,
            hr: row.get(offset + 16)?,
            bb: row.get(offset + 17)?,
            ibb: row.get(offset + 18)?,
            so: row.get(offset + 19)?,
            wp: row.get(offset + 20)?,
            bk: row.get(offset + 21)?,
            hbp: row.get(offset + 22)?,
            gb: row.get(offset + 23)?,
            fb: row.get(offset + 24)?,
            p: row.get(offset + 25)?,
            s: row.get(offset + 26)?,
            decision: row.get(offset + 27)?,
            era: map_sql_real_to_f32(row.get_ref(offset + 28)?),
            fip: map_sql_real_to_f32(row.get_ref(offset + 29)?),
        })
    }

    /// Write the struct to the database using named parameters.
    fn write_row(&self, statement: &mut Statement) -> Result<usize, rusqlite::Error> {
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

    fn column_names<'a>() -> Vec<&'a str> {
        vec![
            "player_id",
            "game_id",
            "team_id",
            "career_game",
            "season_game",
            "team_game",
            "gs",
            "cg",
            "sho",
            "gf",
            "ipouts",
            "ab",
            "bf",
            "h",
            "r",
            "er",
            "hr",
            "bb",
            "ibb",
            "so",
            "wp",
            "bk",
            "hbp",
            "gb",
            "fb",
            "p",
            "s",
            "decision",
            "era",
            "fip",
        ]
    }
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
