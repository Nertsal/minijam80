use super::*;

mod entity;
mod pathfind;

pub use entity::*;

const VIEW_RADIUS: i32 = 3;

#[derive(Serialize, Deserialize, Clone)]
pub struct Level {
    #[serde(skip)]
    pub path: String,
    pub next_level: Option<String>,
    pub entities: Vec<Entity>,
}

impl Level {
    pub fn turn(&mut self, player_move: Move) {
        self.calc_moves(player_move);
        self.make_moves();
    }

    pub fn empty() -> Self {
        Self {
            path: "".to_owned(),
            next_level: None,
            entities: vec![],
        }
    }

    fn load(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
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

    fn get_entity(&self, position: Vec2<i32>) -> Option<&Entity> {
        self.entities
            .iter()
            .find(|entity| entity.position == position)
    }

    fn calc_moves(&mut self, player_move: Move) {
        let mut updates = HashMap::new();
        for (entity_index, entity) in self.entities.iter().enumerate() {
            match &entity.controller {
                Some(controller) => {
                    let direction = match controller.controller_type {
                        ControllerType::Player => player_move.direction(),
                        _ => self.get_move_direction(
                            entity,
                            VIEW_RADIUS,
                            entity.entity_type.enemies(),
                            entity.entity_type.attractors(),
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
            if let Some(mut entity) = self.entities.get(entity_index).cloned() {
                self.move_entity(&mut entity);
                self.collide_entity(&mut entity);
                *self.entities.get_mut(entity_index).unwrap() = entity;
            }
        }
    }

    fn win_state(&self) -> bool {
        if let Some(player) = self.get_player() {
            let targets = player.entity_type.attractors();
            if targets.len() > 0
                && !self
                    .entities
                    .iter()
                    .any(|entity| targets.contains(&entity.entity_type))
            {
                return true;
            }
        }
        false
    }

    fn get_player(&self) -> Option<&Entity> {
        self.entities.iter().find(|entity| {
            if let Some(EntityController {
                controller_type: ControllerType::Player,
                ..
            }) = &entity.controller
            {
                true
            } else {
                false
            }
        })
    }

    fn move_entity(&mut self, entity: &mut Entity) {
        if let Some(controller) = &entity.controller {
            let next_pos = entity.position + controller.next_move.direction();
            let can_move = match &controller.controller_type {
                ControllerType::Dog {
                    chain: Some(Chain { origin, distance }),
                } => position_distance(next_pos, *origin) <= *distance,
                _ => true,
            };
            if can_move
                && self
                    .get_entity(next_pos)
                    .map_or(true, |entity| !entity.entity_type.collidable())
            {
                entity.position = next_pos;
            }
        }
    }

    fn collide_entity(&mut self, entity: &mut Entity) {
        if let Some(other) = self.get_entity(entity.position) {
            let attractors = entity.entity_type.attractors();
            if attractors.contains(&other.entity_type) {
                self.remove_entity(entity.position);
            }
        }
    }

    fn get_move_direction(
        &self,
        entity: &Entity,
        view_radius: i32,
        avoids: Vec<EntityType>,
        attractors: Vec<EntityType>,
    ) -> Vec2<i32> {
        if let Some(direction) = self
            .entities
            .iter()
            .filter_map(|other| {
                if avoids.contains(&other.entity_type) {
                    let distance = entity.distance(other);
                    let direction = self.can_see(entity, other, view_radius);
                    if let Some(direction) = direction {
                        return Some((other.position, distance, direction));
                    }
                }
                None
            })
            .min_by_key(|&(_, distance, _)| distance)
            .map(|(_, _, direction)| direction)
        {
            let direction = -direction;
            let next_pos = entity.position + direction;
            if self.is_empty(next_pos) {
                direction
            } else {
                let direction = vec2(-direction.y, direction.x);
                let next_pos = entity.position + direction;
                if self.is_empty(next_pos) {
                    direction
                } else {
                    -direction
                }
            }
        } else if let Some(direction) = self
            .entities
            .iter()
            .filter_map(|other| {
                if attractors.contains(&other.entity_type) {
                    let distance = entity.distance(other);
                    let direction = self.can_see(entity, other, view_radius);
                    if let Some(direction) = direction {
                        return Some((other.position, distance, direction));
                    }
                }
                None
            })
            .min_by_key(|&(_, distance, _)| distance)
            .map(|(_, _, direction)| direction)
        {
            direction
        } else {
            vec2(0, 0)
        }
    }

    fn is_empty(&self, position: Vec2<i32>) -> bool {
        self.get_entity(position).is_none()
    }
}
