use crate::state::{BetTypeEnum, BetAccount};
use anchor_lang::prelude::*;

pub fn place_bet(
    ctx: Context<PlaceBet>,
    bet_type: BetTypeEnum,
    bet_amount: u64,
) -> Result<()> {
    let bet_account = &mut ctx.accounts.bet_account;
    
    // Validate bet amount
    require_gte!(bet_amount, 1, RouletteError::InvalidBetAmount);
    
    // In a real implementation, you would validate against game config limits
    
    // Initialize bet account
    bet_account.owner = ctx.accounts.player.key();
    bet_account.wheel_pubkey = ctx.accounts.wheel_config.key();
    bet_account.bet_type = bet_type;
    
    // Set payout multiplier based on type (simplified)
    let multiplier = match bet_type {
        BetTypeEnum::Red | BetTypeEnum::Black => 1.0,
        BetTypeEnum::Even | BetTypeEnum::Odd => 1.0,
        BetTypeEnum::Low | BetTypeEnum::High => 1.0,
        _ => 2.0, // Dozens and columns
    };
    
    bet_account.payout_multiplier = multiplier;
    bet_account.bet_amount = bet_amount;
    
    // Update total bets placed in wheel config
    let wheel_config = &mut ctx.accounts.wheel_config;
    
    if let Some(total) = wheel_config.total_bets_placed.checked_add(bet_amount) {
        wheel_config.total_bets_placed = total;
    } else {
        return Err(RouletteError::InvalidBetAmount.into());
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(
        init,
        payer = player,
        seeds = ["bet".as_bytes(), player.key().as_ref(), wheel_config.key().as_ref()],
        bump,
        space = BetAccount::MAXIMUM_SIZE
    )]
    pub bet_account: Account<'info, BetAccount>,
    
    #[account(
        mut,
        seeds = ["wheel".as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub wheel_config: Account<'info, WheelConfigAccount>,
    
    #[account(mut)]
    pub player: Signer<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}