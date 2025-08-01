use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum HandState {
    Active,   // Player can hit or stand
    Stand,   // Player has stood
    Bust,     // Player has busted (over 21)
    BlackJack,// Player has blackjack (21 with two cards)
}