use anchor_lang::prelude::*;

#[account]
pub struct BetAccount {
    pub owner: Pubkey,
    pub wheel_pubkey: Pubkey,
    pub bet_type: BetTypeEnum,
    pub bet_amount: u64,
    pub payout_multiplier: f64,
}