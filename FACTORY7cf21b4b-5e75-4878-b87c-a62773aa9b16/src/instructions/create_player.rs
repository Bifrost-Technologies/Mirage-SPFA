use crate::state::{PlayerState};
use anchor_lang::prelude::*;

pub fn create_player(ctx: Context<CreatePlayer>) -> Result<()> {
    let player_state = &mut ctx.accounts.player_state;
    player_state.init(ctx.accounts.authority.key())?;
    
    // Increment total players in game config
    let game_config = &mut ctx.accounts.game_config;
    if let Some(count) = game_config.total_players.checked_add(1) {
        game_config.total_players = count;
    } else {
        return Err(TreasureHuntError::PlayerAlreadyExists.into());
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct CreatePlayer<'info> {
    #[account(
        init,
        payer = authority,
        seeds = ["player".as_bytes(), authority.key().as_ref()],
        bump,
        space = PlayerState::MAXIMUM_SIZE
    )]
    pub player_state: Account<'info, PlayerState>,
    
    #[account(mut)]
    pub game_config: Account<'info, GameConfigAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}