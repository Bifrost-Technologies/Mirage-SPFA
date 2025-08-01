use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RaceConfig {
    pub race_name: String,
    pub track_length_meters: u32,
    pub max_participants: u32,
    pub race_start_time: i64,
}