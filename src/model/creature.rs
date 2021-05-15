use super::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Creature {
    pub position: Vec2<i32>,
    pub next_move: Move,
    pub creature_type: CreatureType,
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
pub enum CreatureType {
    Player,
    Dog,
}
