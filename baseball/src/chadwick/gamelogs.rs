use std::default::Default;
use std::io;
use std::str;

use csv::ReaderBuilder;
use serde::Serialize;
use serde_derive::Deserialize;

use crate::chadwick::bool_from_int;


#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Cwdaily {
    pub game_id: String,
    pub game_date: String,
    pub game_number: u8,
    pub appearance_date: String,
    pub team_id: String,
    pub player_id: String,
    pub batting_order_slot: u8,
    pub batting_order_sequence: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub home_team: bool,
    pub opponent_id: String,
    pub park_id: String,
    // Batting fields.
    #[serde(deserialize_with = "bool_from_int")]
    pub b_g: bool,
    pub b_pa: Option<u8>,
    pub b_ab: u8,
    pub b_r: u8,
    pub b_h: u8,
    pub b_tb: Option<u8>,
    pub b_2b: Option<u8>,
    pub b_3b: Option<u8>,
    pub b_hr: Option<u8>,
    /// Grand slams.
    pub b_hr4: Option<u8>,
    pub b_rbi: Option<u8>,
    pub b_gwrbi: Option<u8>,
    pub b_bb: Option<u8>,
    pub b_ibb: Option<u8>,
    pub b_so: Option<u8>,
    pub b_gdp: Option<u8>,
    pub b_hbp: Option<u8>,
    pub b_sh: u8,
    pub b_sf: Option<u8>,
    pub b_sb: Option<u8>,
    pub b_cs: Option<u8>,
    pub b_xi: Option<u8>,
    pub b_g_dh: u8,
    pub b_g_ph: u8,
    pub b_g_pr: u8,
    // Pitching fields.
    #[serde(deserialize_with = "bool_from_int")]
    pub p_g: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub p_gs: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub p_cg: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub p_sho: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub p_gf: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub p_w: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub p_l: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub p_sv: bool,
    pub p_outs: u8,
    pub p_tbf: Option<u8>,
    pub p_ab: Option<u8>,
    pub p_r: u8,
    pub p_er: Option<u8>,
    pub p_h: u8,
    pub p_tb: Option<u8>,
    pub p_2b: Option<u8>,
    pub p_3b: Option<u8>,
    pub p_hr: Option<u8>,
    /// Grand slams.
    pub p_hr4: u8,
    pub p_bb: Option<u8>,
    pub p_ibb: Option<u8>,
    pub p_so: Option<u8>,
    pub p_gdp: Option<u8>,
    pub p_hbp: u8,
    pub p_sh: Option<u8>,
    pub p_sf: Option<u8>,
    pub p_xi: Option<u8>,
    pub p_wp: Option<u8>,
    pub p_bk: u8,
    pub p_ir: Option<u8>,
    pub p_irs: Option<u8>,
    pub p_go: Option<u8>,
    pub p_ao: Option<u8>,
    pub p_pitches: Option<u16>,
    pub p_strikes: Option<u16>,
    // Fielding fields.
    #[serde(deserialize_with = "bool_from_int")]
    pub f_p_g: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_p_gs: bool,
    pub f_p_outs: Option<u8>,
    pub f_p_tc: Option<u8>,
    pub f_p_po: Option<u8>,
    pub f_p_a: Option<u8>,
    pub f_p_e: Option<u8>,
    pub f_p_dp: u8,
    pub f_p_tp: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_c_g: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_c_gs: bool,
    pub f_c_outs: Option<u8>,
    pub f_c_tc: Option<u8>,
    pub f_c_po: Option<u8>,
    pub f_c_a: Option<u8>,
    pub f_c_e: Option<u8>,
    pub f_c_dp: u8,
    pub f_c_tp: u8,
    // Catcher have two additional fields.
    pub f_c_pb: Option<u8>,
    pub f_c_ci: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_1b_g: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_1b_gs: bool,
    pub f_1b_outs: Option<u8>,
    pub f_1b_tc: Option<u8>,
    pub f_1b_po: Option<u8>,
    pub f_1b_a: Option<u8>,
    pub f_1b_e: Option<u8>,
    pub f_1b_dp: u8,
    pub f_1b_tp: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_2b_g: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_2b_gs: bool,
    pub f_2b_outs: Option<u8>,
    pub f_2b_tc: Option<u8>,
    pub f_2b_po: Option<u8>,
    pub f_2b_a: Option<u8>,
    pub f_2b_e: Option<u8>,
    pub f_2b_dp: u8,
    pub f_2b_tp: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_3b_g: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_3b_gs: bool,
    pub f_3b_outs: Option<u8>,
    pub f_3b_tc: Option<u8>,
    pub f_3b_po: Option<u8>,
    pub f_3b_a: Option<u8>,
    pub f_3b_e: Option<u8>,
    pub f_3b_dp: u8,
    pub f_3b_tp: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_ss_g: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_ss_gs: bool,
    pub f_ss_outs: Option<u8>,
    pub f_ss_tc: Option<u8>,
    pub f_ss_po: Option<u8>,
    pub f_ss_a: Option<u8>,
    pub f_ss_e: Option<u8>,
    pub f_ss_dp: u8,
    pub f_ss_tp: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_lf_g: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_lf_gs: bool,
    pub f_lf_outs: Option<u8>,
    pub f_lf_tc: Option<u8>,
    pub f_lf_po: Option<u8>,
    pub f_lf_a: Option<u8>,
    pub f_lf_e: Option<u8>,
    pub f_lf_dp: u8,
    pub f_lf_tp: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_cf_g: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_cf_gs: bool,
    pub f_cf_outs: Option<u8>,
    pub f_cf_tc: Option<u8>,
    pub f_cf_po: Option<u8>,
    pub f_cf_a: Option<u8>,
    pub f_cf_e: Option<u8>,
    pub f_cf_dp: u8,
    pub f_cf_tp: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_rf_g: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub f_rf_gs: bool,
    pub f_rf_outs: Option<u8>,
    pub f_rf_tc: Option<u8>,
    pub f_rf_po: Option<u8>,
    pub f_rf_a: Option<u8>,
    pub f_rf_e: Option<u8>,
    pub f_rf_dp: u8,
    pub f_rf_tp: u8,
}


