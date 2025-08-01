use crate::state::{PlayerAccount};
use anchor_lang::prelude::*;

pub fn stand(ctx: Context<Stand>) -> Result<()> {
    let player_account = &mut ctx.accounts.player_account;
    
    // Validate that player's hand is active
    require_eq!(player_account.hand_state, HandState::Active, BlackjackError::GameNotInProgress);
    
    // Set player's hand state to stand
    player_account.hand_state = HandState::Stand;
    
    Ok(())
}

#[derive(Accounts)]
pub struct Stand<'info> {
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