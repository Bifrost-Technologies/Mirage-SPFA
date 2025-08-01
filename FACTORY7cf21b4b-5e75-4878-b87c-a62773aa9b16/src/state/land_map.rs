use crate::state::{MapCell};
use anchor_lang::prelude::*;

#[account]
pub struct LandMap {
    pub cells: Vec<MapCell>,
    pub map_width: u32,
    pub map_height: u32,
}

impl LandMap {
    pub const MAXIMUM_SIZE: usize = 10000;

    pub fn init(&mut self, width: u32, height: u32) -> Result<()> {
        self.map_width = width;
        self.map_height = height;
        
        // Initialize cells
        let mut cells = Vec::new();
        
        for x in 0..width {
            for y in 0..height {
                cells.push(MapCell {
                    x,
                    y,
                    is_explored: false,
                    treasure_item: None, // Will be populated during map generation
                });
            }
        }
        
        self.cells = cells;
        
        Ok(())
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<&MapCell> {
        if x >= self.map_width || y >= self.map_height {
            return None;
        }
        
        let index = (y * self.map_width + x) as usize;
        if index < self.cells.len() {
            Some(&self.cells[index])
        } else {
            None
        }
    }

    pub fn get_cell_mut(&mut self, x: u32, y: u32) -> Option<&mut MapCell> {
        if x >= self.map_width || y >= self.map_height {
            return None;
        }
        
        let index = (y * self.map_width + x) as usize;
        if index < self.cells.len() {
            Some(&mut self.cells[index])
        } else {
            None
        }
    }

    pub fn set_cell_treasure(&mut self, x: u32, y: u32, treasure_item: TreasureItem) -> Result<()> {
        let cell = self.get_cell_mut(x, y).unwrap();
        
        if cell.is_explored && cell.treasure_item.is_none() {
            cell.treasure_item = Some(treasure_item);
            Ok(())
        } else {
            Err(TreasureHuntError::NoTreasureAvailable.into())
        }
    }

    pub fn mark_cell_as_explored(&mut self, x: u32, y: u32) -> Result<()> {
        let cell = self.get_cell_mut(x, y).unwrap();
        
        if !cell.is_explored {
            cell.is_explored = true;
            Ok(())
        } else {
            Err(TreasureHuntError::CellNotExplored.into())
        }
    }
}