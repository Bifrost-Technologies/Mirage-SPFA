use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GuildConfig {
    pub guild_name: String,
    pub max_members: u32,
    pub required_resource_contribution: u64,
    pub loot_distribution_algorithm: String,
}