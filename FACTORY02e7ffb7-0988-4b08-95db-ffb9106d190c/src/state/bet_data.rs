use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BetData {
    pub participant_index: u8,
    pub amount: u64,
}