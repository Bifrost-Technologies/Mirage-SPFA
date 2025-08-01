use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11112222333344445555666677778888");

#[program]
pub mod nft_breeder_farm {
    use super::*;

    pub fn init_farm(ctx: Context<InitFarm>, config: FarmConfig) -> Result<()> {
        instructions::init_farm::init_farm(ctx, config)
    }

    pub fn deposit_nft(ctx: Context<DepositNFT>, nft_traits: NFTTraits) -> Result<()> {
        instructions::deposit_nft::deposit_nft(ctx, nft_traits)
    }

    pub fn breed_nft(
        ctx: Context<BreedNFT>,
        breed_params: BreedParams,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::breed_nft::breed_nft(ctx, breed_params, signature)
    }

    pub fn withdraw_offspring(ctx: Context<WithdrawOffspring>) -> Result<()> {
        instructions::withdraw_offspring::withdraw_offspring(ctx)
    }
}