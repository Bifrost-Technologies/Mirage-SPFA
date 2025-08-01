use crate::state::{GameConfig};
use anchor_lang::prelude::*;

#[account]
pub struct GameState {
    pub config: GameConfig,
    pub is_game_active: bool,
    pub player_hand: Vec<Card>,
    pub banker_hand: Vec<Card>,
    pub game_outcome: Option<OutcomeEnum>,
}

impl GameState {
    pub const MAXIMUM_SIZE: usize = 5000;

    pub fn init(&mut self, config: GameConfig) -> Result<()> {
        require_eq!(self.is_game_active, false, BaccaratError::AlreadyInitialized);
        
        self.config = config;
        self.is_game_active = false;
        self.player_hand = Vec::new();
        self.banker_hand = Vec::new();
        self.game_outcome = None;
        
        Ok(())
    }

    pub fn start_game(&mut self) -> Result<()> {
        require_eq!(self.is_game_active, false, BaccaratError::GameNotInProgress);
        
        self.is_game_active = true;
        
        Ok(())
    }

    pub fn end_game(&mut self) -> Result<()> {
        require_eq!(self.is_game_active, true, BaccaratError::GameNotInProgress);
        
        self.is_game_active = false;
        
        Ok(())
    }
}