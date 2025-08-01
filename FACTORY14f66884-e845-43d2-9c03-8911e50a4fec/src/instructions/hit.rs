use crate::state::{PlayerAccount};
use anchor_lang::prelude::*;

pub fn hit(ctx: Context<Hit>) -> Result<()> {
    let player_account = &mut ctx.accounts.player_account;
    
    // Validate that player's hand is active
    require_eq!(player_account.hand_state, HandState::Active, BlackjackError::GameNotInProgress);
    
    // In a real implementation:
    // 1. Deal another card to the player
    // 2. Check if player busted (went over 21)
    
    Ok(())
}

#[derive(Accounts)]
pub struct Hit<'info> {
    #[account(
        mut,
        seeds = ["player".as_bytes(), player.key().as_ref(), table_state.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>,
    
    #[account(
        mut,
        seeds = ["table".as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub table_state: Account<'info, TableState>,
    
    #[account(mut)]
    pub player: Signer<'info>,
    
}