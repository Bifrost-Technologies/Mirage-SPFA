use crate::state::{MatchData, PlayerStats};
use anchor_lang::prelude::*;

#[account]
pub struct MatchRecord {
    pub match_id: u32,
    pub player_a: Pubkey,
    pub player_b: Pubkey,
    pub player_a_stats: PlayerStats,
    pub player_b_stats: PlayerStats,
    pub current_turn_player: Pubkey, // The pubkey of the player whose turn it is
    pub moves_submitted_count: u8,   // Number of moves submitted so far
    pub match_data: MatchData,
    pub is_resolved: bool,
}

impl MatchRecord {
    pub const MAXIMUM_SIZE: usize = 2000;

    pub fn init(
        &mut self, 
        match_id: u32, 
        player_a_pubkey: Pubkey, 
        player_b_pubkey: Pubkey,
        player_a_stats: PlayerStats,
        player_b_stats: PlayerStats,
        match_data: MatchData
    ) -> Result<()> {
        
        self.match_id = match_id;
        
        // Set players
        self.player_a = player_a_pubkey;
        self.player_b = player_b_pubkey;
        
        // Store stats for both players
        self.player_a_stats = player_a_stats;
        self.player_b_stats = player_b_stats;

        // Initialize current turn to first player (player A)
        self.current_turn_player = player_a_pubkey;
        
        // No moves submitted yet
        self.moves_submitted_count = 0;
        
        // Store match data
        self.match_data = match_data;
        
        // Match is not resolved yet
        self.is_resolved = false;

        Ok(())
    }

    pub fn switch_turn(&mut self) -> Result<()> {
        if self.current_turn_player == self.player_a {
            self.current_turn_player = self.player_b;
        } else {
            self.current_turn_player = self.player_a;
        }
        
        // Increment move count
        if let Some(count) = self.moves_submitted_count.checked_add(1) {
            self.moves_submitted_count = count;
        } else {
            return Err(ArenaError::InvalidMove.into());
        }

        Ok(())
    }
}