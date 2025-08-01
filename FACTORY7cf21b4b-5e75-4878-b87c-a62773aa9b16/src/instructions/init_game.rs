use crate::state::{GameConfig, GameConfigAccount};
use anchor_lang::prelude::*;

pub fn init_game(ctx: Context<InitGame>, config: GameConfig) -> Result<()> {
    let game_config = &mut ctx.accounts.game_config;
    game_config.init(config)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitGame<'info> {
    #[account(
        init,
        payer = authority,
        seeds = ["game".as_bytes()],
        bump,
        space = GameConfigAccount::MAXIMUM_SIZE
    )]
    pub game_config: Account<'info, GameConfigAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}