#[derive(Default, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BattingGamelog {
    pub player_id: String,
    pub game_id: String,
    pub team_id: String,
    pub pa: u8,
    pub ab: u8,
    pub r: u8,
    pub h: u8,
    pub tb: Option<u8>,
    pub d: Option<u8>,
    pub t: Option<u8>,
    pub hr: Option<u8>,
    pub rbi: Option<u8>,
    pub bb: Option<u8>,
    pub ibb: Option<u8>,
    pub so: Option<u8>,
    pub gidp: Option<u8>,
    pub hbp: Option<u8>,
    pub sh: u8,
    pub sf: Option<u8>,
    pub sb: Option<u8>,
    pub cs: Option<u8>,
    pub xi: Option<u8>,

    pub pos: String,
}


#[derive(Default, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct FieldingGamelog {
    pub player_id: String,
    pub game_id: String,
    pub team_id: String,
    pub pos: u8,
    pub gs: bool,
    pub o: Option<u8>,
    pub po: Option<u8>,
    pub tc: Option<u8>,
    pub a: Option<u8>,
    pub e: Option<u8>,
    pub dp: u8,
    pub tp: u8,
    pub pb: Option<u8>,
    pub ci: Option<u8>,
}


#[derive(Default, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PitchingGamelog {
    pub player_id: String,
    pub game_id: String,
    pub team_id: String,
    pub gs: bool,
    pub cg: bool,
    pub sho: bool,
    pub gf: bool,
    pub w: bool,
    pub l: bool,
    pub sv: bool,
    pub ipouts: u8,
    pub ab: Option<u8>,
    pub bf: Option<u8>,
    pub h: u8,
    pub r: u8,
    pub er: Option<u8>,
    pub hr: Option<u8>,
    pub bb: Option<u8>,
    pub ibb: Option<u8>,
    pub so: Option<u8>,
    pub wp: Option<u8>,
    pub bk: u8,
    pub hbp: u8,
    pub go: Option<u8>,
    pub ao: Option<u8>,
    pub p: Option<u16>,
    pub s: Option<u16>,
    pub decision: String,
}


pub type PlayerGameLogs = (Vec<BattingGamelog>, Vec<FieldingGamelog>, Vec<PitchingGamelog>);


