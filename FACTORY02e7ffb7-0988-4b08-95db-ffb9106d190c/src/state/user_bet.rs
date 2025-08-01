use crate::state::{BetData};
use anchor_lang::prelude::*;

#[account]
pub struct UserBet {
    pub user: Pubkey,
    pub market_pubkey: Pubkey,
    pub bet_data: BetData,
    pub amount_won: u64,
    pub is_paid_out: bool,
}

impl UserBet {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, user_pubkey: Pubkey, market_pubkey: Pubkey, bet_data: BetData) -> Result<()> {
        require_eq!(self.user, Pubkey::default(), DerbyError::UserAlreadyBet);
        
        self.user = user_pubkey;
        self.market_pubkey = market_pubkey;
        self.bet_data = bet_data;
        self.amount_won = 0;
        self.is_paid_out = false;
        
        Ok(())
    }
}