use anchor_lang::prelude::*;

#[account]
pub struct LeagueConfig {
    pub race_config: RaceConfig,
    pub is_race_active: bool,
    pub total_registered_nfts: u32,
}

impl LeagueConfig {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, race_config: RaceConfig) -> Result<()> {
        self.race_config = race_config;
        self.is_race_active = false;
        self.total_registered_nfts = 0;
        
        Ok(())
    }
}