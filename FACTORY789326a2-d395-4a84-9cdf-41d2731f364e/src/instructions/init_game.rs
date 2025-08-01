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
        payer = player_a,
        seeds = ["game".as_bytes(), player_a.key().as_ref(), player_b.key().as_ref()],
        bump,
        space = GameState::MAXIMUM_SIZE
    )]
    pub game_state: Account<'info, GameState>,
    
    #[account(
        init,
        payer = player_a,
        seeds = ["player_a".as_bytes(), game_state.key().as_ref()],
        bump,
        space = PlayerAccount::MAXIMUM_SIZE
    )]
    pub player_a_account: Account<'info, PlayerAccount>,
    
    #[account(
        init,
        payer = player_a,
        seeds = ["player_b".as_bytes(), game_state.key().as_ref()],
        bump,
        space = PlayerAccount::MAXIMUM_SIZE
    )]
    pub player_b_account: Account<'info, PlayerAccount>,
    
    #[account(
        init,
        payer = player_a,
        seeds = ["moves".as_bytes(), game_state.key().as_ref()],
        bump,
        space = MoveHistory::MAXIMUM_SIZE
    )]
    pub move_history: Account<'info, MoveHistory>,
    
    #[account(mut)]
    pub player_a: Signer<'info>,
    
    #[account(mut)]
    pub player_b: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}