use anchor_lang::prelude::*;

#[account]
pub struct OffspringMint {
    pub parent_a_mint: Pubkey,
    pub parent_b_mint: Pubkey,
    pub offspring_name: String,
    pub traits: NFTTraits,
    pub is_claimed: bool,
}