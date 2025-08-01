use crate::state::{AdventureState};
use anchor_lang::prelude::*;

pub fn move_room(
    ctx: Context<MoveRoom>,
    room_x: i32,
    room_y: i32,
    signature: Vec<u8>,
) -> Result<()> {
    let adventure_state = &mut ctx.accounts.adventure_state;
    
    // Validate that the adventure is active
    require_eq!(adventure_state.is_active, true, RoguelikeError::AdventureAlreadyStarted);
    
    // In a real implementation, you would verify the signature here
    
    // Update current room position
    adventure_state.current_room_x = room_x;
    adventure_state.current_room_y = room_y;
    
    Ok(())
}

#[derive(Accounts)]
pub struct MoveRoom<'info> {
    #[account(
        mut,
        seeds = ["adventure".as_bytes(), player.key().as_ref()],
        bump
    )]
    pub adventure_state: Account<'info, AdventureState>,
    
    #[account(mut)]
    pub player: Signer<'info>,
}