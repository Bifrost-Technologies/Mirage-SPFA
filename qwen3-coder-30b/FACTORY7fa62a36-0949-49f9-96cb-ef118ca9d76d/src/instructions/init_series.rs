use crate::state::{SeriesConfig, SeriesConfigAccount};
use anchor_lang::prelude::*;

pub fn init_series(ctx: Context<InitSeries>, config: SeriesConfig) -> Result<()> {
    let series_config = &mut ctx.accounts.series_config;
    series_config.init(config)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitSeries<'info> {
    #[account(
        init,
        payer = authority,
        seeds = ["series".as_bytes(), authority.key().as_ref()],
        bump,
        space = SeriesConfigAccount::MAXIMUM_SIZE
    )]
    pub series_config: Account<'info, SeriesConfigAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}