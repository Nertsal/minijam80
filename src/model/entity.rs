use super::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity {
    pub position: Vec2<i32>,
    pub entity_type: EntityType,
    pub controller: Option<EntityController>,
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
    Cat,
    Dog,
    Mouse,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum EntityController {
    Player,
    Cat,
    Dog,
    Mouse,
}

impl EntityController {
    pub fn from_entity_type(entity_type: EntityType) -> Option<Self> {
        match entity_type {
            EntityType::Bush => None,
            EntityType::Cat => Some(EntityController::Cat),
            EntityType::Dog => Some(EntityController::Dog),
            EntityType::Mouse => Some(EntityController::Mouse),
        }
    }
}
