use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BoxData {
    pub box_id: u32,
    pub rarity_tier: RarityTier,
}