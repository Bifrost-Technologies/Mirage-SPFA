use crate::state::{MatchRecord, PlayerStats};
use anchor_lang::prelude::*;

pub fn resolve_match(ctx: Context<ResolveMatch>) -> Result<()> {
    let match_record = &mut ctx.accounts.match_record;
    
    // Verify match is not already resolved
    require_eq!(match_record.is_resolved, false, ArenaError::MatchAlreadyResolved);
    
    // In a real implementation, you would:
    // 1. Validate the final game state
    // 2. Determine winner/loser/draw based on game rules
    // 3. Update ELO ratings for both players
    
    match_record.is_resolved = true;
    
    Ok(())
}

#[derive(Accounts)]
pub struct ResolveMatch<'info> {
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
}