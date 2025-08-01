use crate::state::{TableConfig, TableState};
use anchor_lang::prelude::*;

pub fn init_table(ctx: Context<InitTable>, table_config: TableConfig) -> Result<()> {
    let table_state = &mut ctx.accounts.table_state;
    table_state.init(table_config)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitTable<'info> {
    #[account(
        init,
        payer = authority,
        seeds = ["table".as_bytes(), authority.key().as_ref()],
        bump,
        space = TableState::MAXIMUM_SIZE
    )]
    pub table_state: Account<'info, TableState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}