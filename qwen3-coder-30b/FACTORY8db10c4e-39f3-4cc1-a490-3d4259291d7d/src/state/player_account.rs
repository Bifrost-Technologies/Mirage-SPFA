use crate::state::{PlayerStats};
use anchor_lang::prelude::*;

#[account]
pub struct PlayerAccount {
    pub owner: Pubkey,
    pub stats: PlayerStats,
    pub elo_rating: u32,
    pub is_registered: bool,
}

impl PlayerAccount {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, owner_pubkey: Pubkey, player_stats: PlayerStats) -> Result<()> {
        require_eq!(self.is_registered, false, ArenaError::PlayerAlreadyRegistered);
        
        self.owner = owner_pubkey;
        self.stats = player_stats;
        self.elo_rating = 1200; // Default ELO rating
        self.is_registered = true;
        
        Ok(())
    }
}