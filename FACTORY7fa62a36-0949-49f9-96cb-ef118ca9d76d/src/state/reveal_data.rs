use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RevealData {
    pub box_id: u32,
    pub revealed_rarity: RarityTier,
}