use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ColorEnum {
    Red,
    Black,
    Green
}