impl Cwdaily {
    pub fn split_into_game_logs(&self) -> PlayerGameLogs {
        let mut batting_game_logs = Vec::new();
        let fielding_game_logs = self.fielder_game_logs();
        let mut pitching_game_logs = Vec::new();

        if self.batted() {
            batting_game_logs.push(self.batter_game_log());
        }

        if self.pitched() {
            pitching_game_logs.push(self.pitcher_game_log());
        }

        (batting_game_logs, fielding_game_logs, pitching_game_logs)
    }

    pub fn batter_game_log(&self) -> BattingGamelog {
        BattingGamelog {
            player_id: self.player_id.clone(),
            game_id: self.game_id.clone(),
            team_id: self.team_id.clone(),
            pa: self.b_pa.unwrap_or_else(|| self.estimate_pa()),
            ab: self.b_ab,
            r: self.b_r,
            h: self.b_h,
            tb: self.b_tb,
            d: self.b_2b,
            t: self.b_3b,
            hr: self.b_hr,
            rbi: self.b_rbi,
            bb: self.b_bb,
            ibb: self.b_ibb,
            so: self.b_so,
            gidp: self.b_gdp,
            hbp: self.b_hbp,
            sh: self.b_sh,
            sf: self.b_sf,
            sb: self.b_sb,
            cs: self.b_cs,
            xi: self.b_xi,
            pos: String::new(),
        }
    }

    /// Estimate the plate appearances based on tracked stats.
    fn estimate_pa(&self) -> u8 {
        self.b_ab + self.b_bb.unwrap_or(0) + self.b_hbp.unwrap_or(0) + self.b_sf.unwrap_or(0) + self.b_sh
    }

    pub fn fielder_game_logs(&self) -> Vec<FieldingGamelog> {
        let mut game_logs = Vec::new();

        if self.f_p_g {
            let game_log = FieldingGamelog {
                player_id: self.player_id.clone(),
                game_id: self.game_id.clone(),
                team_id: self.team_id.clone(),
                pos: 1,
                gs: self.f_p_gs,
                o: self.f_p_outs,
                po: self.f_p_po,
                tc: self.f_p_tc,
                a: self.f_p_a,
                e: self.f_p_e,
                dp: self.f_p_dp,
                tp: self.f_p_tp,
                pb: Some(0),
                ci: Some(0),
            };
            game_logs.push(game_log);
        }

        if self.f_c_g {
            let game_log = FieldingGamelog {
                player_id: self.player_id.clone(),
                game_id: self.game_id.clone(),
                team_id: self.team_id.clone(),
                pos: 2,
                gs: self.f_c_gs,
                o: self.f_c_outs,
                po: self.f_c_po,
                tc: self.f_c_tc,
                a: self.f_c_a,
                e: self.f_c_e,
                dp: self.f_c_dp,
                tp: self.f_c_tp,
                pb: self.f_c_pb,
                ci: self.f_c_ci,
            };
            game_logs.push(game_log);
        }

        if self.f_1b_g {
            let game_log = FieldingGamelog {
                player_id: self.player_id.clone(),
                game_id: self.game_id.clone(),
                team_id: self.team_id.clone(),
                pos: 3,
                gs: self.f_1b_gs,
                o: self.f_1b_outs,
                po: self.f_1b_po,
                tc: self.f_1b_tc,
                a: self.f_1b_a,
                e: self.f_1b_e,
                dp: self.f_1b_dp,
                tp: self.f_1b_tp,
                pb: Some(0),
                ci: Some(0),
            };
            game_logs.push(game_log);
        }

        if self.f_2b_g {
            let game_log = FieldingGamelog {
                player_id: self.player_id.clone(),
                game_id: self.game_id.clone(),
                team_id: self.team_id.clone(),
                pos: 4,
                gs: self.f_2b_gs,
                o: self.f_2b_outs,
                po: self.f_2b_po,
                tc: self.f_2b_tc,
                a: self.f_2b_a,
                e: self.f_2b_e,
                dp: self.f_2b_dp,
                tp: self.f_2b_tp,
                pb: Some(0),
                ci: Some(0),
            };
            game_logs.push(game_log);
        }

        if self.f_3b_g {
            let game_log = FieldingGamelog {
                player_id: self.player_id.clone(),
                game_id: self.game_id.clone(),
                team_id: self.team_id.clone(),
                pos: 5,
                gs: self.f_3b_gs,
                o: self.f_3b_outs,
                po: self.f_3b_po,
                tc: self.f_3b_tc,
                a: self.f_3b_a,
                e: self.f_3b_e,
                dp: self.f_3b_dp,
                tp: self.f_3b_tp,
                pb: Some(0),
                ci: Some(0),
            };
            game_logs.push(game_log);
        }

        if self.f_ss_g {
            let game_log = FieldingGamelog {
                player_id: self.player_id.clone(),
                game_id: self.game_id.clone(),
                team_id: self.team_id.clone(),
                pos: 6,
                gs: self.f_ss_gs,
                o: self.f_ss_outs,
                po: self.f_ss_po,
                tc: self.f_ss_tc,
                a: self.f_ss_a,
                e: self.f_ss_e,
                dp: self.f_ss_dp,
                tp: self.f_ss_tp,
                pb: Some(0),
                ci: Some(0),
            };
            game_logs.push(game_log);
        }

        if self.f_lf_g {
            let game_log = FieldingGamelog {
                player_id: self.player_id.clone(),
                game_id: self.game_id.clone(),
                team_id: self.team_id.clone(),
                pos: 1,
                gs: self.f_lf_gs,
                o: self.f_lf_outs,
                po: self.f_lf_po,
                tc: self.f_lf_tc,
                a: self.f_lf_a,
                e: self.f_lf_e,
                dp: self.f_lf_dp,
                tp: self.f_lf_tp,
                pb: Some(0),
                ci: Some(0),
            };
            game_logs.push(game_log);
        }

        if self.f_cf_g {
            let game_log = FieldingGamelog {
                player_id: self.player_id.clone(),
                game_id: self.game_id.clone(),
                team_id: self.team_id.clone(),
                pos: 1,
                gs: self.f_cf_gs,
                o: self.f_cf_outs,
                po: self.f_cf_po,
                tc: self.f_cf_tc,
                a: self.f_cf_a,
                e: self.f_cf_e,
                dp: self.f_cf_dp,
                tp: self.f_cf_tp,
                pb: Some(0),
                ci: Some(0),
            };
            game_logs.push(game_log);
        }

        if self.f_rf_g {
            let game_log = FieldingGamelog {
                player_id: self.player_id.clone(),
                game_id: self.game_id.clone(),
                team_id: self.team_id.clone(),
                pos: 1,
                gs: self.f_rf_gs,
                o: self.f_rf_outs,
                po: self.f_rf_po,
                tc: self.f_rf_tc,
                a: self.f_rf_a,
                e: self.f_rf_e,
                dp: self.f_rf_dp,
                tp: self.f_rf_tp,
                pb: Some(0),
                ci: Some(0),
            };
            game_logs.push(game_log);
        }

        game_logs
    }

