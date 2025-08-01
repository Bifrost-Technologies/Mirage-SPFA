use crate::state::{OffspringMint};
use anchor_lang::prelude::*;

pub fn withdraw_offspring(ctx: Context<WithdrawOffspring>) -> Result<()> {
    let offspring_mint = &mut ctx.accounts.offspring_mint;
    
    // Verify the offspring has not been claimed yet
    require_eq!(offspring_mint.is_claimed, false, NftBreederError::NftAlreadyDeposited);
    
    // Mark as claimed
    offspring_mint.is_claimed = true;
    
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawOffspring<'info> {
    #[account(
        mut,
        seeds = ["offspring".as_bytes(), parent_a_mint.key().as_ref(), parent_b_mint.key().as_ref()],
        bump
    )]
    pub offspring_mint: Account<'info, OffspringMint>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub parent_a_mint: AccountInfo<'info>, // The NFT mint account of parent A
    
    pub parent_b_mint: AccountInfo<'info>, // The NFT mint account of parent B
    
}