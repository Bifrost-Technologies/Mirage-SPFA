use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FarmConfig {
    pub max_nfts: u32,
    pub base_breed_cooldown_seconds: u64,
    pub trait_rarity_weights: Vec<u8>, // Weight for each trait rarity (0-100)
}