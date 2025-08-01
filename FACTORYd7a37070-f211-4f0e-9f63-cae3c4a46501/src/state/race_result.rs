use crate::state::{ResultEntry};
use anchor_lang::prelude::*;

#[account]
pub struct RaceResult {
    pub race_name: String,
    pub results: Vec<ResultEntry>,
    pub is_finalized: bool,
}

impl RaceResult {
    pub const MAXIMUM_SIZE: usize = 5000;

    pub fn init(&mut self, race_name: String) -> Result<()> {
        self.race_name = race_name;
        self.results = Vec::new();
        self.is_finalized = false;
        
        Ok(())
    }

    pub fn add_result(&mut self, result_entry: ResultEntry) -> Result<()> {
        // Limit number of results to prevent account size issues
        if self.results.len() < 100 { // Arbitrary limit
            self.results.push(result_entry);
        } else {
            return Err(RacingError::InvalidTimeSubmitted.into());
        }
        
        Ok(())
    }

    pub fn finalize(&mut self) -> Result<()> {
        require_eq!(self.is_finalized, false, RacingError::RaceNotStarted);
        
        // Sort results by time (ascending)
        self.results.sort_by_key(|entry| entry.time_in_seconds);
        
        // Set final positions
        for (index, entry) in self.results.iter_mut().enumerate() {
            entry.final_position = index as u32 + 1;
        }
        
        self.is_finalized = true;
        
        Ok(())
    }
}