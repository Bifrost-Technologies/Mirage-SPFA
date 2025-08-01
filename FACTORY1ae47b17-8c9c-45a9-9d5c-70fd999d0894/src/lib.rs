use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11223344556677889900112233445566");

#[program]
pub mod onchain_baccarat {
    use super::*;

    pub fn init_game(ctx: Context<InitGame>, game_config: GameConfig) -> Result<()> {
        instructions::init_game::init_game(ctx, game_config)
    }

    pub fn place_bet(
        ctx: Context<PlaceBet>,
        bet_type: BetType,
        bet_amount: u64,
    ) -> Result<()> {
        instructions::place_bet::place_bet(ctx, bet_type, bet_amount)
    }

    pub fn deal_hand(ctx: Context<DealHand>) -> Result<()> {
        instructions::deal_hand::deal_hand(ctx)
    }

    pub fn resolve_bet(ctx: Context<ResolveBet>) -> Result<()> {
        instructions::resolve_bet::resolve_bet(ctx)
    }
}