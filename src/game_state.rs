use super::*;

pub struct GameState {
    geng: Rc<Geng>,
    assets: Rc<Assets>,
    camera: Camera,
    initial_level: Level,
    level: Level,
    level_renderer: LevelRenderer,
    transition: Option<geng::Transition>,
}

impl GameState {
    pub fn new(geng: &Rc<Geng>, assets: &Rc<Assets>, level: Level) -> Self {
        let initial_level = level.clone();
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: Camera::new(10.0),
            initial_level,
            level,
            level_renderer: LevelRenderer::new(geng, assets),
            transition: None,
        }
    }
}

impl geng::State for GameState {
    fn update(&mut self, delta_time: f64) {
        self.camera.update(delta_time as f32);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.level_renderer
            .draw(&self.level, &self.camera, framebuffer);
    }
    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::Right => {
                    self.level.turn(Move::Right);
                }
                geng::Key::Left => {
                    self.level.turn(Move::Left);
                }
                geng::Key::Up => {
                    self.level.turn(Move::Up);
                }
                geng::Key::Down => {
                    self.level.turn(Move::Down);
                }
                geng::Key::Space => {
                    self.level.turn(Move::Wait);
                }
                geng::Key::R => {
                    self.transition = Some(geng::Transition::Switch(Box::new(GameState::new(
                        &self.geng,
                        &self.assets,
                        self.initial_level.clone(),
                    ))));
                }
                geng::Key::Escape => {
                    self.transition = Some(geng::Transition::Pop);
                }
                _ => (),
            },
            _ => {}
        }
    }
    fn transition(&mut self) -> Option<geng::Transition> {
        self.transition.take()
    }
}
