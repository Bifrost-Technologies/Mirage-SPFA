use crate::state::{PlayerAccount};
use anchor_lang::prelude::*;

pub fn join_game(ctx: Context<JoinGame>) -> Result<()> {
    let table_state = &mut ctx.accounts.table_state;
    
    // Add player to game
    table_state.add_player(ctx.accounts.player.key())?;
    
    // Initialize player account
    let player_account = &mut ctx.accounts.player_account;
    
    player_account.owner = ctx.accounts.player.key();
    player_account.table_pubkey = ctx.accounts.table_state.key();
    player_account.player_hand = Vec::new();
    player_account.hand_state = HandState::Active;
    
    Ok(())
}

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(
        mut,
        seeds = ["table".as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub table_state: Account<'info, TableState>,
    
    #[account(
        init,
        payer = player,
        seeds = ["player".as_bytes(), player.key().as_ref(), table_state.key().as_ref()],
        bump,
        space = PlayerAccount::MAXIMUM_SIZE
    )]
    pub player_account: Account<'info, PlayerAccount>,
    
    #[account(mut)]
    pub player: Signer<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}