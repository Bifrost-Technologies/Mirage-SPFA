use crate::state::{Inventory};
use anchor_lang::prelude::*;

#[account]
pub struct PlayerState {
    pub owner: Pubkey,
    pub adventure_pubkey: Pubkey,
    pub inventory: Inventory,
    pub total_damage_dealt: u32,
}

impl PlayerState {
    pub const MAXIMUM_SIZE: usize = 5000;

    pub fn init(&mut self, owner_pubkey: Pubkey, adventure_pubkey: Pubkey) -> Result<()> {
        self.owner = owner_pubkey;
        self.adventure_pubkey = adventure_pubkey;
        self.inventory = Inventory {
            items: Vec::new(),
            total_value: 0,
        };
        self.total_damage_dealt = 0;
        
        Ok(())
    }
}