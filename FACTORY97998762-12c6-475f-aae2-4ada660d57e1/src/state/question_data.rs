use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct QuestionData {
    pub question_text: String,
    pub options: Vec<String>,
    pub correct_answer_index: u8,
    pub reveal_time: i64,
}