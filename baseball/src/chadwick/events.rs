use serde_derive::Deserialize;

use crate::chadwick::bool_from_string;
use crate::chadwick::parse_handedness;


#[derive(Deserialize, Debug)]
pub struct FullEventRow<'a> {
    // 0
    pub game_id: &'a str,
    pub away_team_id: &'a str,
    pub inn_ct: &'a str,
    pub bat_home_id: &'a str,
    pub outs_ct: &'a str,
    pub balls_ct: &'a str,
    pub strikes_ct: &'a str,
    pub pitch_seq_tx: &'a str,
    pub away_score_ct: &'a str,
    pub home_score_ct: &'a str,
    // 20
    pub bat_id: &'a str,
    pub bat_hand_cd: &'a str,
    pub resp_bat_id: &'a str,
    pub resp_bat_hand_cd: &'a str,
    pub pit_id: &'a str,
    pub pit_hand_cd: &'a str,
    pub resp_pit_id: &'a str,
    pub resp_pit_hand_cd: &'a str,
    pub pos2_fld_id: &'a str,
    pub pos3_fld_id: &'a str,
    // 10
    pub pos4_fld_id: &'a str,
    pub pos5_fld_id: &'a str,
    pub pos6_fld_id: &'a str,
    pub pos7_fld_id: &'a str,
    pub pos8_fld_id: &'a str,
    pub pos9_fld_id: &'a str,
    pub base1_run_id: &'a str,
    pub base2_run_id: &'a str,
    pub base3_run_id: &'a str,
    pub event_tx: &'a str,
    // 30
    pub leadoff_fl: &'a str,
    pub ph_fl: &'a str,
    pub bat_fld_cd: &'a str,
    pub bat_lineup_id: &'a str,
    pub event_cd: &'a str,
    pub bat_event_fl: &'a str,
    pub ab_fl: &'a str,
    pub h_cd: &'a str,
    pub sh_fl: &'a str,
    pub sf_fl: &'a str,
    // 40
    pub event_outs_ct: &'a str,
    pub dp_fl: &'a str,
    pub tp_fl: &'a str,
    pub rbi_ct: &'a str,
    pub wp_fl: &'a str,
    pub pb_fl: &'a str,
    pub fld_cd: &'a str,
    pub battedball_cd: &'a str,
    pub bunt_fl: &'a str,
    pub foul_fl: &'a str,
    // 50
    pub battedball_loc_tx: &'a str,
    pub err_ct: &'a str,
    pub err1_fld_cd: &'a str,
    pub err1_cd: &'a str,
    pub err2_fld_cd: &'a str,
    pub err2_cd: &'a str,
    pub err3_fld_cd: &'a str,
    pub err3_cd: &'a str,
    pub bat_dest_id: &'a str,
    pub run1_dest_id: &'a str,
    // 60
    pub run2_dest_id: &'a str,
    pub run3_dest_id: &'a str,
    pub bat_play_tx: &'a str,
    pub run1_play_tx: &'a str,
    pub run2_play_tx: &'a str,
    pub run3_play_tx: &'a str,
    pub run1_sb_fl: &'a str,
    pub run2_sb_fl: &'a str,
    pub run3_sb_fl: &'a str,
    pub run1_cs_fl: &'a str,
    // 70
    pub run2_cs_fl: &'a str,
    pub run3_cs_fl: &'a str,
    pub run1_pk_fl: &'a str,
    pub run2_pk_fl: &'a str,
    pub run3_pk_fl: &'a str,
    pub run1_resp_pit_id: &'a str,
    pub run2_resp_pit_id: &'a str,
    pub run3_resp_pit_id: &'a str,
    pub game_new_fl: &'a str,
    pub game_end_fl: &'a str,
    // 80
    pub pr_run1_fl: &'a str,
    pub pr_run2_fl: &'a str,
    pub pr_run3_fl: &'a str,
    pub removed_for_pr_run1_id: &'a str,
    pub removed_for_pr_run2_id: &'a str,
    pub removed_for_pr_run3_id: &'a str,
    pub removed_for_ph_bat_id: &'a str,
    pub removed_for_ph_bat_fld_cd: &'a str,
    pub po1_fld_cd: &'a str,
    pub po2_fld_cd: &'a str,
    // 90
    pub po3_fld_cd: &'a str,
    pub ass1_fld_cd: &'a str,
    pub ass2_fld_cd: &'a str,
    pub ass3_fld_cd: &'a str,
    pub ass4_fld_cd: &'a str,
    pub ass5_fld_cd: &'a str,
    pub event_id: &'a str,
    // Extended event 0
    pub home_team_id: &'a str,
    pub bat_team_id: &'a str,
    pub fld_team_id: &'a str,
    pub bat_last_id: &'a str,
    pub inn_new_fl: &'a str,
    pub inn_end_fl: &'a str,
    pub start_bat_score_ct: &'a str,
    pub start_fld_score_ct: &'a str,
    pub inn_runs_ct: &'a str,
    pub game_pa_ct: &'a str,
    // Extended event 10
    pub inn_pa_ct: &'a str,
    pub pa_new_fl: &'a str,
    pub pa_trunc_fl: &'a str,
    pub start_bases_cd: &'a str,
    pub end_bases_cd: &'a str,
    pub bat_start_fl: &'a str,
    pub resp_bat_start_fl: &'a str,
    pub bat_on_deck_id: &'a str,
    pub bat_in_hold_id: &'a str,
    pub pit_start_fl: &'a str,
    // Extended event 20
    pub resp_pit_start_fl: &'a str,
    pub run1_fld_cd: &'a str,
    pub run1_lineup_cd: &'a str,
    pub run1_origin_event_id: &'a str,
    pub run2_fld_cd: &'a str,
    pub run2_lineup_cd: &'a str,
    pub run2_origin_event_id: &'a str,
    pub run3_fld_cd: &'a str,
    pub run3_lineup_cd: &'a str,
    pub run3_origin_event_id: &'a str,
    // Extended event 30
    pub run1_resp_cat_id: &'a str,
    pub run2_resp_cat_id: &'a str,
    pub run3_resp_cat_id: &'a str,
    pub pa_ball_ct: &'a str,
    pub pa_called_ball_ct: &'a str,
    pub pa_intent_ball_ct: &'a str,
    pub pa_pitchout_ball_ct: &'a str,
    pub pa_hitbatter_ball_ct: &'a str,
    pub pa_other_ball_ct: &'a str,
    pub pa_strike_ct: &'a str,
    // Extended event 40
    pub pa_called_strike_ct: &'a str,
    pub pa_swingmiss_strike_ct: &'a str,
    pub pa_foul_strike_ct: &'a str,
    pub pa_inplay_strike_ct: &'a str,
    pub pa_other_strike_ct: &'a str,
    pub event_runs_ct: &'a str,
    pub fld_id: &'a str,
    pub base2_force_fl: &'a str,
    pub base3_force_fl: &'a str,
    pub base4_force_fl: &'a str,
    // Extended event 50
    pub bat_safe_err_fl: &'a str,
    pub bat_fate_id: &'a str,
    pub run1_fate_id: &'a str,
    pub run2_fate_id: &'a str,
    pub run3_fate_id: &'a str,
    pub fate_runs_ct: &'a str,
    pub ass6_fld_cd: &'a str,
    pub ass7_fld_cd: &'a str,
    pub ass8_fld_cd: &'a str,
    pub ass9_fld_cd: &'a str,
    // Extended event 60
    pub ass10_fld_cd: &'a str,
    pub unknown_out_exc_fl: &'a str,
    pub uncertain_play_exc_fl: &'a str,
    pub count_tx: &'a str,
}


