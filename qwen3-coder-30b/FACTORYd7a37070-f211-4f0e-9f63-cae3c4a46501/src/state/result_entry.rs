use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ResultEntry {
    pub racer_pubkey: Pubkey,
    pub time_in_seconds: u32,
    pub final_position: u32,
}