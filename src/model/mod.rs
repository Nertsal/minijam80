use super::*;

mod creature;
mod level;
mod tile;

pub use creature::*;
use level::*;
pub use tile::*;

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
            level: Some(Level::test()),
        }
    }

    pub fn update(&mut self, delta_time: f32) {}

    pub fn make_move(&mut self, player_move: Move) {
        if let Some(level) = &mut self.level {
            let player = level.get_player_mut().unwrap();
            player.next_move = player_move;
            level.make_move()
        }
    }
}
