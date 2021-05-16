use super::*;

mod entity;
mod id;
mod pathfind;

pub use entity::*;
use id::*;

const VIEW_RADIUS: i32 = 3;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum LevelState {
    Playing,
    Win,
    Loss,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Level {
    #[serde(skip)]
    pub path: String,
    id_generator: IdGenerator,
    pub next_level: Option<String>,
    pub entities: HashMap<Id, Entity>,
}

impl Level {
    pub fn turn(&mut self, player_move: Move) {
        println!("TURN");
        println!("{:?}", self.entities);
        for entity in self.entities.values_mut() {
            if let Some(c) = &mut entity.controller {
                c.next_move = Move::Wait;
            }
        }
        self.calc_moves(player_move);
        self.make_moves();
        self.collide();
    }

    pub fn empty() -> Self {
        Self {
            path: "".to_owned(),
            id_generator: IdGenerator::new(),
            next_level: None,
            entities: HashMap::new(),
        }
    }

    pub fn remove_entity(&mut self, position: Vec2<i32>) -> Option<Entity> {
        if let Some(i) = self
            .entities
            .iter()
            .find(|(_, entity)| entity.position == position)
            .map(|(&i, _)| i)
        {
            self.entities.remove(&i)
        } else {
            None
        }
    }

    pub fn set_entity(&mut self, entity: Entity) -> Option<Entity> {
        let old_entity = self.remove_entity(entity.position);
        self.entities.insert(self.id_generator.gen(), entity);
        old_entity
    }

    fn get_entity(&self, position: Vec2<i32>) -> Option<(&Id, &Entity)> {
        self.entities
            .iter()
            .find(|(_, entity)| entity.position == position)
    }

    fn get_entity_mut(&mut self, position: Vec2<i32>) -> Option<&mut Entity> {
        self.entities
            .values_mut()
            .find(|entity| entity.position == position)
    }

    fn calc_moves(&mut self, player_move: Move) {
        let mut updates = HashMap::new();
        let ids: Vec<Id> = self.entities.keys().copied().collect();
        for entity_id in ids {
            let entity = self.entities.get(&entity_id).unwrap();
            match &entity.controller {
                Some(controller) => {
                    let direction = match controller.controller_type {
                        ControllerType::Player => player_move.direction(),
                        _ => {
                            let enemies = entity.entity_type.enemies();
                            let attractors = entity.entity_type.attractors();
                            self.get_move_direction(entity_id, VIEW_RADIUS, enemies, attractors)
                        }
                    };
                    let next_move = Move::from_direction(direction).unwrap_or(Move::Wait);
                    updates.insert(entity_id, next_move);
                }
                None => {}
            }
        }
        // let mut ignore_ids = Vec::new();
        for (update_id, update_move) in updates {
            // if ignore_ids.contains(&update_id) {
            //     continue;
            // }

            // if update_move != Move::Wait {
            //     let entity = self.entities.get(&update_id).unwrap();
            //     let move_pos = entity.position + update_move.direction();
            //     if let Some((&move_entity_id, move_entity)) = self.get_entity(move_pos) {
            //         if let Some(controller) = &move_entity.controller {
            //             if controller.next_move.direction() == -update_move.direction() {
            //                 // Two units try to move through each other
            //                 if entity
            //                     .entity_type
            //                     .attractors()
            //                     .contains(&move_entity.entity_type)
            //                 {
            //                     ignore_ids.push(move_entity_id);
            //                 } else if move_entity
            //                     .entity_type
            //                     .attractors()
            //                     .contains(&entity.entity_type)
            //                 {
            //                     ignore_ids.push(update_id);
            //                 } else {
            //                     ignore_ids.push(update_id);
            //                     ignore_ids.push(move_entity_id);
            //                 }
            //                 continue;
            //             }
            //         }
            //     }
            // }

            let entity = self.entities.get_mut(&update_id).unwrap();
            entity.controller.as_mut().unwrap().next_move = update_move;
        }
    }

    fn move_entity(
        &mut self,
        prev_pos: Option<Vec2<i32>>,
        position: Vec2<i32>,
        override_direction: Option<Vec2<i32>>,
        state: &mut HashSet<Id>,
    ) -> bool {
        let (&entity_id, entity) = self.get_entity(position).unwrap();
        let entity_type = entity.entity_type;
        let direction = entity
            .controller
            .as_ref()
            .map(|c| c.next_move.direction())
            .or(override_direction)
            .unwrap_or(vec2(0, 0));
        if direction == vec2(0, 0) {
            return false;
        }
        if !state.insert(entity_id) {
            return false;
        }
        let next_pos = entity.position + direction;
        let moved = if let Some((_, other)) = self.get_entity(next_pos) {
            let other_entity_type = other.entity_type;
            if !entity_type.attractors().contains(&other.entity_type)
                && other.entity_type.property() == Some(EntityProperty::Pushable)
            {
                self.move_entity(Some(position), next_pos, Some(direction), state)
            } else if self.move_entity(Some(position), next_pos, None, state) {
                true
            } else if entity_type.attractors().contains(&other_entity_type) {
                self.get_entity_mut(position)
                    .unwrap()
                    .controller
                    .as_mut()
                    .unwrap()
                    .last_attractor_pos = None;
                self.remove_entity(next_pos);
                true
            } else {
                false
            }
        } else {
            true
        };
        if moved {
            self.get_entity_mut(position).unwrap().position = next_pos;
        }
        moved && Some(next_pos) != prev_pos
    }

    fn make_moves(&mut self) {
        let mut entity_ids = self.entities.iter().collect::<Vec<(&Id, &Entity)>>();
        entity_ids.sort_by_key(|(_, entity)| (-entity.position.y, entity.position.x));
        let entity_ids = entity_ids
            .into_iter()
            .map(|(&id, _)| id)
            .collect::<Vec<Id>>();
        for &entity_id in &entity_ids {
            let entity = self.entities.get(&entity_id).unwrap();
            println!("{:?}", entity);
        }
        let mut state = HashSet::new();
        for entity_id in entity_ids {
            if let Some(entity) = self.entities.get(&entity_id) {
                self.move_entity(None, entity.position, None, &mut state);
            }
        }
    }

    fn collide(&mut self) {
        let entity_ids: Vec<Id> = self.entities.keys().copied().collect();
        for entity_id in entity_ids {
            if let Some(entity) = self.entities.get(&entity_id).cloned() {
                self.collide_entity(&entity);
            }
        }
    }

    pub fn get_state(&self) -> LevelState {
        if let Some(player) = self.get_player() {
            let targets = player.entity_type.attractors();
            if targets.len() > 0
                && !self
                    .entities
                    .values()
                    .any(|entity| targets.contains(&entity.entity_type))
            {
                LevelState::Win
            } else {
                LevelState::Playing
            }
        } else {
            LevelState::Loss
        }
    }

    fn get_player(&self) -> Option<&Entity> {
        self.entities.values().find(|entity| {
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

    // fn move_entity(&mut self, entity: &mut Entity) {
    //     if let Some(controller) = &entity.controller {
    //         if controller.next_move == Move::Wait {
    //             return;
    //         }
    //         let direction = controller.next_move.direction();
    //         let position = entity.position;
    //         let next_pos = position + direction;
    //         let can_move = match &controller.controller_type {
    //             ControllerType::Dog {
    //                 chain: Some(Chain { origin, distance }),
    //             } => position_distance(next_pos, *origin) <= *distance,
    //             _ => true,
    //         };
    //         if can_move {
    //             if self.can_move(position, direction) {
    //                 entity.controller.as_mut().unwrap().next_move = Move::Wait;
    //                 entity.position = next_pos;
    //             } else if let Some(last_position) = self.can_push(position, direction) {
    //                 self.push(position, last_position, direction);
    //                 entity.controller.as_mut().unwrap().next_move = Move::Wait;
    //                 entity.position = next_pos;
    //             }
    //         }
    //     }
    // }

    fn collide_entity(&mut self, entity: &Entity) {
        let position = entity.position;
        for remove_id in self
            .entities
            .iter()
            .filter_map(|(&id, other)| {
                let attractors = entity.entity_type.attractors();
                if other.position == position && attractors.contains(&other.entity_type) {
                    Some(id)
                } else {
                    None
                }
            })
            .collect::<Vec<Id>>()
        {
            self.entities.remove(&remove_id);
        }
    }

    fn can_move(&self, position: Vec2<i32>, direction: Vec2<i32>) -> bool {
        if let Some((_, entity)) = self.get_entity(position) {
            let next_pos = position + direction;
            if let Some((_, next_entity)) = self.get_entity(next_pos) {
                if entity
                    .entity_type
                    .attractors()
                    .contains(&next_entity.entity_type)
                {
                    return true;
                } else if let Some(controller) = &next_entity.controller {
                    if controller.next_move != Move::Wait {
                        return true;
                    }
                }
            } else {
                return true;
            }
        }
        false
    }

    fn can_push(&self, position: Vec2<i32>, direction: Vec2<i32>) -> Option<Vec2<i32>> {
        let next_pos = position + direction;
        self.get_entity(next_pos)
            .map_or(Some(position), |(_, entity)| {
                match entity.entity_type.property() {
                    Some(EntityProperty::Pushable) => self.can_push(entity.position, direction),
                    _ => None,
                }
            })
    }

    fn push(&mut self, origin: Vec2<i32>, last_position: Vec2<i32>, direction: Vec2<i32>) {
        if last_position != origin {
            if let Some(entity) = self.get_entity_mut(last_position) {
                let next_pos = last_position + direction;
                entity.position = next_pos;
                let last_pos = last_position - direction;
                self.push(origin, last_pos, direction);
            }
        }
    }

    fn get_move_direction(
        &mut self,
        entity_id: Id,
        view_radius: i32,
        avoids: Vec<EntityType>,
        attractors: Vec<EntityType>,
    ) -> Vec2<i32> {
        let entity = self.entities.get(&entity_id).unwrap();
        if let Some((avoid_pos, direction)) = self
            .entities
            .values()
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
            .map(|(avoid_pos, _, direction)| (avoid_pos, direction))
        {
            let direction = -direction;
            let next_pos = entity.position + direction;
            return if self.is_empty(next_pos) {
                direction
            } else {
                let direction = vec2(-direction.y, direction.x);
                let delta = entity.position - avoid_pos;
                if Vec2::dot(direction, delta) >= 0 {
                    let next_pos = entity.position + direction;
                    if self.is_empty(next_pos) {
                        direction
                    } else {
                        -direction
                    }
                } else {
                    let direction = -direction;
                    let next_pos = entity.position + direction;
                    if self.is_empty(next_pos) {
                        direction
                    } else {
                        -direction
                    }
                }
            };
        } else if let Some((attractor_pos, direction)) = self
            .entities
            .values()
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
            .map(|(attractor_pos, _, direction)| (attractor_pos, direction))
        {
            let entity = self.entities.get_mut(&entity_id).unwrap();
            if let Some(controller) = &mut entity.controller {
                controller.last_attractor_pos = Some(attractor_pos);
            }
            return direction;
        } else if let Some(controller) = &entity.controller {
            if let Some(last_attractor_pos) = controller.last_attractor_pos {
                if let Some(direction) =
                    self.pathfind(entity.position, last_attractor_pos, VIEW_RADIUS * 2)
                {
                    if direction != vec2(0, 0) {
                        return direction;
                    }
                }
                let entity = self.entities.get_mut(&entity_id).unwrap();
                entity.controller.as_mut().unwrap().last_attractor_pos = None;
            }
        }
        vec2(0, 0)
    }

    fn is_empty(&self, position: Vec2<i32>) -> bool {
        self.get_entity(position).is_none()
    }
}
