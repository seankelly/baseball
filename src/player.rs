use std::default::Default;
use baseball::chadwick::gamelogs;


pub trait PlayerGamelog {
    fn player_id(&self) -> &str;

    fn game_id(&self) -> &str;

    fn team_id(&self) -> &str;

    fn set_team_game(&mut self, game: u16);
}


#[derive(Default)]
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


#[derive(Default)]
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


#[derive(Default)]
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


impl BattingGamelog {
}


impl PlayerGamelog for BattingGamelog {
    fn player_id(&self) -> &str { &self.player_id }

    fn game_id(&self) -> &str { &self.game_id }

    fn team_id(&self) -> &str { &self.team_id }

    fn set_team_game(&mut self, game: u16) { self.team_game = game; }
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
