use crate::state::{GameConfig, GameState};
use anchor_lang::prelude::*;

pub fn init_game(ctx: Context<InitGame>, game_config: GameConfig) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    game_state.init(game_config)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitGame<'info> {
    #[account(
        init,
        payer = authority,
        seeds = ["game".as_bytes(), authority.key().as_ref()],
        bump,
        space = GameState::MAXIMUM_SIZE
    )]
    pub game_state: Account<'info, GameState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}