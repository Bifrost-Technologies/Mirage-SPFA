use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MarketPrice {
    pub base_price: u64,
    pub volatility_factor: f64,
}