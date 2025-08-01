use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RoomData {
    pub room_x: i32,
    pub room_y: i32,
    pub monster_spawned: bool,
    pub items_available: Vec<String>,
}