use serde_derive::Deserialize;

use crate::chadwick::bool_from_string;
use crate::chadwick::parse_handedness;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct Event {
    pub game_id: String,
    pub away_team_id: String,
    pub inn_ct: u8,
    pub bat_home_id: String,
    pub outs_ct: u8,
    pub balls_ct: u8,
    pub strikes_ct: u8,
    pub pitch_seq_tx: String,
    pub away_score_ct: u8,
    pub home_score_ct: u8,
    pub bat_id: String,
    #[serde(deserialize_with = "parse_handedness")]
    pub bat_hand_cd: Handedness,
    pub resp_bat_id: String,
    #[serde(deserialize_with = "parse_handedness")]
    pub resp_bat_hand_cd: Handedness,
    pub pit_id: String,
    #[serde(deserialize_with = "parse_handedness")]
    pub pit_hand_cd: Handedness,
    pub resp_pit_id: String,
    #[serde(deserialize_with = "parse_handedness")]
    pub resp_pit_hand_cd: Handedness,
    pub pos2_fld_id: String,
    pub pos3_fld_id: String,
    pub pos4_fld_id: String,
    pub pos5_fld_id: String,
    pub pos6_fld_id: String,
    pub pos7_fld_id: String,
    pub pos8_fld_id: String,
    pub pos9_fld_id: String,
    pub base1_run_id: String,
    pub base2_run_id: String,
    pub base3_run_id: String,
    pub event_tx: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub leadoff_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub ph_fl: bool,
    pub bat_fld_cd: u8,
    pub bat_lineup_id: u8,
    pub event_cd: u8,
    #[serde(deserialize_with = "bool_from_string")]
    pub bat_event_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub ab_fl: bool,
    pub h_cd: u8,
    #[serde(deserialize_with = "bool_from_string")]
    pub sh_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub sf_fl: bool,
    pub event_outs_ct: u8,
    #[serde(deserialize_with = "bool_from_string")]
    pub dp_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub tp_fl: bool,
    pub rbi_ct: u8,
    #[serde(deserialize_with = "bool_from_string")]
    pub wp_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub pb_fl: bool,
    pub fld_cd: u8,
    pub battedball_cd: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub bunt_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub foul_fl: bool,
    pub battedball_loc_tx: String,
    pub err_ct: u8,
    pub err1_fld_cd: u8,
    pub err1_cd: String,
    pub err2_fld_cd: u8,
    pub err2_cd: String,
    pub err3_fld_cd: u8,
    pub err3_cd: String,
    pub bat_dest_id: String,
    pub run1_dest_id: String,
    pub run2_dest_id: String,
    pub run3_dest_id: String,
    pub bat_play_tx: String,
    pub run1_play_tx: String,
    pub run2_play_tx: String,
    pub run3_play_tx: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub run1_sb_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub run2_sb_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub run3_sb_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub run1_cs_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub run2_cs_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub run3_cs_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub run1_pk_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub run2_pk_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub run3_pk_fl: bool,
    pub run1_resp_pit_id: String,
    pub run2_resp_pit_id: String,
    pub run3_resp_pit_id: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub game_new_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub game_end_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub pr_run1_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub pr_run2_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub pr_run3_fl: bool,
    pub removed_for_pr_run1_id: String,
    pub removed_for_pr_run2_id: String,
    pub removed_for_pr_run3_id: String,
    pub removed_for_ph_bat_id: String,
    pub removed_for_ph_bat_fld_cd: u8,
    pub po1_fld_cd: u8,
    pub po2_fld_cd: u8,
    pub po3_fld_cd: u8,
    pub ass1_fld_cd: u8,
    pub ass2_fld_cd: u8,
    pub ass3_fld_cd: u8,
    pub ass4_fld_cd: u8,
    pub ass5_fld_cd: u8,
    pub event_id: String,
}

#[derive(Debug)]
pub enum Handedness {
    Unknown,
    Both,
    Left,
    Right,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct EventExtended {
    home_team_id: String,
    bat_team_id: String,
    fld_team_id: String,
    bat_last_id: String,
    inn_new_fl: bool,
    inn_end_fl: bool,
    start_bat_score_ct: u8,
    start_fld_score_ct: u8,
    inn_runs_ct: u8,
    game_pa_ct: u8,
    inn_pa_ct: u8,
    pa_new_fl: bool,
    pa_trunc_fl: bool,
    start_bases_cd: String,
    end_bases_cd: String,
    bat_start_fl: bool,
    resp_bat_start_fl: bool,
    bat_on_deck_id: String,
    bat_in_hold_id: String,
    pit_start_fl: bool,
    resp_pit_start_fl: bool,
    run1_fld_cd: String,
    run1_lineup_cd: String,
    run1_origin_event_id: String,
    run2_fld_cd: String,
    run2_lineup_cd: String,
    run2_origin_event_id: String,
    run3_fld_cd: String,
    run3_lineup_cd: String,
    run3_origin_event_id: String,
    run1_resp_cat_id: String,
    run2_resp_cat_id: String,
    run3_resp_cat_id: String,
    pa_ball_ct: u8,
    pa_called_ball_ct: u8,
    pa_intent_ball_ct: u8,
    pa_pitchout_ball_ct: u8,
    pa_hitbatter_ball_ct: u8,
    pa_other_ball_ct: u8,
    pa_strike_ct: u8,
    pa_called_strike_ct: u8,
    pa_swingmiss_strike_ct: u8,
    pa_foul_strike_ct: u8,
    pa_inplay_strike_ct: u8,
    pa_other_strike_ct: u8,
    event_runs_ct: u8,
    fld_id: String,
    base2_force_fl: bool,
    base3_force_fl: bool,
    base4_force_fl: bool,
    bat_safe_err_fl: bool,
    bat_fate_id: String,
    run1_fate_id: String,
    run2_fate_id: String,
    run3_fate_id: String,
    fate_runs_ct: u8,
    ass6_fld_cd: String,
    ass7_fld_cd: String,
    ass8_fld_cd: String,
    ass9_fld_cd: String,
    ass10_fld_cd: String,
    unknown_out_exc_fl: bool,
    uncertain_play_exc_fl: bool,
}
