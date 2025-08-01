use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SeriesConfig {
    pub series_name: String,
    pub total_boxes: u32,
    pub box_price: u64,
    pub rarity_weights: Vec<u8>, // Weight for each rarity tier (0-100)
}