use super::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Level {
    pub entities: Vec<Entity>,
}

impl Level {
    pub fn empty() -> Self {
        Self { entities: vec![] }
    }

    pub fn load(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        Ok(serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open(path)?,
        ))?)
    }

    pub fn get_player_mut(&mut self) -> Option<&mut Entity> {
        if let Some(i) = self
            .entities
            .iter()
            .enumerate()
            .find(|(_, creature)| {
                if let Some(EntityController::Player) = creature.controller {
                    true
                } else {
                    false
                }
            })
            .map(|(i, _)| i)
        {
            self.entities.get_mut(i)
        } else {
            None
        }
    }

    pub fn make_move(&mut self, player_move: Move) {
        for entity_index in 0..self.entities.len() {
            let mut entity = self.entities[entity_index].clone();
            match entity.controller {
                Some(controller) => match controller {
                    EntityController::Player => {
                        let direction = match player_move {
                            Move::Up => vec2(0, 1),
                            Move::Down => vec2(0, -1),
                            Move::Right => vec2(1, 0),
                            Move::Left => vec2(-1, 0),
                            Move::Wait => vec2(0, 0),
                        };
                        let next_pos = entity.position + direction;
                        if self.get_entity(next_pos).is_none() {
                            entity.position = next_pos;
                        }
                    }
                    EntityController::Cat => {}
                    EntityController::Dog => {}
                    EntityController::Mouse => {}
                },
                None => {}
            }
            *self.entities.get_mut(entity_index).unwrap() = entity;
        }
    }

    pub fn remove_entity(&mut self, position: Vec2<i32>) -> Option<Entity> {
        if let Some(i) = self
            .entities
            .iter()
            .enumerate()
            .find(|(_, entity)| entity.position == position)
            .map(|(i, _)| i)
        {
            Some(self.entities.remove(i))
        } else {
            None
        }
    }

    pub fn set_entity(&mut self, entity: Entity) -> Option<Entity> {
        let old_entity = self.remove_entity(entity.position);
        self.entities.push(entity);
        old_entity
    }

    pub fn get_entity(&self, position: Vec2<i32>) -> Option<&Entity> {
        self.entities
            .iter()
            .find(|entity| entity.position == position)
    }
}
