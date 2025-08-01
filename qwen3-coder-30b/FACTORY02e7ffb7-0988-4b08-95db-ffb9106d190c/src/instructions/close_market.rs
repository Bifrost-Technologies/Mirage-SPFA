use crate::state::{MarketState, OutcomeEnum};
use anchor_lang::prelude::*;

pub fn close_market(ctx: Context<CloseMarket>, outcome: OutcomeEnum) -> Result<()> {
    let market_state = &mut ctx.accounts.market_state;
    
    // Validate that the outcome is valid
    require_eq!(outcome, OutcomeEnum::Completed, DerbyError::InvalidOutcome);
    
    // Close the market
    market_state.close()?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct CloseMarket<'info> {
    #[account(
        mut,
        seeds = ["market".as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub market_state: Account<'info, MarketState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}