use super::*;

pub struct GameState {
    geng: Rc<Geng>,
    assets: Rc<Assets>,
    next_level: Option<usize>,
    camera: Camera,
    initial_level: Level,
    level: Level,
    level_renderer: LevelRenderer,
    transition: Option<geng::Transition>,
}

impl GameState {
    pub fn new(
        geng: &Rc<Geng>,
        assets: &Rc<Assets>,
        level: Level,
        next_level: Option<usize>,
    ) -> Self {
        let initial_level = level.clone();
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: Camera::new(10.0),
            initial_level,
            level,
            level_renderer: LevelRenderer::new(geng, assets),
            transition: None,
            next_level,
        }
    }
}

impl geng::State for GameState {
    fn update(&mut self, delta_time: f64) {
        self.camera.update(delta_time as f32);
        if self.level.get_state() == LevelState::Win && self.transition.is_none() {
            let next_level = self.next_level.and_then(|next| {
                if next < self.assets.levels.len() {
                    Some(next)
                } else {
                    None
                }
            });
            if let Some(next) = next_level {
                self.transition = Some(geng::Transition::Switch(Box::new(GameState::new(
                    &self.geng,
                    &self.assets,
                    self.assets.levels[next].clone(),
                    Some(next + 1),
                ))));
            } else {
                self.transition = Some(geng::Transition::Pop);
            }
        }
        for entity in self.level.entities.values_mut() {
            entity.render_pos += (entity.position.map(|x| x as f32) - entity.render_pos)
                .clamp(delta_time as f32 * 10.0);
        }
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.camera.optimize(&self.level);
        self.level_renderer
            .draw(&self.level, &self.camera, framebuffer);
        let text = self
            .level
            .name
            .as_ref()
            .map(|name| name.as_str())
            .unwrap_or("custom level");
        self.level_renderer.renderer.draw_text(
            framebuffer,
            &Camera::new(10.0),
            text,
            vec2(0.0, 4.0),
            0.5,
            1.0,
            &self.assets.font,
            Color::BLACK,
        );
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
                        self.next_level,
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
