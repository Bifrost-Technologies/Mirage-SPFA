use crate::state::{RaceResult};
use anchor_lang::prelude::*;

pub fn finalize_race(ctx: Context<FinalizeRace>) -> Result<()> {
    let race_result = &mut ctx.accounts.race_result;
    
    // Finalize the race results
    race_result.finalize()?;
    
    // In a real implementation:
    // 1. Distribute prizes based on final positions
    // 2. Update leaderboard
    
    Ok(())
}

#[derive(Accounts)]
pub struct FinalizeRace<'info> {
    #[account(
        mut,
        seeds = ["race".as_bytes(), league_config.race_config.race_name.as_bytes()],
        bump
    )]
    pub race_result: Account<'info, RaceResult>,
    
    #[account(
        mut,
        seeds = ["league".as_bytes()],
        bump
    )]
    pub league_config: Account<'info, LeagueConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}