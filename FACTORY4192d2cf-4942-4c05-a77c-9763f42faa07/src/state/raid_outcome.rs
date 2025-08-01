use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RaidOutcome {
    pub target_location: String,
    pub success_probability: f64,
    pub loot_value: u64,
}