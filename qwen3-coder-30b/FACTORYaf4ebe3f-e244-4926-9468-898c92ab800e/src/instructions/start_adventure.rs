use crate::state::{AdventureState};
use anchor_lang::prelude::*;

pub fn start_adventure(ctx: Context<StartAdventure>, seed: u64) -> Result<()> {
    let adventure_state = &mut ctx.accounts.adventure_state;
    adventure_state.init(seed)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct StartAdventure<'info> {
    #[account(
        init,
        payer = player,
        seeds = ["adventure".as_bytes(), player.key().as_ref()],
        bump,
        space = AdventureState::MAXIMUM_SIZE
    )]
    pub adventure_state: Account<'info, AdventureState>,
    
    #[account(mut)]
    pub player: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}