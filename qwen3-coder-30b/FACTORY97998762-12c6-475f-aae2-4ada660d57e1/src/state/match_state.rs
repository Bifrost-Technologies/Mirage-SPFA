use crate::state::{QuestionData};
use anchor_lang::prelude::*;

#[account]
pub struct MatchState {
    pub question_data: QuestionData,
    pub is_revealed: bool,
    pub reveal_time: i64,
    pub answers_submitted_count: u32,
}

impl MatchState {
    pub const MAXIMUM_SIZE: usize = 5000;

    pub fn init(&mut self, question_data: QuestionData) -> Result<()> {
        require_eq!(self.question_data.question_text, String::new(), TriviaError::MatchAlreadyStarted);
        
        self.question_data = question_data;
        self.is_revealed = false;
        self.reveal_time = 0;
        self.answers_submitted_count = 0;
        
        Ok(())
    }

    pub fn reveal(&mut self) -> Result<()> {
        require_eq!(self.is_revealed, false, TriviaError::QuestionNotRevealed);
        
        let current_time = Clock::get()?.unix_timestamp;
        
        // Set the reveal time to be in 10 seconds from now
        self.reveal_time = current_time + 10; // 10 second delay for reveal
        
        self.is_revealed = true;
        
        Ok(())
    }
}