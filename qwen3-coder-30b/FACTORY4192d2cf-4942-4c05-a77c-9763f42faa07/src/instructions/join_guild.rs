use crate::state::{MemberStats, MemberState};
use anchor_lang::prelude::*;

pub fn join_guild(ctx: Context<JoinGuild>, member_stats: MemberStats) -> Result<()> {
    let member_state = &mut ctx.accounts.member_state;
    member_state.init(
        ctx.accounts.authority.key(),
        ctx.accounts.guild_state.key(),
        member_stats
    )?;
    
    // Increment total members in guild state
    let guild_state = &mut ctx.accounts.guild_state;
    if let Some(count) = guild_state.total_members.checked_add(1) {
        guild_state.total_members = count;
    } else {
        return Err(GuildConquestError::MemberAlreadyInGuild.into());
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct JoinGuild<'info> {
    #[account(
        init,
        payer = authority,
        seeds = ["member".as_bytes(), authority.key().as_ref(), guild_state.key().as_ref()],
        bump,
        space = MemberState::MAXIMUM_SIZE
    )]
    pub member_state: Account<'info, MemberState>,
    
    #[account(mut)]
    pub guild_state: Account<'info, GuildState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}