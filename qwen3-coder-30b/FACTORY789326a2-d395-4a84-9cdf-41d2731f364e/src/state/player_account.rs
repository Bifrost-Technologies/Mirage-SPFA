use anchor_lang::prelude::*;

#[account]
pub struct PlayerAccount {
    pub owner: Pubkey,
    pub player_pubkey: Pubkey,
    pub time_remaining: i64, // Time remaining in seconds
}

impl PlayerAccount {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, owner_pubkey: Pubkey, player_pubkey: Pubkey) -> Result<()> {
        self.owner = owner_pubkey;
        self.player_pubkey = player_pubkey;
        // Initialize with full time
        let clock_time = Clock::get()?.unix_timestamp;
        
        // In a real implementation, this would be set based on game config
        self.time_remaining = 3600; // 1 hour default
        
        Ok(())
    }
}