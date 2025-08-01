use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WheelConfig {
    pub min_bet: u64,
    pub max_bet: u64,
    pub house_edge: f64,
    pub payout_table: PayoutTable,
}