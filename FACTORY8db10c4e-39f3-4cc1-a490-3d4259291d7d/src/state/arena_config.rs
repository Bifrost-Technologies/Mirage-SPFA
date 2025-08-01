use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ArenaConfig {
    pub max_players: u32,
    pub match_timeout_seconds: u64,
    pub base_elo_rating: u32,
    pub elo_k_factor: u32,
}