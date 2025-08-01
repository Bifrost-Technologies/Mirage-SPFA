use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct NFTTraits {
    pub rarity: u8,
    pub traits: Vec<String>,
    pub stats: Vec<u32>,
}