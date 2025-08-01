use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GameConfig {
    pub player_a: Pubkey,
    pub player_b: Pubkey,
    pub time_per_player_seconds: u64,
    pub clock_start_time: i64,
}