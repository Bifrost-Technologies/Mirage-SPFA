use crate::state::{AnswerEnum, MatchState};
use anchor_lang::prelude::*;

pub fn reveal_answers(ctx: Context<RevealAnswers>, answer: AnswerEnum) -> Result<()> {
    let match_state = &mut ctx.accounts.match_state;
    
    // Validate that the question is revealed
    require_eq!(match_state.is_revealed, true, TriviaError::QuestionNotRevealed);
    
    // In a real implementation:
    // 1. Verify the answer matches the hash submitted by players
    // 2. Calculate scores based on correct/incorrect answers
    
    Ok(())
}

#[derive(Accounts)]
pub struct RevealAnswers<'info> {
    #[account(
        mut,
        seeds = ["match".as_bytes(), player_a.key().as_ref(), player_b.key().as_ref()],
        bump
    )]
    pub match_state: Account<'info, MatchState>,
    
    #[account(mut)]
    pub player_a: Signer<'info>,
    
    #[account(mut)]
    pub player_b: Signer<'info>,
}