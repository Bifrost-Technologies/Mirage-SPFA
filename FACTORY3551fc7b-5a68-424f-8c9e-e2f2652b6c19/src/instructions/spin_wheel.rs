use crate::state::{WheelConfigAccount};
use anchor_lang::prelude::*;

pub fn spin_wheel(ctx: Context<SpinWheel>) -> Result<()> {
    let wheel_config = &mut ctx.accounts.wheel_config;
    
    // In a real implementation with VRF:
    // 1. Use VRF oracle to generate random number
    // 2. Determine color based on number (0=green, 1-10=red, 11-36=black)
    
    Ok(())
}

#[derive(Accounts)]
pub struct SpinWheel<'info> {
    #[account(
        mut,
        seeds = ["wheel".as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub wheel_config: Account<'info, WheelConfigAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}