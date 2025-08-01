use crate::state::{RaceConfig};
use anchor_lang::prelude::*;

#[account]
pub struct MarketState {
    pub config: RaceConfig,
    pub is_closed: bool,
    pub winner_index: Option<u8>,
    pub total_bets_placed: u64,
}

impl MarketState {
    pub const MAXIMUM_SIZE: usize = 2000;

    pub fn init(&mut self, config: RaceConfig) -> Result<()> {
        require_eq!(self.is_closed, false, DerbyError::AlreadyInitialized);
        
        self.config = config;
        self.is_closed = false;
        self.winner_index = None;
        self.total_bets_placed = 0;
        
        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        require_eq!(self.is_closed, false, DerbyError::MarketNotClosed);
        
        self.is_closed = true;
        
        Ok(())
    }

    pub fn set_winner(&mut self, winner_index: u8) -> Result<()> {
        // Validate that the winner index is valid
        require!(
            winner_index < self.config.participants.len() as u8,
            DerbyError::InvalidOutcome
        );
        
        self.winner_index = Some(winner_index);
        
        Ok(())
    }
}