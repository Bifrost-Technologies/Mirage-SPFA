use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TreasureItem {
    pub item_type: String,
    pub rarity: u8, // 0-100 scale
    pub value: u32,
}