use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RaceConfig {
    pub race_name: String,
    pub participants: Vec<String>,
    pub start_time: i64,
    pub end_time: i64,
    pub total_pot: u64,
}