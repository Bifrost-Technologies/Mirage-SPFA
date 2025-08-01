use crate::state::{AdventureState};
use anchor_lang::prelude::*;

pub fn end_adventure(ctx: Context<EndAdventure>) -> Result<()> {
    let adventure_state = &mut ctx.accounts.adventure_state;
    
    // End the adventure
    adventure_state.end()?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct EndAdventure<'info> {
    #[account(
        mut,
        seeds = ["adventure".as_bytes(), player.key().as_ref()],
        bump
    )]
    pub adventure_state: Account<'info, AdventureState>,
    
    #[account(mut)]
    pub player: Signer<'info>,
}