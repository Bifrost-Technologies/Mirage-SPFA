use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PayoutTable {
    pub red_payout: f64,
    pub black_payout: f64,
    pub even_payout: f64,
    pub odd_payout: f64,
    pub low_payout: f64,
    pub high_payout: f64,
}