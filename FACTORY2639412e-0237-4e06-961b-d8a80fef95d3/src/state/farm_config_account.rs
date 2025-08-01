use crate::state::{FarmConfig};
use anchor_lang::prelude::*;

#[account]
pub struct FarmConfigAccount {
    pub config: FarmConfig,
    pub is_initialized: bool,
    pub total_nfts: u32,
}

impl FarmConfigAccount {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, config: FarmConfig) -> Result<()> {
        require_eq!(self.is_initialized, false, NftBreederError::AlreadyInitialized);
        
        self.config = config;
        self.is_initialized = true;
        self.total_nfts = 0;
        
        Ok(())
    }
}