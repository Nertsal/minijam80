use super::*;

mod entity;
mod level;

pub use entity::*;
use level::*;

#[derive(Clone, Copy)]
pub enum Mode {
    Play,
    Edit,
}

pub struct Model {
    pub mode: Mode,
    pub level: Option<Level>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            mode: Mode::Play,
            level: Some(Level::load("levels/custom/custom_level.json").unwrap()),
        }
    }

    pub fn update(&mut self, delta_time: f32) {}

    pub fn make_move(&mut self, player_move: Move) {
        if let Some(level) = &mut self.level {
            let player = level.get_player_mut().unwrap();
            player.movement_type = MovementType::Creature {
                next_move: player_move,
            };
            level.make_move()
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
        batbox::save_file(
            "Save custom level",
            "levels/custom/custom_level.json",
            |writer| {
                serde_json::to_writer(writer, self.level.as_ref().unwrap())?;
                Ok(())
            },
        )
        .unwrap();
    }
}
