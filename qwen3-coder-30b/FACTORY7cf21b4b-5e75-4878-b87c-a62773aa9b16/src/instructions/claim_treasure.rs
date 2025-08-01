use crate::state::{LandMap, MapCell};
use anchor_lang::prelude::*;

pub fn claim_treasure(
    ctx: Context<ClaimTreasure>,
    item_type: String,
    signature: Vec<u8>,
) -> Result<()> {
    let land_map = &mut ctx.accounts.land_map;
    
    // In a real implementation, you would verify the signature here
    
    // Find the treasure in the player's last explored cell
    let player_state = &mut ctx.accounts.player_state;
    
    // Get current cell coordinates from player state
    let x = player_state.last_explored_cell_x;
    let y = player_state.last_explored_cell_y;
    
    // Validate coordinates
    require!(
        x < land_map.map_width && y < land_map.map_height,
        TreasureHuntError::InvalidCellCoordinates
    );
    
    // Get the cell and check if it has treasure
    let cell = land_map.get_cell(x, y).unwrap();
    
    require!(cell.is_explored, TreasureHuntError::CellNotExplored);
    
    // Check if there's a treasure in this cell
    require!(
        cell.treasure_item.is_some(),
        TreasureHuntError::NoTreasureAvailable
    );
    
    // Get the treasure item
    let treasure = cell.treasure_item.clone().unwrap();
    
    // Add to player inventory (in a real implementation, you would mint an NFT)
    player_state.inventory.collected_items.push(treasure.clone());
    
    // Update total value
    player_state.inventory.total_value += treasure.value;
    
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimTreasure<'info> {
    #[account(
        mut,
        seeds = ["land_map".as_bytes()],
        bump
    )]
    pub land_map: Account<'info, LandMap>,
    
    #[account(
        mut,
        seeds = ["player".as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub player_state: Account<'info, PlayerState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}