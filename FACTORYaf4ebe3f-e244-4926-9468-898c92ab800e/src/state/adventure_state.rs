use anchor_lang::prelude::*;

#[account]
pub struct AdventureState {
    pub seed: u64,
    pub is_active: bool,
    pub current_room_x: i32,
    pub current_room_y: i32,
    pub player_health: u32,
}

impl AdventureState {
    pub const MAXIMUM_SIZE: usize = 1000;

    pub fn init(&mut self, seed: u64) -> Result<()> {
        require_eq!(self.is_active, false, RoguelikeError::AdventureAlreadyStarted);
        
        self.seed = seed;
        self.is_active = true;
        self.current_room_x = 0;
        self.current_room_y = 0;
        self.player_health = 100; // Default health
        
        Ok(())
    }

    pub fn end(&mut self) -> Result<()> {
        require_eq!(self.is_active, true, RoguelikeError::AdventureAlreadyStarted);
        
        self.is_active = false;
        
        Ok(())
    }
}