use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PlayerInventory {
    pub collected_items: Vec<TreasureItem>,
    pub total_value: u32,
}