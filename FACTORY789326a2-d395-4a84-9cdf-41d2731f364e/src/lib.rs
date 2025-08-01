use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11223344556677889900112233445566");

#[program]
pub mod onchain_chess {
    use super::*;

    pub fn init_game(ctx: Context<InitGame>, game_config: GameConfig) -> Result<()> {
        instructions::init_game::init_game(ctx, game_config)
    }

    pub fn submit_move(
        ctx: Context<SubmitMove>,
        move_data: Vec<u8>,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::submit_move::submit_move(ctx, move_data, signature)
    }

    pub fn claim_win(ctx: Context<ClaimWin>) -> Result<()> {
        instructions::claim_win::claim_win(ctx)
    }

    pub fn claim_draw(ctx: Context<ClaimDraw>) -> Result<()> {
        instructions::claim_draw::claim_draw(ctx)
    }
}