use crate::state::{TableState};
use anchor_lang::prelude::*;

pub fn deal_cards(ctx: Context<DealCards>) -> Result<()> {
    let table_state = &mut ctx.accounts.table_state;
    
    // Validate that game is active
    require_eq!(table_state.is_game_active, true, BlackjackError::GameNotInProgress);
    
    // In a real implementation with VRF:
    // 1. Use VRF oracle to shuffle deck
    // 2. Deal cards to players and dealer
    
    Ok(())
}

#[derive(Accounts)]
pub struct DealCards<'info> {
    #[account(
        mut,
        seeds = ["table".as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub table_state: Account<'info, TableState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}