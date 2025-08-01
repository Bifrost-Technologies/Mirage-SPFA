use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GameConfig {
    pub map_width: u32,
    pub map_height: u32,
    pub base_treasure_probability: f64,
    pub max_players: u32,
}