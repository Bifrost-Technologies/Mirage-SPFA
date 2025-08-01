use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum OutcomeEnum {
    PlayerWin,
    BankerWin,
    TieGame
}