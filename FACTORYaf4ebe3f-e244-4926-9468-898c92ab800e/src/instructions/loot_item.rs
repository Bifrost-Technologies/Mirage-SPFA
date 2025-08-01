use crate::state::{Inventory};
use anchor_lang::prelude::*;

pub fn loot_item(
    ctx: Context<LootItem>,
    item_type: String,
    signature: Vec<u8>,
) -> Result<()> {
    let adventure_state = &mut ctx.accounts.adventure_state;
    
    // Validate that the adventure is active
    require_eq!(adventure_state.is_active, true, RoguelikeError::AdventureAlreadyStarted);
    
    // In a real implementation, you would verify the signature here
    
    // Add item to player's inventory (in a real implementation, this would mint an NFT)
    let player_state = &mut ctx.accounts.player_state;
    
    player_state.inventory.items.push(item_type.clone());
    
    Ok(())
}

#[derive(Accounts)]
pub struct LootItem<'info> {
    #[account(
        mut,
        seeds = ["adventure".as_bytes(), player.key().as_ref()],
        bump
    )]
    pub adventure_state: Account<'info, AdventureState>,
    
    #[account(
        mut,
        seeds = ["player".as_bytes(), player.key().as_ref(), adventure_state.key().as_ref()],
        bump
    )]
    pub player_state: Account<'info, PlayerState>,
    
    #[account(mut)]
    pub player: Signer<'info>,
}