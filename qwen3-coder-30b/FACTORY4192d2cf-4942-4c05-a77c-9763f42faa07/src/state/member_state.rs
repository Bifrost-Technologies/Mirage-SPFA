use crate::state::{MemberStats};
use anchor_lang::prelude::*;

#[account]
pub struct MemberState {
    pub owner: Pubkey,
    pub guild_pubkey: Pubkey,
    pub stats: MemberStats,
    pub resources_contributed: u64,
    pub is_active_member: bool,
}

impl MemberState {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, owner_pubkey: Pubkey, guild_pubkey: Pubkey, member_stats: MemberStats) -> Result<()> {
        require_eq!(self.owner, Pubkey::default(), GuildConquestError::MemberAlreadyInGuild);
        
        self.owner = owner_pubkey;
        self.guild_pubkey = guild_pubkey;
        self.stats = member_stats;
        self.resources_contributed = 0;
        self.is_active_member = true;
        
        Ok(())
    }
}