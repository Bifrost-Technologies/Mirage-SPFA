use crate::state::{GameConfig};
use anchor_lang::prelude::*;

#[account]
pub struct GameConfigAccount {
    pub config: GameConfig,
    pub is_initialized: bool,
    pub total_players: u32,
}

impl GameConfigAccount {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, config: GameConfig) -> Result<()> {
        require_eq!(self.is_initialized, false, TreasureHuntError::AlreadyInitialized);
        
        self.config = config;
        self.is_initialized = true;
        self.total_players = 0;
        
        Ok(())
    }
}