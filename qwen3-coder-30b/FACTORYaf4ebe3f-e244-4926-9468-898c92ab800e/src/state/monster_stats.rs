use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MonsterStats {
    pub name: String,
    pub health: u32,
    pub attack_power: u32,
    pub defense: u32,
}