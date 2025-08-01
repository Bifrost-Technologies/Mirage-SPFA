use crate::state::{PlayerAccount, PlayerStats};
use anchor_lang::prelude::*;

pub fn register_player(ctx: Context<RegisterPlayer>, player_stats: PlayerStats) -> Result<()> {
    let player_account = &mut ctx.accounts.player_account;
    player_account.init(ctx.accounts.authority.key(), player_stats)?;
    
    // Increment total players in arena state
    let arena_state = &mut ctx.accounts.arena_state;
    if let Some(count) = arena_state.total_players.checked_add(1) {
        arena_state.total_players = count;
    } else {
        return Err(ArenaError::PlayerAlreadyRegistered.into());
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct RegisterPlayer<'info> {
    #[account(
        init,
        payer = authority,
        seeds = ["player".as_bytes(), authority.key().as_ref()],
        bump,
        space = PlayerAccount::MAXIMUM_SIZE
    )]
    pub player_account: Account<'info, PlayerAccount>,
    
    #[account(mut)]
    pub arena_state: Account<'info, ArenaState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}