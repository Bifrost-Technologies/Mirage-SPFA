use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11223344556677889900112233445566");

#[program]
pub mod onchain_trivia_battle {
    use super::*;

    pub fn start_match(ctx: Context<StartMatch>, question_data: QuestionData) -> Result<()> {
        instructions::start_match::start_match(ctx, question_data)
    }

    pub fn submit_answer(
        ctx: Context<SubmitAnswer>,
        answer_hash: [u8; 32],
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::submit_answer::submit_answer(ctx, answer_hash, signature)
    }

    pub fn reveal_answers(ctx: Context<RevealAnswers>, answer: AnswerEnum) -> Result<()> {
        instructions::reveal_answers::reveal_answers(ctx, answer)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        instructions::claim_reward::claim_reward(ctx)
    }
}