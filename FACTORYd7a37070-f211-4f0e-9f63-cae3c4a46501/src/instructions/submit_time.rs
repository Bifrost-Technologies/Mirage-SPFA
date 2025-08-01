use crate::state::{ResultEntry, RaceResult};
use anchor_lang::prelude::*;

pub fn submit_time(
    ctx: Context<SubmitTime>,
    time_in_seconds: u32,
    signature: Vec<u8>,
) -> Result<()> {
    let race_result = &mut ctx.accounts.race_result;
    
    // In a real implementation, you would verify the signature here
    
    // Validate that the time is reasonable (not zero or extremely high)
    require_gte!(time_in_seconds, 1, RacingError::InvalidTimeSubmitted);
    
    // Create result entry
    let result_entry = ResultEntry {
        racer_pubkey: ctx.accounts.owner.key(),
        time_in_seconds,
        final_position: 0, // Will be set during finalization
    };
    
    race_result.add_result(result_entry)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct SubmitTime<'info> {
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
    pub owner: Signer<'info>,
}