use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum BetType {
    Player,
    Banker,
    Tie
}