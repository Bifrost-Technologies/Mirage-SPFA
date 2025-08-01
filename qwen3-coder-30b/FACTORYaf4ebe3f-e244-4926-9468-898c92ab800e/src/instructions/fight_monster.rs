use crate::state::{MonsterStats};
use anchor_lang::prelude::*;

pub fn fight_monster(
    ctx: Context<FightMonster>,
    monster_stats: MonsterStats,
    signature: Vec<u8>,
) -> Result<()> {
    let adventure_state = &mut ctx.accounts.adventure_state;
    
    // Validate that the adventure is active
    require_eq!(adventure_state.is_active, true, RoguelikeError::AdventureAlreadyStarted);
    
    // In a real implementation, you would verify the signature here
    
    // Simulate combat - reduce player health based on monster stats
    let damage_dealt = monster_stats.attack_power;
    
    if let Some(health) = adventure_state.player_health.checked_sub(damage_dealt) {
        adventure_state.player_health = health;
    } else {
        adventure_state.player_health = 0; // Player dies
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct FightMonster<'info> {
    #[account(
        mut,
        seeds = ["adventure".as_bytes(), player.key().as_ref()],
        bump
    )]
    pub adventure_state: Account<'info, AdventureState>,
    
    #[account(mut)]
    pub player: Signer<'info>,
}