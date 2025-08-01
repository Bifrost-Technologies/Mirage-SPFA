use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum BetTypeEnum {
    Red,
    Black,
    Even,
    Odd,
    Low,      // 1-18
    High,     // 19-36
    Dozen1,   // 1-12
    Dozen2,   // 13-24
    Dozen3,   // 25-36
    Column1,
    Column2,
    Column3,
}