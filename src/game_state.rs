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
    win_timer: f64,
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
            win_timer: 1.0,
        }
    }
}

impl geng::State for GameState {
    fn update(&mut self, delta_time: f64) {
        self.camera.update(delta_time as f32);
        if self.level.get_state() == LevelState::Win && self.transition.is_none() {
            self.win_timer -= delta_time;
            if self.win_timer < 0.0 {
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
                }
            }
        }
        for entity in self.level.entities.values_mut() {
            entity.render_pos += (entity.position.map(|x| x as f32) - entity.render_pos)
                .clamp(delta_time as f32 * 10.0);
        }
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        if self.win_timer > 0.0 {
            self.camera.optimize(&self.level);
            self.level_renderer
                .draw(&self.level, &self.camera, framebuffer);
            let text = match self.level.get_state() {
                LevelState::Playing => self
                    .level
                    .name
                    .as_ref()
                    .map(|name| name.as_str())
                    .unwrap_or("custom level"),
                LevelState::Loss => "f",
                LevelState::Win => "pog",
            };
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
            self.level_renderer.renderer.draw_text(
                framebuffer,
                &Camera::new(10.0),
                "r restarts",
                vec2(0.0, -4.9),
                0.5,
                1.0,
                &self.assets.font,
                Color::BLACK,
            );
        } else {
            ugli::clear(framebuffer, Some(Color::BLACK), None);
            self.level_renderer.renderer.draw_text(
                framebuffer,
                &Camera::new(3.0),
                "gg",
                vec2(0.0, -1.0),
                0.5,
                2.0,
                &self.assets.font,
                Color::WHITE,
            );
        }
    }
    fn handle_event(&mut self, event: geng::Event) {
        let mut player_move = None;
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::Right => {
                    player_move = Some(Move::Right);
                }
                geng::Key::Left => {
                    player_move = Some(Move::Left);
                }
                geng::Key::Up => {
                    player_move = Some(Move::Up);
                }
                geng::Key::Down => {
                    player_move = Some(Move::Down);
                }
                geng::Key::Space => {
                    player_move = Some(Move::Wait);
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
        if let Some(player_move) = player_move {
            if self.level.get_state() == LevelState::Playing {
                self.level.turn(player_move);
            }
        }
    }
    fn transition(&mut self) -> Option<geng::Transition> {
        self.transition.take()
    }
}
