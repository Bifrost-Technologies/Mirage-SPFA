use crate::state::{RaceConfig, LeagueConfig};
use anchor_lang::prelude::*;

pub fn start_race(ctx: Context<StartRace>, race_config: RaceConfig) -> Result<()> {
    let league_config = &mut ctx.accounts.league_config;
    
    // Validate that race is not already active
    require_eq!(league_config.is_race_active, false, RacingError::RaceNotStarted);
    
    // Update league config with new race config
    league_config.race_config = race_config;
    
    // Mark race as active
    league_config.is_race_active = true;
    
    Ok(())
}

#[derive(Accounts)]
pub struct StartRace<'info> {
    #[account(
        mut,
        seeds = ["league".as_bytes()],
        bump
    )]
    pub league_config: Account<'info, LeagueConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}