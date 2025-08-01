use crate::state::{GuildConfig, GuildState};
use anchor_lang::prelude::*;

pub fn init_guild(ctx: Context<InitGuild>, config: GuildConfig) -> Result<()> {
    let guild_state = &mut ctx.accounts.guild_state;
    guild_state.init(config)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(
        init,
        payer = authority,
        seeds = ["guild".as_bytes(), authority.key().as_ref()],
        bump,
        space = GuildState::MAXIMUM_SIZE
    )]
    pub guild_state: Account<'info, GuildState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}