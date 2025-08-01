use crate::state::{ArenaConfig, PlayerStats};
use anchor_lang::prelude::*;

#[account]
pub struct ArenaState {
    pub config: ArenaConfig,
    pub is_initialized: bool,
    pub total_players: u32,
    pub total_matches: u32,
}

impl ArenaState {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, config: ArenaConfig) -> Result<()> {
        require_eq!(self.is_initialized, false, ArenaError::AlreadyInitialized);
        
        self.config = config;
        self.is_initialized = true;
        self.total_players = 0;
        self.total_matches = 0;
        
        Ok(())
    }
}