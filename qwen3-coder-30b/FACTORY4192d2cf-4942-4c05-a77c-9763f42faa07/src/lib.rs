use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11223344556677889900112233445566");

#[program]
pub mod guild_conquest {
    use super::*;

    pub fn init_guild(ctx: Context<InitGuild>, config: GuildConfig) -> Result<()> {
        instructions::init_guild::init_guild(ctx, config)
    }

    pub fn join_guild(ctx: Context<JoinGuild>, member_stats: MemberStats) -> Result<()> {
        instructions::join_guild::join_guild(ctx, member_stats)
    }

    pub fn contribute_resources(
        ctx: Context<ContributeResources>,
        resource_amount: u64,
    ) -> Result<()> {
        instructions::contribute_resources::contribute_resources(ctx, resource_amount)
    }

    pub fn launch_raid(
        ctx: Context<LaunchRaid>,
        raid_outcome: RaidOutcome,
    ) -> Result<()> {
        instructions::launch_raid::launch_raid(ctx, raid_outcome)
    }

    pub fn distribute_loot(ctx: Context<DistributeLoot>) -> Result<()> {
        instructions::distribute_loot::distribute_loot(ctx)
    }
}