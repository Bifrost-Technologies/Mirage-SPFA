use crate::state::{BoxState};
use anchor_lang::prelude::*;

#[account]
pub struct UserBoxAccount {
    pub owner: Pubkey,
    pub series_pubkey: Pubkey,
    pub box_id: u32,
    pub box_state: BoxState,
    pub revealed_rarity: Option<RarityTier>,
}

impl UserBoxAccount {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, owner_pubkey: Pubkey, series_pubkey: Pubkey, box_id: u32) -> Result<()> {
        self.owner = owner_pubkey;
        self.series_pubkey = series_pubkey;
        self.box_id = box_id;
        self.box_state = BoxState::Minted; // Initially minted but not revealed
        self.revealed_rarity = None;
        
        Ok(())
    }
}