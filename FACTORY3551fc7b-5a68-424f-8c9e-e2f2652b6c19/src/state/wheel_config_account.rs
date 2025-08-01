use crate::state::{WheelConfig};
use anchor_lang::prelude::*;

#[account]
pub struct WheelConfigAccount {
    pub config: WheelConfig,
    pub is_initialized: bool,
    pub total_bets_placed: u64,
    pub total_winnings_paid: u64,
}

impl WheelConfigAccount {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, config: WheelConfig) -> Result<()> {
        require_eq!(self.is_initialized, false, RouletteError::AlreadyInitialized);
        
        self.config = config;
        self.is_initialized = true;
        self.total_bets_placed = 0;
        self.total_winnings_paid = 0;
        
        Ok(())
    }
}