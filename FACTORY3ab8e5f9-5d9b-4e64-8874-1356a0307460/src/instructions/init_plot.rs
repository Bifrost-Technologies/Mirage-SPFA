use crate::state::{CropType, PlotState};
use anchor_lang::prelude::*;

pub fn init_plot(ctx: Context<InitPlot>, crop_type: CropType) -> Result<()> {
    let plot_state = &mut ctx.accounts.plot_state;
    
    // Validate crop type
    require!(
        crop_type != CropType::Wheat || 
        crop_type != CropType::Corn || 
        crop_type != CropType::Soybeans,
        FarmingError::InvalidCropType
    );
    
    plot_state.init(ctx.accounts.owner.key(), crop_type)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitPlot<'info> {
    #[account(
        init,
        payer = owner,
        seeds = ["plot".as_bytes(), owner.key().as_ref()],
        bump,
        space = PlotState::MAXIMUM_SIZE
    )]
    pub plot_state: Account<'info, PlotState>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}