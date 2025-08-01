use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11223344556677889900112233445566");

#[program]
pub mod onchain_roulette {
    use super::*;

    pub fn init_wheel(ctx: Context<InitWheel>, config: WheelConfig) -> Result<()> {
        instructions::init_wheel::init_wheel(ctx, config)
    }

    pub fn place_bet(
        ctx: Context<PlaceBet>,
        bet_type: BetTypeEnum,
        bet_amount: u64,
    ) -> Result<()> {
        instructions::place_bet::place_bet(ctx, bet_type, bet_amount)
    }

    pub fn spin_wheel(ctx: Context<SpinWheel>) -> Result<()> {
        instructions::spin_wheel::spin_wheel(ctx)
    }

    pub fn resolve_bets(ctx: Context<ResolveBets>) -> Result<()> {
        instructions::resolve_bets::resolve_bets(ctx)
    }
}