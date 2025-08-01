use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum PieceEnum {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}