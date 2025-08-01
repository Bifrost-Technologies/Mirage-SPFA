use crate::state::{PlotState};
use anchor_lang::prelude::*;

pub fn sell_crop(
    ctx: Context<SellCrop>,
    crop_amount: u64,
    signature: Vec<u8>,
) -> Result<()> {
    let plot_state = &mut ctx.accounts.plot_state;
    
    // Validate that there are enough crops to sell
    require_gte!(
        plot_state.crops_harvested,
        crop_amount,
        FarmingError::NotEnoughSeeds
    );
    
    // In a real implementation, you would verify the signature here
    
    // Calculate revenue based on market price (simplified)
    let base_price = match plot_state.crop_type {
        CropType::Wheat => 100u64,
        CropType::Corn => 150u64,
        CropType::Soybeans => 200u64,
    };
    
    let revenue = base_price * crop_amount;
    
    // Update farmer account with revenue (in a real implementation, this would be in the market vault)
    let farmer_account = &mut ctx.accounts.farmer_account;
    
    if let Some(total) = farmer_account.total_revenue.checked_add(revenue) {
        farmer_account.total_revenue = total;
    }
    
    // Update plot state
    plot_state.crops_harvested -= crop_amount;
    
    Ok(())
}

#[derive(Accounts)]
pub struct SellCrop<'info> {
    #[account(
        mut,
        seeds = ["plot".as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub plot_state: Account<'info, PlotState>,
    
    #[account(
        mut,
        seeds = ["farmer".as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub farmer_account: Account<'info, FarmerAccount>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
}