    pub fn pitcher_game_log(&self) -> PitchingGamelog {
        PitchingGamelog {
            player_id: self.player_id.clone(),
            game_id: self.game_id.clone(),
            team_id: self.team_id.clone(),
            gs: self.p_gs,
            cg: self.p_cg,
            sho: self.p_sho,
            gf: self.p_gf,
            w: self.p_w,
            l: self.p_l,
            sv: self.p_sv,
            ipouts: self.p_outs,
            ab: self.p_ab,
            bf: self.p_tbf,
            h: self.p_h,
            r: self.p_r,
            er: self.p_er,
            hr: self.p_hr,
            bb: self.p_bb,
            ibb: self.p_ibb,
            so: self.p_so,
            wp: self.p_wp,
            bk: self.p_bk,
            hbp: self.p_hbp,
            go: self.p_go,
            ao: self.p_ao,
            p: self.p_pitches,
            s: self.p_strikes,
            decision: String::new(),
        }
    }

    pub fn batted(&self) -> bool { self.batting_order_slot > 0 }
    pub fn pitched(&self) -> bool { self.p_g }
}


pub fn gamelogs_from_daily_stats<T: io::BufRead>(daily_csv: T) -> PlayerGameLogs {
    let mut batting_gamelogs = Vec::new();
    let mut fielding_gamelogs = Vec::new();
    let mut pitching_gamelogs = Vec::new();

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(daily_csv);

    for result in reader.deserialize() {
        match result {
            Ok(game) => {
                let game: Cwdaily = game;
                let game_logs = game.split_into_game_logs();
                batting_gamelogs.extend(game_logs.0);
                fielding_gamelogs.extend(game_logs.1);
                pitching_gamelogs.extend(game_logs.2);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    (batting_gamelogs, fielding_gamelogs, pitching_gamelogs)
}
