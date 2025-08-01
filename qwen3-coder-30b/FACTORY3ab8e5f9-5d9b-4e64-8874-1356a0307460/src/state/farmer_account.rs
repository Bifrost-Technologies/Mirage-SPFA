use anchor_lang::prelude::*;

#[account]
pub struct FarmerAccount {
    pub owner: Pubkey,
    pub total_yield: u64,
    pub total_revenue: u64,
}

impl FarmerAccount {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, owner_pubkey: Pubkey) -> Result<()> {
        self.owner = owner_pubkey;
        self.total_yield = 0;
        self.total_revenue = 0;
        
        Ok(())
    }
}