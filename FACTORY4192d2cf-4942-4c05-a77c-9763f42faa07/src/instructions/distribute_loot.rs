use crate::state::{RaidRecord};
use anchor_lang::prelude::*;

pub fn distribute_loot(ctx: Context<DistributeLoot>) -> Result<()> {
    let raid_record = &mut ctx.accounts.raid_record;
    
    // Validate that the raid is completed
    require_eq!(raid_record.is_completed, true, GuildConquestError::RaidNotFound);
    
    // Validate that loot hasn't been distributed yet
    require_eq!(raid_record.loot_distributed, false, GuildConquestError::RaidNotFound);
    
    // In a real implementation:
    // 1. Calculate loot distribution based on contribution scores and member stats
    // 2. Transfer loot from resource vault to members
    
    raid_record.loot_distributed = true;
    
    Ok(())
}

#[derive(Accounts)]
pub struct DistributeLoot<'info> {
    #[account(
        mut,
        seeds = ["raid".as_bytes(), guild_state.key().as_ref()],
        bump
    )]
    pub raid_record: Account<'info, RaidRecord>,
    
    #[account(mut)]
    pub guild_state: Account<'info, GuildState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}