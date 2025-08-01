use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum GrowthStage {
    Seed,
    Sprout,
    Mature,
}