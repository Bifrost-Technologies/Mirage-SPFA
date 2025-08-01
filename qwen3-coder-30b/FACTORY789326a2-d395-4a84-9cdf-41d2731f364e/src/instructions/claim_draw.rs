use crate::state::{GameState};
use anchor_lang::prelude::*;

pub fn claim_draw(ctx: Context<ClaimDraw>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    
    // Set game status to draw by agreement
    game_state.game_status = GameStatus::DrawByAgreement;
    
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimDraw<'info> {
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