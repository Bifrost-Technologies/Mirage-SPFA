use crate::state::{FarmConfig, FarmConfigAccount};
use anchor_lang::prelude::*;

pub fn init_farm(ctx: Context<InitFarm>, config: FarmConfig) -> Result<()> {
    let farm_config = &mut ctx.accounts.farm_config;
    farm_config.init(config)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitFarm<'info> {
    #[account(
        init,
        payer = authority,
        seeds = ["farm".as_bytes()],
        bump,
        space = FarmConfigAccount::MAXIMUM_SIZE
    )]
    pub farm_config: Account<'info, FarmConfigAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}