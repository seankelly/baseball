use std::default::Default;


pub struct Guts {
    pub season: u16,
    pub woba: WobaWeights,
    pub fip_constant: f32,
}


#[derive(Default)]
pub struct WobaWeights {
    pub scale: f32,
    pub bb_weight: f32,
    pub hbp_weight: f32,
    pub s_weight: f32,
    pub d_weight: f32,
    pub t_weight: f32,
    pub hr_weight: f32,
}


impl Guts {
    pub fn new(season: u16) -> Self {
        Self {
            season,
            woba: WobaWeights::default(),
            fip_constant: 0.0,
        }
    }
}
