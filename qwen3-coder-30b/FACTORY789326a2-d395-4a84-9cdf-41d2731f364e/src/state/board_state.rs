use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BoardState {
    pub pieces: Vec<Option<PieceEnum>>,
}