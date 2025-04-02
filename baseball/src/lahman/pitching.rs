use serde_derive::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Pitching {
    #[serde(rename = "playerID")]
    pub player_id: String,
    #[serde(rename = "yearID")]
    pub year_id: u16,
    #[serde(rename = "stint")]
    pub stint: String,
    #[serde(rename = "teamID")]
    pub team_id: String,
    #[serde(rename = "lgID")]
    pub league_id: String,
    #[serde(rename = "W")]
    pub wins: u8,
    #[serde(rename = "L")]
    pub losses: u8,
    #[serde(rename = "G")]
    pub games: u8,
    #[serde(rename = "GS")]
    pub games_started: u8,
    #[serde(rename = "CG")]
    pub complete_games: u8,
    #[serde(rename = "SHO")]
    pub shutouts: u8,
    #[serde(rename = "SV")]
    pub saves: u8,
    #[serde(rename = "IPouts")]
    pub ip_outs: u16,
    #[serde(rename = "H")]
    pub hits: u16,
    #[serde(rename = "ER")]
    pub earned_runs: u16,
    #[serde(rename = "HR")]
    pub home_runs: u8,
    #[serde(rename = "BB")]
    pub walks: u16,
    #[serde(rename = "SO")]
    pub strikeouts: u16,
    #[serde(rename = "BAOpp")]
    pub ba_opp: Option<f32>,
    #[serde(rename = "ERA")]
    pub era: Option<f32>,
    #[serde(rename = "IBB")]
    pub intentional_walks: Option<u8>,
    #[serde(rename = "WP")]
    pub wild_pitches: u8,
    #[serde(rename = "HBP")]
    pub hit_by_pitches: Option<u8>,
    #[serde(rename = "BK")]
    pub balks: u8,
    #[serde(rename = "BFP")]
    pub batters_faced: Option<u16>,
    #[serde(rename = "GF")]
    pub games_finished: u8,
    #[serde(rename = "R")]
    pub runs: u16,
    #[serde(rename = "SH")]
    pub sacrifice_hits: Option<u8>,
    #[serde(rename = "SF")]
    pub sacrifice_flies: Option<u8>,
    #[serde(rename = "GIDP")]
    pub gidp: Option<u8>,
}
