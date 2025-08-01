use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TableConfig {
    pub max_players: u32,
    pub min_bet: u64,
    pub max_bet: u64,
    pub deck_count: u32,
}