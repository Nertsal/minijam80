use super::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity {
    pub position: Vec2<i32>,
    pub entity_type: EntityType,
    pub controller: Option<EntityController>,
}

impl Entity {
    pub fn distance(&self, other: &Self) -> i32 {
        (self.position.x - other.position.x).abs() + (self.position.y - other.position.y).abs()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Move {
    Wait,
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn direction(self) -> Vec2<i32> {
        match self {
            Move::Wait => vec2(0, 0),
            Move::Up => vec2(0, 1),
            Move::Down => vec2(0, -1),
            Move::Right => vec2(1, 0),
            Move::Left => vec2(-1, 0),
        }
    }
    pub fn from_direction(direction: Vec2<i32>) -> Option<Self> {
        match direction {
            Vec2 { x: 0, y: 0 } => Some(Move::Wait),
            Vec2 { x: 0, y: 1 } => Some(Move::Up),
            Vec2 { x: 0, y: -1 } => Some(Move::Down),
            Vec2 { x: 1, y: 0 } => Some(Move::Right),
            Vec2 { x: -1, y: 0 } => Some(Move::Left),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Bush,
    Cat,
    Dog,
    Mouse,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EntityController {
    pub next_move: Move,
    pub controller_type: ControllerType,
}

impl EntityController {
    pub fn from_entity_type(entity_type: EntityType) -> Option<Self> {
        match entity_type {
            EntityType::Bush => None,
            EntityType::Cat => Some(ControllerType::Cat),
            EntityType::Dog => Some(ControllerType::Dog),
            EntityType::Mouse => Some(ControllerType::Mouse),
        }
        .map(|controller_type| Self {
            next_move: Move::Wait,
            controller_type,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ControllerType {
    Player,
    Cat,
    Dog,
    Mouse,
}
