use crate::state::{PlotState};
use anchor_lang::prelude::*;

pub fn plant_crop(
    ctx: Context<PlantCrop>,
    seed_amount: u64,
    signature: Vec<u8>,
) -> Result<()> {
    let plot_state = &mut ctx.accounts.plot_state;
    
    // Validate that there are enough seeds
    require_gte!(seed_amount, 1, FarmingError::NotEnoughSeeds);
    
    // In a real implementation, you would verify the signature here
    
    // Update plot state with planting information
    let current_time = Clock::get()?.unix_timestamp;
    
    plot_state.planted_time = current_time;
    
    // Set harvest ready time based on crop type (simplified)
    let harvest_ready_seconds = match plot_state.crop_type {
        CropType::Wheat => 3600, // 1 hour
        CropType::Corn => 7200, // 2 hours
        CropType::Soybeans => 10800, // 3 hours
    };
    
    plot_state.update_harvest_ready_time(current_time + harvest_ready_seconds)?;
    
    plot_state.seeds_planted = seed_amount;
    
    Ok(())
}

#[derive(Accounts)]
pub struct PlantCrop<'info> {
    #[account(
        mut,
        seeds = ["plot".as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub plot_state: Account<'info, PlotState>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
}