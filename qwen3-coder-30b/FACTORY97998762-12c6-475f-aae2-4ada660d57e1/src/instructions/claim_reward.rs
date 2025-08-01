use crate::state::{MatchState, PlayerState};
use anchor_lang::prelude::*;

pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
    let match_state = &mut ctx.accounts.match_state;
    
    // In a real implementation:
    // 1. Calculate final scores for players
    // 2. Transfer rewards from reward vault to winners
    
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(
        mut,
        seeds = ["match".as_bytes(), player_a.key().as_ref(), player_b.key().as_ref()],
        bump
    )]
    pub match_state: Account<'info, MatchState>,
    
    #[account(
        mut,
        seeds = ["player".as_bytes(), player.key().as_ref(), match_state.key().as_ref()],
        bump
    )]
    pub player_state: Account<'info, PlayerState>,
    
    #[account(mut)]
    pub player_a: Signer<'info>,
    
    #[account(mut)]
    pub player_b: Signer<'info>,
    
}