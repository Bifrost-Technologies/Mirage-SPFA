use crate::state::{RaceConfig, MarketState};
use anchor_lang::prelude::*;

pub fn create_race(ctx: Context<CreateRace>, config: RaceConfig) -> Result<()> {
    let market_state = &mut ctx.accounts.market_state;
    market_state.init(config)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct CreateRace<'info> {
    #[account(
        init,
        payer = authority,
        seeds = ["market".as_bytes(), authority.key().as_ref()],
        bump,
        space = MarketState::MAXIMUM_SIZE
    )]
    pub market_state: Account<'info, MarketState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}