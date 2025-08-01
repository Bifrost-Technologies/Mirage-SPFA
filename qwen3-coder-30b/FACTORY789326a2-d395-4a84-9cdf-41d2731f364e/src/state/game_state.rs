use crate::state::{GameConfig, GameStatus};
use anchor_lang::prelude::*;

#[account]
pub struct GameState {
    pub config: GameConfig,
    pub board_state: BoardState,
    pub current_turn_player: Pubkey, // The pubkey of the player whose turn it is
    pub game_status: GameStatus,
    pub move_count: u32,
}

impl GameState {
    pub const MAXIMUM_SIZE: usize = 5000;

    pub fn init(&mut self, config: GameConfig) -> Result<()> {
        require_eq!(self.config.player_a, Pubkey::default(), ChessError::AlreadyInitialized);
        
        self.config = config;
        self.current_turn_player = config.player_a; // Player A starts first
        self.game_status = GameStatus::Active;
        self.move_count = 0;
        
        // Initialize board state with standard chess setup
        let mut pieces: Vec<Option<PieceEnum>> = vec![None; 64];
        
        // Set up pawns
        for i in 8..16 {
            pieces[i] = Some(PieceEnum::Pawn);
            pieces[56 + i - 8] = Some(PieceEnum::Pawn);
        }
        
        // Set up other pieces
        pieces[0] = Some(PieceEnum::Rook);
        pieces[1] = Some(PieceEnum::Knight);
        pieces[2] = Some(PieceEnum::Bishop);
        pieces[3] = Some(PieceEnum::Queen);
        pieces[4] = Some(PieceEnum::King);
        pieces[5] = Some(PieceEnum::Bishop);
        pieces[6] = Some(PieceEnum::Knight);
        pieces[7] = Some(PieceEnum::Rook);
        
        pieces[56] = Some(PieceEnum::Rook);
        pieces[57] = Some(PieceEnum::Knight);
        pieces[58] = Some(PieceEnum::Bishop);
        pieces[59] = Some(PieceEnum::Queen);
        pieces[60] = Some(PieceEnum::King);
        pieces[61] = Some(PieceEnum::Bishop);
        pieces[62] = Some(PieceEnum::Knight);
        pieces[63] = Some(PieceEnum::Rook);
        
        self.board_state.pieces = pieces;
        
        Ok(())
    }

    pub fn switch_turn(&mut self) -> Result<()> {
        if self.current_turn_player == self.config.player_a {
            self.current_turn_player = self.config.player_b;
        } else {
            self.current_turn_player = self.config.player_a;
        }
        
        // Increment move count
        if let Some(count) = self.move_count.checked_add(1) {
            self.move_count = count;
        } else {
            return Err(ChessError::InvalidMove.into());
        }

        Ok(())
    }
}