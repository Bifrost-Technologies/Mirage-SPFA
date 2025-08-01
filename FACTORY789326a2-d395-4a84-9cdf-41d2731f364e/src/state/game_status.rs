use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum GameStatus {
    Active,
    Checkmate,
    Stalemate,
    DrawByAgreement,
}