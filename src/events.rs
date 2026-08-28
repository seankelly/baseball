
pub struct Event {
    pub game_id: String,
    pub batter_id: String,
    pub pitcher_id: String,
    pub balls: u8,
    pub strikes: u8,
    pub pitch_sequence: String,
    pub event_type: u8,
    pub event_number: u8,
}
