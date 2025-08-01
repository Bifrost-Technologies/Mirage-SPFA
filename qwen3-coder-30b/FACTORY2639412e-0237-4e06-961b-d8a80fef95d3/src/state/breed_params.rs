use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BreedParams {
    pub parent_a_mint: Pubkey,
    pub parent_b_mint: Pubkey,
    pub offspring_name: String,
}