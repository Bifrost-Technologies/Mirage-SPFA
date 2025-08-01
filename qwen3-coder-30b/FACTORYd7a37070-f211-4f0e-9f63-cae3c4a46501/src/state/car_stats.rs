use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CarStats {
    pub speed: u32,
    pub acceleration: u32,
    pub handling: u32,
    pub durability: u32,
}