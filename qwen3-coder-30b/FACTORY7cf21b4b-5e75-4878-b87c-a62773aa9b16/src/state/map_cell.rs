use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MapCell {
    pub x: u32,
    pub y: u32,
    pub is_explored: bool,
    pub treasure_item: Option<TreasureItem>,
}