use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum OutcomeEnum {
    Pending,
    Completed,
}