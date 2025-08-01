use crate::state::{RevealData, UserBoxAccount};
use anchor_lang::prelude::*;

pub fn reveal(
    ctx: Context<Reveal>,
    reveal_data: RevealData,
    signature: Vec<u8>,
) -> Result<()> {
    let user_box_account = &mut ctx.accounts.user_box_account;
    
    // Validate that the box is in minted state
    require_eq!(user_box_account.box_state, BoxState::Minted, BlindBoxError::BoxNotMinted);
    
    // In a real implementation, you would verify the signature here
    
    // Set revealed rarity
    user_box_account.revealed_rarity = Some(reveal_data.revealed_rarity);
    
    // Update box state to revealed
    user_box_account.box_state = BoxState::Revealed;
    
    Ok(())
}

#[derive(Accounts)]
pub struct Reveal<'info> {
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