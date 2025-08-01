use anchor_lang::prelude::*;

#[account]
pub struct MoveHistory {
    pub moves: Vec<Vec<u8>>,
}

impl MoveHistory {
    pub const MAXIMUM_SIZE: usize = 5000;

    pub fn init(&mut self) -> Result<()> {
        self.moves = Vec::new();
        
        Ok(())
    }

    pub fn add_move(&mut self, move_data: Vec<u8>) -> Result<()> {
        // Limit number of moves to prevent account size issues
        if self.moves.len() < 1000 { // Arbitrary limit
            self.moves.push(move_data);
        } else {
            return Err(ChessError::InvalidMove.into());
        }
        
        Ok(())
    }
}