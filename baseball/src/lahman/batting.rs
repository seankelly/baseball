use serde_derive::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Batting {
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
    #[serde(rename = "G")]
    pub games: u8,
    #[serde(rename = "G_batting")]
    pub games_batting: Option<u8>,
    #[serde(rename = "AB")]
    pub atbats: u16,
    #[serde(rename = "R")]
    pub runs: u8,
    #[serde(rename = "H")]
    pub hits: u16,
    #[serde(rename = "2B")]
    pub doubles: u8,
    #[serde(rename = "3B")]
    pub triples: u8,
    #[serde(rename = "HR")]
    pub home_runs: u8,
    #[serde(rename = "RBI")]
    pub runs_batted_in: Option<u8>,
    #[serde(rename = "SB")]
    pub stolen_bases: Option<u8>,
    #[serde(rename = "CS")]
    pub caught_stealing: Option<u8>,
    #[serde(rename = "BB")]
    pub walks: u8,
    #[serde(rename = "SO")]
    pub strikeouts: Option<u8>,
    #[serde(rename = "IBB")]
    pub intentional_walks: Option<u8>,
    #[serde(rename = "HBP")]
    pub hit_by_pitches: Option<u8>,
    #[serde(rename = "SH")]
    pub sacrifice_hits: Option<u8>,
    #[serde(rename = "SF")]
    pub sacrifice_flies: Option<u8>,
    #[serde(rename = "GIDP")]
    pub gidp: Option<u8>,
    #[serde(rename = "G_old")]
    pub games_old: Option<u8>,
}
