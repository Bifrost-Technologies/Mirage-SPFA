use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11223344556677889900112233445566");

#[program]
pub mod onchain_blackjack {
    use super::*;

    pub fn init_table(ctx: Context<InitTable>, table_config: TableConfig) -> Result<()> {
        instructions::init_table::init_table(ctx, table_config)
    }

    pub fn join_game(ctx: Context<JoinGame>) -> Result<()> {
        instructions::join_game::join_game(ctx)
    }

    pub fn deal_cards(ctx: Context<DealCards>) -> Result<()> {
        instructions::deal_cards::deal_cards(ctx)
    }

    pub fn hit(ctx: Context<Hit>) -> Result<()> {
        instructions::hit::hit(ctx)
    }

    pub fn stand(ctx: Context<Stand>) -> Result<()> {
        instructions::stand::stand(ctx)
    }

    pub fn resolve_game(ctx: Context<ResolveGame>) -> Result<()> {
        instructions::resolve_game::resolve_game(ctx)
    }
}