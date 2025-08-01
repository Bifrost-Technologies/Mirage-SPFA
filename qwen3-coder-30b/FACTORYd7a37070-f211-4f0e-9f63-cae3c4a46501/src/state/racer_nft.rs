use crate::state::{CarStats};
use anchor_lang::prelude::*;

#[account]
pub struct RacerNFT {
    pub owner: Pubkey,
    pub nft_mint: Pubkey,
    pub car_stats: CarStats,
    pub is_registered: bool,
}

impl RacerNFT {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, owner_pubkey: Pubkey, nft_mint: Pubkey, car_stats: CarStats) -> Result<()> {
        require_eq!(self.is_registered, false, RacingError::NftAlreadyRegistered);
        
        self.owner = owner_pubkey;
        self.nft_mint = nft_mint;
        self.car_stats = car_stats;
        self.is_registered = true;
        
        Ok(())
    }
}