use crate::state::{ArenaConfig, ArenaState};
use anchor_lang::prelude::*;

pub fn initialize_arena(ctx: Context<InitializeArena>, config: ArenaConfig) -> Result<()> {
    let arena_state = &mut ctx.accounts.arena_state;
    arena_state.init(config)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeArena<'info> {
    #[account(
        init,
        payer = authority,
        seeds = ["arena".as_bytes()],
        bump,
        space = ArenaState::MAXIMUM_SIZE
    )]
    pub arena_state: Account<'info, ArenaState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}