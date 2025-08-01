use crate::state::{GameState};
use anchor_lang::prelude::*;

pub fn claim_win(ctx: Context<ClaimWin>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    
    // Set game status to checkmate
    game_state.game_status = GameStatus::Checkmate;
    
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimWin<'info> {
    #[account(
        mut,
        seeds = ["game".as_bytes(), player_a.key().as_ref(), player_b.key().as_ref()],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    
    #[account(mut)]
    pub player_a: Signer<'info>,
    
    #[account(mut)]
    pub player_b: Signer<'info>,
}