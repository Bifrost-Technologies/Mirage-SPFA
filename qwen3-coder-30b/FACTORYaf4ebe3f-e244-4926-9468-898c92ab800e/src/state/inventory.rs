use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Inventory {
    pub items: Vec<String>,
    pub total_value: u32,
}