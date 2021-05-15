use super::*;

pub struct Creature {
    pub position: Vec2<i32>,
    pub next_move: Option<Move>,
    pub creature_type: CreatureType,
}

pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

pub enum CreatureType {
    Player,
    Dog,
}