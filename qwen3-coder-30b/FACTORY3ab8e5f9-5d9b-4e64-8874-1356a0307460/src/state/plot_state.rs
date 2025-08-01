use crate::state::{CropType, GrowthStage};
use anchor_lang::prelude::*;

#[account]
pub struct PlotState {
    pub owner: Pubkey,
    pub crop_type: CropType,
    pub growth_stage: GrowthStage,
    pub planted_time: i64,
    pub harvest_ready_time: i64,
    pub seeds_planted: u64,
    pub crops_harvested: u64,
}

impl PlotState {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, owner_pubkey: Pubkey, crop_type: CropType) -> Result<()> {
        require_eq!(self.owner, Pubkey::default(), FarmingError::PlotAlreadyInitialized);
        
        self.owner = owner_pubkey;
        self.crop_type = crop_type;
        self.growth_stage = GrowthStage::Seed;
        self.planted_time = 0;
        self.harvest_ready_time = 0;
        self.seeds_planted = 0;
        self.crops_harvested = 0;
        
        Ok(())
    }

    pub fn set_growth_stage(&mut self, stage: GrowthStage) -> Result<()> {
        self.growth_stage = stage;
        
        Ok(())
    }

    pub fn update_harvest_ready_time(&mut self, ready_time: i64) -> Result<()> {
        self.harvest_ready_time = ready_time;
        
        Ok(())
    }
}