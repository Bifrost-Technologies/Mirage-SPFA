use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MemberStats {
    pub level: u32,
    pub experience: u32,
    pub contribution_score: u32,
}