use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MatchData {
    pub game_type: String, // e.g., "chess", "tic-tac-toe"
    pub board_state: Vec<u8>, // Serialized board state
    pub turn_count: u32,
}