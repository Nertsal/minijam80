use super::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entity {
    pub position: Vec2<i32>,
    pub entity_type: EntityType,
    pub controller: Option<EntityController>,
}

impl Entity {
    pub fn distance(&self, other: &Self) -> i32 {
        position_distance(self.position, other.position)
    }
}

pub fn position_distance(pos1: Vec2<i32>, pos2: Vec2<i32>) -> i32 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, IntoEnumIterator, Debug)]
pub enum EntityType {
    Bush,
    Cat,
    Dog,
    Mouse,
    Doghouse,
    Box,
    Cheese,
    Bone,
    Fence,
    Wall,
    Water,
    Fish,
}

impl EntityType {
    pub fn enemies(&self) -> Vec<Self> {
        use EntityType::*;
        match self {
            Cat => vec![Dog],
            Dog => vec![],
            Mouse => vec![Cat],
            _ => vec![],
        }
    }
    pub fn attractors(&self) -> Vec<Self> {
        use EntityType::*;
        match self {
            Cat => vec![Mouse, Fish],
            Dog => vec![Cat, Bone],
            Mouse => vec![Cheese],
            _ => vec![],
        }
    }
    pub fn property(&self) -> Option<EntityProperty> {
        use EntityType::*;
        match self {
            Bush | Doghouse | Fence | Wall | Water => Some(EntityProperty::Collidable),
            Box | Cheese | Bone => Some(EntityProperty::Pushable),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum EntityProperty {
    Collidable,
    Pushable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntityController {
    pub next_move: Move,
    pub last_attractor_pos: Option<Vec2<i32>>,
    pub controller_type: ControllerType,
}

impl EntityController {
    pub fn from_entity_type(entity_type: EntityType) -> Option<Self> {
        use EntityType::*;
        match entity_type {
            Cat => Some(ControllerType::Cat),
            Dog => Some(ControllerType::Dog { chain: None }),
            Mouse => Some(ControllerType::Mouse),
            _ => None,
        }
        .map(|controller_type| Self {
            next_move: Move::Wait,
            last_attractor_pos: None,
            controller_type,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ControllerType {
    Player,
    Cat,
    Dog { chain: Option<Chain> },
    Mouse,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chain {
    pub origin: Vec2<i32>,
    pub distance: i32,
}
