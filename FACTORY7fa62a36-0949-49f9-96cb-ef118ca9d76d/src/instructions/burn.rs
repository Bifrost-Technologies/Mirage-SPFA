use crate::state::{UserBoxAccount};
use anchor_lang::prelude::*;

pub fn burn(ctx: Context<Burn>) -> Result<()> {
    let user_box_account = &mut ctx.accounts.user_box_account;
    
    // Validate that the box is in revealed state
    require_eq!(user_box_account.box_state, BoxState::Revealed, BlindBoxError::BoxNotMinted);
    
    // In a real implementation:
    // 1. Burn the NFT mint account
    // 2. Transfer any associated tokens to treasury
    
    Ok(())
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(
        mut,
        seeds = ["box".as_bytes(), owner.key().as_ref(), series_config.key().as_ref()],
        bump
    )]
    pub user_box_account: Account<'info, UserBoxAccount>,
    
    #[account(mut)]
    pub series_config: Account<'info, SeriesConfigAccount>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
}