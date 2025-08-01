use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CooldownState {
    pub last_breed_time: i64,
    pub cooldown_seconds: u64,
}