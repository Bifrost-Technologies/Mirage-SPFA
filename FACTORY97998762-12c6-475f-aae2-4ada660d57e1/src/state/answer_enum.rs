use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AnswerEnum {
    A,
    B,
    C,
    D,
}