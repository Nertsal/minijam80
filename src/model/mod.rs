use super::*;

pub struct Model {}

impl Model {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, delta_time: f32) {}
    pub fn handle_event(&mut self, event: &geng::Event) {}
}
