use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SpinResult {
    pub number: u8,
    pub color: ColorEnum,
}