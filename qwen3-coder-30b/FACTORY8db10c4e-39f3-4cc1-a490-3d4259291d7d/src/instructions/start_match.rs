use crate::state::{MatchData, MatchRecord};
use anchor_lang::prelude::*;

pub fn start_match(ctx: Context<StartMatch>, match_data: MatchData) -> Result<()> {
    let match_record = &mut ctx.accounts.match_record;
    
    // Get player accounts to retrieve stats
    let player_a_account = &ctx.accounts.player_a_account;
    let player_b_account = &ctx.accounts.player_b_account;

    // Initialize the match record
    match_record.init(
        ctx.accounts.arena_state.total_matches + 1,
        ctx.accounts.player_a.key(),
        ctx.accounts.player_b.key(),
        player_a_account.stats.clone(),
        player_b_account.stats.clone(),
        match_data
    )?;
    
    // Increment total matches in arena state
    let arena_state = &mut ctx.accounts.arena_state;
    if let Some(count) = arena_state.total_matches.checked_add(1) {
        arena_state.total_matches = count;
    } else {
        return Err(ArenaError::MatchNotFound.into());
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct StartMatch<'info> {
    #[account(
        init,
        payer = player_a,
        seeds = ["match".as_bytes(), player_a.key().as_ref(), player_b.key().as_ref()],
        bump,
        space = MatchRecord::MAXIMUM_SIZE
    )]
    pub match_record: Account<'info, MatchRecord>,
    
    #[account(mut)]
    pub arena_state: Account<'info, ArenaState>,
    
    #[account(
        mut,
        seeds = ["player".as_bytes(), player_a.key().as_ref()],
        bump
    )]
    pub player_a_account: Account<'info, PlayerAccount>,
    
    #[account(
        mut,
        seeds = ["player".as_bytes(), player_b.key().as_ref()],
        bump
    )]
    pub player_b_account: Account<'info, PlayerAccount>,
    
    #[account(mut)]
    pub player_a: Signer<'info>,
    
    #[account(mut)]
    pub player_b: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}