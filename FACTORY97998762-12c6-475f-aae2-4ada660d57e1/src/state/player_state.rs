use anchor_lang::prelude::*;

#[account]
pub struct PlayerState {
    pub owner: Pubkey,
    pub match_pubkey: Pubkey,
    pub answer_hash: Option<[u8; 32]>,
    pub score: u32,
}