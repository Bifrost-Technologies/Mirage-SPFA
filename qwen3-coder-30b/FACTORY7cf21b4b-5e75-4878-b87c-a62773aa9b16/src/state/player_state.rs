use crate::state::{PlayerInventory};
use anchor_lang::prelude::*;

#[account]
pub struct PlayerState {
    pub owner: Pubkey,
    pub inventory: PlayerInventory,
    pub last_explored_cell_x: u32,
    pub last_explored_cell_y: u32,
}

impl PlayerState {
    pub const MAXIMUM_SIZE: usize = 5000;

    pub fn init(&mut self, owner_pubkey: Pubkey) -> Result<()> {
        require_eq!(self.owner, Pubkey::default(), TreasureHuntError::PlayerAlreadyExists);
        
        self.owner = owner_pubkey;
        self.inventory = PlayerInventory {
            collected_items: Vec::new(),
            total_value: 0,
        };
        self.last_explored_cell_x = 0;
        self.last_explored_cell_y = 0;
        
        Ok(())
    }
}