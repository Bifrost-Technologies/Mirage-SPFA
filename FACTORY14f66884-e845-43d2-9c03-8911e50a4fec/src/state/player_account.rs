use anchor_lang::prelude::*;

#[account]
pub struct PlayerAccount {
    pub owner: Pubkey,
    pub table_pubkey: Pubkey,
    pub player_hand: Vec<Card>,
    pub hand_state: HandState,
    pub bet_amount: u64,
}