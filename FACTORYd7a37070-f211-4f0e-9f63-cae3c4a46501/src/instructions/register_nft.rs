use crate::state::{CarStats, RacerNFT};
use anchor_lang::prelude::*;

pub fn register_nft(ctx: Context<RegisterNFT>, car_stats: CarStats) -> Result<()> {
    let racer_nft = &mut ctx.accounts.racer_nft;
    
    // Validate car stats
    require_gte!(car_stats.speed, 1, RacingError::InvalidCarStats);
    require_gte!(car_stats.acceleration, 1, RacingError::InvalidCarStats);
    require_gte!(car_stats.handling, 1, RacingError::InvalidCarStats);
    require_gte!(car_stats.durability, 1, RacingError::InvalidCarStats);
    
    racer_nft.init(
        ctx.accounts.owner.key(),
        ctx.accounts.nft_mint.key(),
        car_stats
    )?;
    
    // Increment total registered NFTs in league config
    let league_config = &mut ctx.accounts.league_config;
    if let Some(count) = league_config.total_registered_nfts.checked_add(1) {
        league_config.total_registered_nfts = count;
    } else {
        return Err(RacingError::InvalidCarStats.into());
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct RegisterNFT<'info> {
    #[account(
        init,
        payer = owner,
        seeds = ["racer".as_bytes(), nft_mint.key().as_ref()],
        bump,
        space = RacerNFT::MAXIMUM_SIZE
    )]
    pub racer_nft: Account<'info, RacerNFT>,
    
    #[account(mut)]
    pub league_config: Account<'info, LeagueConfig>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub nft_mint: AccountInfo<'info>, // The NFT mint account
    
    pub system_program: Program<'info, System>,
}