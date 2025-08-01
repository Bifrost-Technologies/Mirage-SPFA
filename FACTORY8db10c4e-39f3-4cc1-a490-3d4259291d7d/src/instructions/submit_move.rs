use crate::state::{MatchRecord};
use anchor_lang::prelude::*;

pub fn submit_move(ctx: Context<SubmitMove>, move_data: Vec<u8>, signature: Vec<u8>) -> Result<()> {
    let match_record = &mut ctx.accounts.match_record;
    
    // Verify it's the current player's turn
    require_eq!(
        ctx.accounts.player.key(),
        match_record.current_turn_player,
        ArenaError::NotYourTurn
    );
    
    // Verify move is valid (basic check)
    require_gte!(move_data.len(), 1, ArenaError::InvalidMove);
    
    // In a real implementation, you would verify the signature here
    
    // Switch turn to next player
    match_record.switch_turn()?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct SubmitMove<'info> {
    #[account(
        mut,
        seeds = ["match".as_bytes(), player_a.key().as_ref(), player_b.key().as_ref()],
        bump
    )]
    pub match_record: Account<'info, MatchRecord>,
    
    #[account(
        mut,
        seeds = ["player".as_bytes(), player_a.key().as_ref()],
        bump
    )]
    pub player_a_account: Account<'info, PlayerAccount>,
    
    #[account(
        mut,
        seeds = ["player".as_bytes(), player_b.key().as_ref()],
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