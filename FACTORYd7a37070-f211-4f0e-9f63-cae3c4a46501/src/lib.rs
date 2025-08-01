use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11223344556677889900112233445566");

#[program]
pub mod nft_racing_league {
    use super::*;

    pub fn register_nft(ctx: Context<RegisterNFT>, car_stats: CarStats) -> Result<()> {
        instructions::register_nft::register_nft(ctx, car_stats)
    }

    pub fn start_race(ctx: Context<StartRace>, race_config: RaceConfig) -> Result<()> {
        instructions::start_race::start_race(ctx, race_config)
    }

    pub fn submit_time(
        ctx: Context<SubmitTime>,
        time_in_seconds: u32,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::submit_time::submit_time(ctx, time_in_seconds, signature)
    }

    pub fn finalize_race(ctx: Context<FinalizeRace>) -> Result<()> {
        instructions::finalize_race::finalize_race(ctx)
    }
}