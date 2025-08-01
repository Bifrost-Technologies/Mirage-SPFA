use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum GameOutcome {
    Win,
    Loss,
    Push, // Tie
}