use crate::state::{TableConfig};
use anchor_lang::prelude::*;

#[account]
pub struct TableState {
    pub config: TableConfig,
    pub is_game_active: bool,
    pub dealer_hand: Vec<Card>,
    pub players_in_game: Vec<Pubkey>,
}

impl TableState {
    pub const MAXIMUM_SIZE: usize = 5000;

    pub fn init(&mut self, config: TableConfig) -> Result<()> {
        require_eq!(self.is_game_active, false, BlackjackError::TableAlreadyInitialized);
        
        self.config = config;
        self.is_game_active = false;
        self.dealer_hand = Vec::new();
        self.players_in_game = Vec::new();
        
        Ok(())
    }

    pub fn add_player(&mut self, player_pubkey: Pubkey) -> Result<()> {
        // Check if player is already in game
        for &existing_player in &self.players_in_game {
            if existing_player == player_pubkey {
                return Err(BlackjackError::PlayerAlreadyJoined.into());
            }
        }
        
        self.players_in_game.push(player_pubkey);
        
        Ok(())
    }

    pub fn start_game(&mut self) -> Result<()> {
        require_gte!(self.players_in_game.len(), 1, BlackjackError::NotEnoughPlayers);
        
        self.is_game_active = true;
        
        Ok(())
    }
}