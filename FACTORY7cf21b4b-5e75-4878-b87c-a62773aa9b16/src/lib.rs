use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11111122223333444455556666777788");

#[program]
pub mod treasure_hunt_quest {
    use super::*;

    pub fn init_game(ctx: Context<InitGame>, config: GameConfig) -> Result<()> {
        instructions::init_game::init_game(ctx, config)
    }

    pub fn create_player(ctx: Context<CreatePlayer>) -> Result<()> {
        instructions::create_player::create_player(ctx)
    }

    pub fn explore_cell(
        ctx: Context<ExploreCell>,
        cell_x: u32,
        cell_y: u32,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::explore_cell::explore_cell(ctx, cell_x, cell_y, signature)
    }

    pub fn claim_treasure(
        ctx: Context<ClaimTreasure>,
        item_type: String,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::claim_treasure::claim_treasure(ctx, item_type, signature)
    }
}