use crate::state::{NFTTraits, ParentAccount};
use anchor_lang::prelude::*;

pub fn deposit_nft(ctx: Context<DepositNFT>, nft_traits: NFTTraits) -> Result<()> {
    let parent_account = &mut ctx.accounts.parent_account;
    
    // Initialize the parent account with NFT details
    parent_account.init(
        ctx.accounts.owner.key(),
        ctx.accounts.nft_mint.key(),
        nft_traits
    )?;
    
    // Increment total NFTs in farm config
    let farm_config = &mut ctx.accounts.farm_config;
    if let Some(count) = farm_config.total_nfts.checked_add(1) {
        farm_config.total_nfts = count;
    } else {
        return Err(NftBreederError::NftAlreadyDeposited.into());
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct DepositNFT<'info> {
    #[account(
        init,
        payer = owner,
        seeds = ["parent".as_bytes(), nft_mint.key().as_ref()],
        bump,
        space = ParentAccount::MAXIMUM_SIZE
    )]
    pub parent_account: Account<'info, ParentAccount>,
    
    #[account(mut)]
    pub farm_config: Account<'info, FarmConfigAccount>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub nft_mint: AccountInfo<'info>, // The NFT mint account
    
    pub system_program: Program<'info, System>,
}