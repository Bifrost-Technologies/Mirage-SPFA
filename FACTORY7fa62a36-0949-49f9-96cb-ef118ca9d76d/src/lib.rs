use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11223344556677889900112233445566");

#[program]
pub mod blind_box_mint {
    use super::*;

    pub fn init_series(ctx: Context<InitSeries>, config: SeriesConfig) -> Result<()> {
        instructions::init_series::init_series(ctx, config)
    }

    pub fn mint_blind_box(
        ctx: Context<MintBlindBox>,
        box_data: BoxData,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::mint_blind_box::mint_blind_box(ctx, box_data, signature)
    }

    pub fn reveal(
        ctx: Context<Reveal>,
        reveal_data: RevealData,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::reveal::reveal(ctx, reveal_data, signature)
    }

    pub fn burn(ctx: Context<Burn>) -> Result<()> {
        instructions::burn::burn(ctx)
    }
}