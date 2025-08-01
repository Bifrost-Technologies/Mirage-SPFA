use crate::state::{BoxData, UserBoxAccount};
use anchor_lang::prelude::*;

pub fn mint_blind_box(
    ctx: Context<MintBlindBox>,
    box_data: BoxData,
    signature: Vec<u8>,
) -> Result<()> {
    let user_box_account = &mut ctx.accounts.user_box_account;
    
    // Validate that the box data is valid
    require!(
        box_data.box_id > 0 && 
        box_data.box_id <= ctx.accounts.series_config.config.total_boxes,
        BlindBoxError::InvalidBoxData
    );
    
    // In a real implementation, you would verify the signature here
    
    // Initialize user's blind box account
    user_box_account.init(
        ctx.accounts.owner.key(),
        ctx.accounts.series_config.key(),
        box_data.box_id
    )?;
    
    // Increment total boxes minted in series config
    let series_config = &mut ctx.accounts.series_config;
    if let Some(count) = series_config.total_boxes_minted.checked_add(1) {
        series_config.total_boxes_minted = count;
    } else {
        return Err(BlindBoxError::InvalidBoxData.into());
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct MintBlindBox<'info> {
    #[account(
        init,
        payer = owner,
        seeds = ["box".as_bytes(), owner.key().as_ref(), series_config.key().as_ref()],
        bump,
        space = UserBoxAccount::MAXIMUM_SIZE
    )]
    pub user_box_account: Account<'info, UserBoxAccount>,
    
    #[account(mut)]
    pub series_config: Account<'info, SeriesConfigAccount>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}