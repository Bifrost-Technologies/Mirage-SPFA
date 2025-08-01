use crate::state::{RaidOutcome, RaidRecord};
use anchor_lang::prelude::*;

pub fn launch_raid(ctx: Context<LaunchRaid>, raid_outcome: RaidOutcome) -> Result<()> {
    let raid_record = &mut ctx.accounts.raid_record;
    
    // Validate that the raid outcome is valid
    require_gte!(raid_outcome.loot_value, 1, GuildConquestError::InvalidRaidOutcome);
    
    // Initialize the raid record
    raid_record.init(ctx.accounts.guild_state.key(), raid_outcome)?;
    
    // Launch the raid
    raid_record.launch()?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct LaunchRaid<'info> {
    #[account(
        init,
        payer = authority,
        seeds = ["raid".as_bytes(), guild_state.key().as_ref()],
        bump,
        space = RaidRecord::MAXIMUM_SIZE
    )]
    pub raid_record: Account<'info, RaidRecord>,
    
    #[account(mut)]
    pub guild_state: Account<'info, GuildState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}