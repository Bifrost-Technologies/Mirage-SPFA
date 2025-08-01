use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Scoreboard {
    pub player_scores: Vec<(Pubkey, u32)>,
}