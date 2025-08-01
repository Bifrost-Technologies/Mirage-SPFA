use crate::state::{NFTTraits, CooldownState};
use anchor_lang::prelude::*;

#[account]
pub struct ParentAccount {
    pub owner: Pubkey,
    pub nft_mint: Pubkey,
    pub traits: NFTTraits,
    pub cooldown_state: CooldownState,
    pub is_deposited: bool,
}

impl ParentAccount {
    pub const MAXIMUM_SIZE: usize = 5000;

    pub fn init(&mut self, owner_pubkey: Pubkey, nft_mint: Pubkey, traits: NFTTraits) -> Result<()> {
        require_eq!(self.is_deposited, false, NftBreederError::NftAlreadyDeposited);
        
        self.owner = owner_pubkey;
        self.nft_mint = nft_mint;
        self.traits = traits;
        self.cooldown_state = CooldownState {
            last_breed_time: 0,
            cooldown_seconds: 0,
        };
        self.is_deposited = true;
        
        Ok(())
    }

    pub fn update_cooldown(&mut self, current_time: i64) -> Result<()> {
        // Update the cooldown period based on base time and trait rarity
        let base_cooldown = self.cooldown_state.cooldown_seconds;
        
        // In a real implementation, this would be more complex with trait-based calculations
        self.cooldown_state.last_breed_time = current_time;
        
        Ok(())
    }
}