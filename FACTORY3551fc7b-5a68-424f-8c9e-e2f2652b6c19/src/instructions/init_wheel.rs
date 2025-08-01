use crate::state::{WheelConfig, WheelConfigAccount};
use anchor_lang::prelude::*;

pub fn init_wheel(ctx: Context<InitWheel>, config: WheelConfig) -> Result<()> {
    let wheel_config = &mut ctx.accounts.wheel_config;
    wheel_config.init(config)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitWheel<'info> {
    #[account(
        init,
        payer = authority,
        seeds = ["wheel".as_bytes(), authority.key().as_ref()],
        bump,
        space = WheelConfigAccount::MAXIMUM_SIZE
    )]
    pub wheel_config: Account<'info, WheelConfigAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}