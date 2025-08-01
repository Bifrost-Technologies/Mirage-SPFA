use crate::state::{BetAccount, BetType};
use anchor_lang::prelude::*;

pub fn place_bet(
    ctx: Context<PlaceBet>,
    bet_type: BetType,
    bet_amount: u64,
) -> Result<()> {
    let bet_account = &mut ctx.accounts.bet_account;
    
    // Validate bet amount
    require_gte!(bet_amount, 1, BaccaratError::InvalidBetAmount);
    
    // In a real implementation, you would validate against game config limits
    
    // Initialize bet account
    bet_account.owner = ctx.accounts.player.key();
    bet_account.game_pubkey = ctx.accounts.game_state.key();
    bet_account.bet_type = bet_type;
    
    // Set payout multiplier based on type (simplified)
    match bet_type {
        BetType::Player => {
            // Player wins with 1:1 payout
            bet_account.payout_multiplier = 1.0;
        },
        BetType::Banker => {
            // Banker wins with 0.95x payout (house edge)
            bet_account.payout_multiplier = 0.95;
        },
        BetType::Tie => {
            // Tie wins with 8:1 payout
            bet_account.payout_multiplier = 8.0;
        }
    }
    
    bet_account.bet_amount = bet_amount;
    
    Ok(())
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(
        init,
        payer = player,
        seeds = ["bet".as_bytes(), player.key().as_ref(), game_state.key().as_ref()],
        bump,
        space = BetAccount::MAXIMUM_SIZE
    )]
    pub bet_account: Account<'info, BetAccount>,
    
    #[account(
        mut,
        seeds = ["game".as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    
    #[account(mut)]
    pub player: Signer<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}