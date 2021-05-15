use super::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity {
    pub position: Vec2<i32>,
    pub movement_type: MovementType,
    pub entity_type: EntityType,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MovementType {
    Static,
    Creature { next_move: Move },
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Move {
    Wait,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum EntityType {
    Bush,
    Player,
    Dog,
}
