use crate::state::{GuildConfig};
use anchor_lang::prelude::*;

#[account]
pub struct GuildState {
    pub config: GuildConfig,
    pub is_initialized: bool,
    pub total_members: u32,
    pub total_resources_contributed: u64,
}

impl GuildState {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, config: GuildConfig) -> Result<()> {
        require_eq!(self.is_initialized, false, GuildConquestError::AlreadyInitialized);
        
        self.config = config;
        self.is_initialized = true;
        self.total_members = 0;
        self.total_resources_contributed = 0;
        
        Ok(())
    }
}