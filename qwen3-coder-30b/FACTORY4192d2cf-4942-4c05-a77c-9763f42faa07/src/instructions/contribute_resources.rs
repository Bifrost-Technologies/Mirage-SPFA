use crate::state::{MemberState};
use anchor_lang::prelude::*;

pub fn contribute_resources(ctx: Context<ContributeResources>, resource_amount: u64) -> Result<()> {
    let member_state = &mut ctx.accounts.member_state;
    
    // Validate that the contribution is positive
    require_gte!(resource_amount, 1, GuildConquestError::InsufficientResources);
    
    // Update member's resources contributed
    if let Some(amount) = member_state.resources_contributed.checked_add(resource_amount) {
        member_state.resources_contributed = amount;
    } else {
        return Err(GuildConquestError::InsufficientResources.into());
    }
    
    // Update guild's total resources contributed
    let guild_state = &mut ctx.accounts.guild_state;
    if let Some(amount) = guild_state.total_resources_contributed.checked_add(resource_amount) {
        guild_state.total_resources_contributed = amount;
    } else {
        return Err(GuildConquestError::InsufficientResources.into());
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct ContributeResources<'info> {
    #[account(
        mut,
        seeds = ["member".as_bytes(), authority.key().as_ref(), guild_state.key().as_ref()],
        bump
    )]
    pub member_state: Account<'info, MemberState>,
    
    #[account(mut)]
    pub guild_state: Account<'info, GuildState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}