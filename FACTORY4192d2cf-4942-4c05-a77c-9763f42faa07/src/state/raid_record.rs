use crate::state::{RaidOutcome};
use anchor_lang::prelude::*;

#[account]
pub struct RaidRecord {
    pub guild_pubkey: Pubkey,
    pub raid_outcome: RaidOutcome,
    pub is_launched: bool,
    pub is_completed: bool,
    pub loot_distributed: bool,
}

impl RaidRecord {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, guild_pubkey: Pubkey, raid_outcome: RaidOutcome) -> Result<()> {
        self.guild_pubkey = guild_pubkey;
        self.raid_outcome = raid_outcome;
        self.is_launched = false;
        self.is_completed = false;
        self.loot_distributed = false;
        
        Ok(())
    }

    pub fn launch(&mut self) -> Result<()> {
        require_eq!(self.is_launched, false, GuildConquestError::RaidNotFound);
        
        self.is_launched = true;
        
        Ok(())
    }

    pub fn complete(&mut self) -> Result<()> {
        require_eq!(self.is_completed, false, GuildConquestError::RaidNotFound);
        
        self.is_completed = true;
        
        Ok(())
    }
}