#[derive(Debug)]
pub enum Handedness {
    Unknown,
    Both,
    Left,
    Right,
}


#[derive(Deserialize, Debug)]
pub struct FullEvent {
    // Event 0
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
    // Event 10
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
    // Event 20
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
    // Event 30
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
    // Event 40
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
    // Event 50
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
    // Event 60
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
    // Event 70
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
    // Event 80
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
    // Event 90
    pub po3_fld_cd: u8,
    pub ass1_fld_cd: u8,
    pub ass2_fld_cd: u8,
    pub ass3_fld_cd: u8,
    pub ass4_fld_cd: u8,
    pub ass5_fld_cd: u8,
    pub event_id: String,
}


#[derive(Deserialize, Debug)]
pub struct Event {
    // Event 0
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
    // Event 10
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
    // Event 20
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
    // Event 30
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
    // Event 40
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
    // Event 50
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
    // Event 60
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
    // Event 70
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
    // Event 80
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
    // Event 90
    pub po3_fld_cd: u8,
    pub ass1_fld_cd: u8,
    pub ass2_fld_cd: u8,
    pub ass3_fld_cd: u8,
    pub ass4_fld_cd: u8,
    pub ass5_fld_cd: u8,
    pub event_id: String,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct ExtendedEvent {
    // Event 0
    pub home_team_id: String,
    pub bat_team_id: String,
    pub fld_team_id: String,
    pub bat_last_id: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub inn_new_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub inn_end_fl: bool,
    pub start_bat_score_ct: u8,
    pub start_fld_score_ct: u8,
    pub inn_runs_ct: u8,
    pub game_pa_ct: u8,
    // Event 10
    pub inn_pa_ct: u8,
    #[serde(deserialize_with = "bool_from_string")]
    pub pa_new_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub pa_trunc_fl: bool,
    pub start_bases_cd: String,
    pub end_bases_cd: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub bat_start_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub resp_bat_start_fl: bool,
    pub bat_on_deck_id: String,
    pub bat_in_hold_id: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub pit_start_fl: bool,
    // Event 20
    #[serde(deserialize_with = "bool_from_string")]
    pub resp_pit_start_fl: bool,
    pub run1_fld_cd: String,
    pub run1_lineup_cd: String,
    pub run1_origin_event_id: String,
    pub run2_fld_cd: String,
    pub run2_lineup_cd: String,
    pub run2_origin_event_id: String,
    pub run3_fld_cd: String,
    pub run3_lineup_cd: String,
    pub run3_origin_event_id: String,
    // Event 30
    pub run1_resp_cat_id: String,
    pub run2_resp_cat_id: String,
    pub run3_resp_cat_id: String,
    pub pa_ball_ct: u8,
    pub pa_called_ball_ct: u8,
    pub pa_intent_ball_ct: u8,
    pub pa_pitchout_ball_ct: u8,
    pub pa_hitbatter_ball_ct: u8,
    pub pa_other_ball_ct: u8,
    pub pa_strike_ct: u8,
    // Event 40
    pub pa_called_strike_ct: u8,
    pub pa_swingmiss_strike_ct: u8,
    pub pa_foul_strike_ct: u8,
    pub pa_inplay_strike_ct: u8,
    pub pa_other_strike_ct: u8,
    pub event_runs_ct: u8,
    pub fld_id: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub base2_force_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub base3_force_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub base4_force_fl: bool,
    // Event 50
    #[serde(deserialize_with = "bool_from_string")]
    pub bat_safe_err_fl: bool,
    pub bat_fate_id: String,
    pub run1_fate_id: String,
    pub run2_fate_id: String,
    pub run3_fate_id: String,
    pub fate_runs_ct: u8,
    pub ass6_fld_cd: String,
    pub ass7_fld_cd: String,
    pub ass8_fld_cd: String,
    pub ass9_fld_cd: String,
    // Event 60
    pub ass10_fld_cd: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub unknown_out_exc_fl: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub uncertain_play_exc_fl: bool,
    pub count_tx: String,
}
