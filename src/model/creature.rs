use super::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Creature {
    pub position: Vec2<i32>,
    pub next_move: Option<Move>,
    pub creature_type: CreatureType,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Move {
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
