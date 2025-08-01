use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("11223344556677889900112233445566");

#[program]
pub mod onchain_roguelike {
    use super::*;

    pub fn start_adventure(ctx: Context<StartAdventure>, seed: u64) -> Result<()> {
        instructions::start_adventure::start_adventure(ctx, seed)
    }

    pub fn move_room(
        ctx: Context<MoveRoom>,
        room_x: i32,
        room_y: i32,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::move_room::move_room(ctx, room_x, room_y, signature)
    }

    pub fn fight_monster(
        ctx: Context<FightMonster>,
        monster_stats: MonsterStats,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::fight_monster::fight_monster(ctx, monster_stats, signature)
    }

    pub fn loot_item(
        ctx: Context<LootItem>,
        item_type: String,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::loot_item::loot_item(ctx, item_type, signature)
    }

    pub fn end_adventure(ctx: Context<EndAdventure>) -> Result<()> {
        instructions::end_adventure::end_adventure(ctx)
    }
}