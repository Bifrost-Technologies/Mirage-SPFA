use crate::state::{SeriesConfig};
use anchor_lang::prelude::*;

#[account]
pub struct SeriesConfigAccount {
    pub config: SeriesConfig,
    pub is_initialized: bool,
    pub total_boxes_minted: u32,
}

impl SeriesConfigAccount {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, config: SeriesConfig) -> Result<()> {
        require_eq!(self.is_initialized, false, BlindBoxError::AlreadyInitialized);
        
        self.config = config;
        self.is_initialized = true;
        self.total_boxes_minted = 0;
        
        Ok(())
    }
}