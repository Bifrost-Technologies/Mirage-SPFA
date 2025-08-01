use crate::state::{PlayerState};
use anchor_lang::prelude::*;

pub fn submit_answer(
    ctx: Context<SubmitAnswer>,
    answer_hash: [u8; 32],
    signature: Vec<u8>,
) -> Result<()> {
    let player_state = &mut ctx.accounts.player_state;
    
    // In a real implementation, you would verify the signature here
    
    // Store the hash of the answer
    player_state.answer_hash = Some(answer_hash);
    
    // Increment answers submitted count in match state
    let match_state = &mut ctx.accounts.match_state;
    
    if let Some(count) = match_state.answers_submitted_count.checked_add(1) {
        match_state.answers_submitted_count = count;
    } else {
        return Err(TriviaError::InvalidAnswer.into());
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct SubmitAnswer<'info> {
    #[account(
        init,
        payer = player,
        seeds = ["player".as_bytes(), player.key().as_ref(), match_state.key().as_ref()],
        bump,
        space = PlayerState::MAXIMUM_SIZE
    )]
    pub player_state: Account<'info, PlayerState>,
    
    #[account(
        mut,
        seeds = ["match".as_bytes(), player_a.key().as_ref(), player_b.key().as_ref()],
        bump
    )]
    pub match_state: Account<'info, MatchState>,
    
    #[account(mut)]
    pub player: Signer<'info>,
    
    #[account(mut)]
    pub player_a: Signer<'info>,
    
    #[account(mut)]
    pub player_b: Signer<'info>,
    
}