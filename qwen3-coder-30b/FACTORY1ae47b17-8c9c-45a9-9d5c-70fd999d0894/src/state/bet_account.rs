use anchor_lang::prelude::*;

#[account]
pub struct BetAccount {
    pub owner: Pubkey,
    pub game_pubkey: Pubkey,
    pub bet_type: BetType,
    pub bet_amount: u64,
    pub payout_multiplier: f64,
}