use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11223344556677889900112233445566");

#[program]
pub mod defi_farming_simulator {
    use super::*;

    pub fn init_plot(ctx: Context<InitPlot>, crop_type: CropType) -> Result<()> {
        instructions::init_plot::init_plot(ctx, crop_type)
    }

    pub fn plant_crop(
        ctx: Context<PlantCrop>,
        seed_amount: u64,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::plant_crop::plant_crop(ctx, seed_amount, signature)
    }

    pub fn harvest(ctx: Context<Harvest>) -> Result<()> {
        instructions::harvest::harvest(ctx)
    }

    pub fn sell_crop(
        ctx: Context<SellCrop>,
        crop_amount: u64,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::sell_crop::sell_crop(ctx, crop_amount, signature)
    }
}