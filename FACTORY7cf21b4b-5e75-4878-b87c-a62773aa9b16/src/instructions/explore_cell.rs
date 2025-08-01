use crate::state::{LandMap, MapCell};
use anchor_lang::prelude::*;

pub fn explore_cell(
    ctx: Context<ExploreCell>,
    cell_x: u32,
    cell_y: u32,
    signature: Vec<u8>,
) -> Result<()> {
    let land_map = &mut ctx.accounts.land_map;
    
    // Validate coordinates
    require!(
        cell_x < land_map.map_width && cell_y < land_map.map_height,
        TreasureHuntError::InvalidCellCoordinates
    );
    
    // Mark the cell as explored
    land_map.mark_cell_as_explored(cell_x, cell_y)?;
    
    // In a real implementation, you would verify the signature here
    
    Ok(())
}

#[derive(Accounts)]
pub struct ExploreCell<'info> {
    #[account(
        mut,
        seeds = ["land_map".as_bytes()],
        bump
    )]
    pub land_map: Account<'info, LandMap>,
    
    #[account(mut)]
    pub player_state: Account<'info, PlayerState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}