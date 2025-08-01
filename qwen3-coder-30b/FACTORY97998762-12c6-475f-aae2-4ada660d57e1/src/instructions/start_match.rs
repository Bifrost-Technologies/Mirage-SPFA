use crate::state::{QuestionData, MatchState};
use anchor_lang::prelude::*;

pub fn start_match(ctx: Context<StartMatch>, question_data: QuestionData) -> Result<()> {
    let match_state = &mut ctx.accounts.match_state;
    match_state.init(question_data)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct StartMatch<'info> {
    #[account(
        init,
        payer = player_a,
        seeds = ["match".as_bytes(), player_a.key().as_ref(), player_b.key().as_ref()],
        bump,
        space = MatchState::MAXIMUM_SIZE
    )]
    pub match_state: Account<'info, MatchState>,
    
    #[account(mut)]
    pub player_a: Signer<'info>,
    
    #[account(mut)]
    pub player_b: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}