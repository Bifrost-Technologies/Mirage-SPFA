use crate::state::{GameState};
use anchor_lang::prelude::*;

pub fn submit_move(
    ctx: Context<SubmitMove>,
    move_data: Vec<u8>,
    signature: Vec<u8>,
) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    
    // Verify it's the current player's turn
    require_eq!(
        ctx.accounts.player.key(),
        game_state.current_turn_player,
        ChessError::NotYourTurn
    );
    
    // Verify move is valid (basic check)
    require_gte!(move_data.len(), 1, ChessError::InvalidMove);
    
    // In a real implementation, you would verify the signature here
    
    // Switch turn to next player
    game_state.switch_turn()?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct SubmitMove<'info> {
    #[account(
        mut,
        seeds = ["game".as_bytes(), player_a.key().as_ref(), player_b.key().as_ref()],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    
    #[account(
        mut,
        seeds = ["player_a".as_bytes(), game_state.key().as_ref()],
        bump
    )]
    pub player_a_account: Account<'info, PlayerAccount>,
    
    #[account(
        mut,
        seeds = ["player_b".as_bytes(), game_state.key().as_ref()],
        bump
    )]
    pub player_b_account: Account<'info, PlayerAccount>,
    
    #[account(mut)]
    pub player_a: Signer<'info>,
    
    #[account(mut)]
    pub player_b: Signer<'info>,
    
    #[account(mut)]
    pub player: Signer<'info>, // The current turn player
    
}