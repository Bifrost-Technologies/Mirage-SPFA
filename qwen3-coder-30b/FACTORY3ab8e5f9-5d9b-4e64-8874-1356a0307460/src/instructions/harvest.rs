use crate::state::{PlotState};
use anchor_lang::prelude::*;

pub fn harvest(ctx: Context<Harvest>) -> Result<()> {
    let plot_state = &mut ctx.accounts.plot_state;
    
    // Validate that the crop is ready for harvest
    let current_time = Clock::get()?.unix_timestamp;
    
    require_gte!(
        current_time,
        plot_state.harvest_ready_time,
        FarmingError::CropNotReadyForHarvest
    );
    
    // In a real implementation, you would:
    // 1. Calculate the actual harvest amount based on growth time and crop type
    // 2. Update the plot state with harvested crops
    
    let harvest_amount = plot_state.seeds_planted * 2; // Simplified yield calculation
    
    plot_state.crops_harvested = harvest_amount;
    
    Ok(())
}

#[derive(Accounts)]
pub struct Harvest<'info> {
    #[account(
        mut,
        seeds = ["plot".as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub plot_state: Account<'info, PlotState>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
}