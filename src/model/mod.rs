use super::*;

mod creature;
mod level;
mod tile;

pub use creature::*;
use level::*;
pub use tile::*;

pub struct Model {
    pub level: Option<Level>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            level: Some(Level::test()),
        }
    }

    pub fn update(&mut self, delta_time: f32) {}

    pub fn handle_event(&mut self, event: &geng::Event) {
        if let Some(level) = &mut self.level {
            match event {
                geng::Event::KeyDown { key } => {
                    if let Some(player_move) = match key {
                        geng::Key::Right => Some(Move::Right),
                        geng::Key::Left => Some(Move::Left),
                        geng::Key::Up => Some(Move::Up),
                        geng::Key::Down => Some(Move::Down),
                        _ => None,
                    } {
                        let player = level.get_player_mut().unwrap();
                        player.next_move = Some(player_move);
                        level.make_move();
                    }
                }
                _ => (),
            }
        }
    }
}
