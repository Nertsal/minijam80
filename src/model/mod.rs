use super::*;

mod entity;
mod level;

pub use entity::*;
pub use level::*;

pub struct Model {
    pub level: Option<Level>,
}

impl Model {
    pub fn new() -> Self {
        let mut model = Self { level: None };
        model.load_level("levels/custom/custom_level.json");
        model
    }

    pub fn update(&mut self, delta_time: f32) {}

    pub fn make_move(&mut self, player_move: Move) {
        if let Some(level) = &mut self.level {
            level.turn(player_move);
        }
    }

    pub fn set_entity(&mut self, entity: Entity) {
        if let Some(level) = &mut self.level {
            level.set_entity(entity);
        }
    }

    pub fn remove_entity(&mut self, position: Vec2<i32>) {
        if let Some(level) = &mut self.level {
            level.remove_entity(position);
        }
    }

    pub fn save_level(&self) {
    }

    pub fn load_level(&mut self, level_path: impl AsRef<std::path::Path>) {
        match Level::load(&level_path) {
            Ok(mut level) => {
                level.path = level_path.as_ref().to_str().unwrap().to_owned();
                self.level = Some(level);
            }
            Err(err) => {
                println!("Error loading level: {:?}", err);
            }
        };
    }

    pub fn next_level(&mut self) {
        if let Some(level) = &self.level {
            if let Some(level_name) = level.next_level.clone() {
                self.load_level(level_name);
            }
        }
    }

    pub fn reset_level(&mut self) {
        if let Some(level) = &self.level {
            let path = level.path.clone();
            self.load_level(path);
        }
    }
}
