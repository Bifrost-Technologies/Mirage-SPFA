use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11111111111111222222333334444555");

#[program]
pub mod on_chain_arena {
    use super::*;

    pub fn initialize_arena(ctx: Context<InitializeArena>, config: ArenaConfig) -> Result<()> {
        instructions::initialize_arena::initialize_arena(ctx, config)
    }

    pub fn register_player(ctx: Context<RegisterPlayer>, player_stats: PlayerStats) -> Result<()> {
        instructions::register_player::register_player(ctx, player_stats)
    }

    pub fn start_match(ctx: Context<StartMatch>, match_data: MatchData) -> Result<()> {
        instructions::start_match::start_match(ctx, match_data)
    }

    pub fn submit_move(
        ctx: Context<SubmitMove>,
        move_data: Vec<u8>,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::submit_move::submit_move(ctx, move_data, signature)
    }

    pub fn resolve_match(ctx: Context<ResolveMatch>) -> Result<()> {
        instructions::resolve_match::resolve_match(ctx)
    }
}