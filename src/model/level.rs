use super::*;

const VIEW_RADIUS: i32 = 3;

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

    pub fn get_player_mut(&mut self) -> Option<&mut Entity> {
        if let Some(i) = self
            .entities
            .iter()
            .enumerate()
            .find(|(_, creature)| {
                if let Some(EntityController {
                    controller_type: ControllerType::Player,
                    ..
                }) = creature.controller
                {
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

    pub fn turn(&mut self, player_move: Move) {
        self.calc_moves(player_move);
        self.make_moves();
    }

    fn calc_moves(&mut self, player_move: Move) {
        let mut updates = HashMap::new();
        for (entity_index, entity) in self.entities.iter().enumerate() {
            match &entity.controller {
                Some(controller) => {
                    let direction = match controller.controller_type {
                        ControllerType::Player => player_move.direction(),
                        ControllerType::Cat => self.get_move_direction(
                            entity,
                            VIEW_RADIUS,
                            vec![EntityType::Dog],
                            vec![EntityType::Mouse],
                        ),
                        ControllerType::Dog => self.get_move_direction(
                            entity,
                            VIEW_RADIUS,
                            vec![],
                            vec![EntityType::Cat],
                        ),
                        ControllerType::Mouse => self.get_move_direction(
                            entity,
                            VIEW_RADIUS,
                            vec![EntityType::Cat],
                            vec![],
                        ),
                    };
                    let next_move = Move::from_direction(direction).unwrap_or(Move::Wait);
                    updates.insert(entity_index, next_move);
                }
                None => {}
            }
        }
        for (update_index, update_move) in updates {
            let entity = self.entities.get_mut(update_index).unwrap();
            if let Some(controller) = &mut entity.controller {
                controller.next_move = update_move;
            } else {
                unreachable!()
            }
        }
    }

    fn make_moves(&mut self) {
        for entity_index in 0..self.entities.len() {
            let mut entity = self.entities.get(entity_index).unwrap().clone();
            if let Some(controller) = &entity.controller {
                let entity_move = controller.next_move;
                self.move_entity(&mut entity, entity_move);
            }
            *self.entities.get_mut(entity_index).unwrap() = entity;
        }
    }

    fn move_entity(&mut self, entity: &mut Entity, entity_move: Move) {
        let next_pos = entity.position + entity_move.direction();
        if self.get_entity(next_pos).is_none() {
            entity.position = next_pos;
        }
    }

    fn get_move_direction(
        &self,
        entity: &Entity,
        view_radius: i32,
        avoids: Vec<EntityType>,
        attractors: Vec<EntityType>,
    ) -> Vec2<i32> {
        let avoid: Vec<Vec2<i32>> = self
            .entities
            .iter()
            .filter(|other| {
                avoids.contains(&other.entity_type) && entity.distance(other) <= view_radius
            })
            .map(|entity| entity.position)
            .collect();
        if avoid.len() > 0 {
            let mut direction = vec2(0.0, 0.0);
            for avoid_pos in avoid {
                direction += (entity.position - avoid_pos).map(|x| x as f32).normalize();
            }
            direction.map(|x| x.ceil() as i32)
        } else {
            let closest = self
                .entities
                .iter()
                .filter_map(|other| {
                    let distance = entity.distance(other);
                    if attractors.contains(&other.entity_type) && distance <= view_radius {
                        Some((other.position, distance))
                    } else {
                        None
                    }
                })
                .min_by_key(|&(_, distance)| distance)
                .map(|(other_pos, _)| other_pos);
            if let Some(closest) = closest {
                let direction = (closest - entity.position).map(|x| x as f32).normalize();
                direction.map(|x| x.ceil() as i32)
            } else {
                vec2(0, 0)
            }
        }
    }
}
