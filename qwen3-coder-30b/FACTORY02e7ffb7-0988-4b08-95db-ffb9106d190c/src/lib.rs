use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11122233344455566677788899900011");

#[program]
pub mod prediction_market_derby {
    use super::*;

    pub fn create_race(ctx: Context<CreateRace>, config: RaceConfig) -> Result<()> {
        instructions::create_race::create_race(ctx, config)
    }

    pub fn place_bet(
        ctx: Context<PlaceBet>,
        bet_data: BetData,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::place_bet::place_bet(ctx, bet_data, signature)
    }

    pub fn close_market(ctx: Context<CloseMarket>, outcome: OutcomeEnum) -> Result<()> {
        instructions::close_market::close_market(ctx, outcome)
    }

    pub fn payout(ctx: Context<Payout>) -> Result<()> {
        instructions::payout::payout(ctx)
    }
}