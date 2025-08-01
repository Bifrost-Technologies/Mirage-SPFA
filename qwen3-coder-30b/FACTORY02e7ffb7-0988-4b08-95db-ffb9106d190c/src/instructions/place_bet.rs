use crate::state::{BetData, UserBet};
use anchor_lang::prelude::*;

pub fn place_bet(
    ctx: Context<PlaceBet>,
    bet_data: BetData,
    signature: Vec<u8>,
) -> Result<()> {
    let market_state = &mut ctx.accounts.market_state;
    
    // Validate that the market is not closed
    require_eq!(market_state.is_closed, false, DerbyError::MarketNotClosed);
    
    // Validate bet amount
    require_gte!(bet_data.amount, 1, DerbyError::InvalidBetAmount);
    
    // In a real implementation, you would verify the signature here
    
    // Create or update user bet
    let user_bet = &mut ctx.accounts.user_bet;
    
    if user_bet.user == Pubkey::default() {
        // New bet
        user_bet.init(ctx.accounts.user.key(), ctx.accounts.market_state.key(), bet_data)?;
        
        // Update total bets placed in market
        market_state.total_bets_placed += bet_data.amount;
    } else {
        // Update existing bet - for simplicity, we'll just replace it
        user_bet.bet_data = bet_data;
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(
        mut,
        seeds = ["market".as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub market_state: Account<'info, MarketState>,
    
    #[account(
        init,
        payer = user,
        seeds = ["bet".as_bytes(), user.key().as_ref(), market_state.key().as_ref()],
        bump,
        space = UserBet::MAXIMUM_SIZE
    )]
    pub user_bet: Account<'info, UserBet>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}