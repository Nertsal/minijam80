use geng::State;

use super::*;

pub struct Editor {
    geng: Rc<Geng>,
    assets: Rc<Assets>,
    camera: Camera,
    transition: Option<geng::Transition>,
    selected_entity: Option<EntityType>,
    level: Level,
    level_renderer: LevelRenderer,
    framebuffer_size: Vec2<usize>,
}

impl Editor {
    pub fn new(geng: &Rc<Geng>, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: Camera::new(10.0),
            transition: None,
            selected_entity: None,
            level: Level::empty(),
            level_renderer: LevelRenderer::new(geng, assets),
            framebuffer_size: vec2(1, 1),
        }
    }
    fn spawn_selected(&mut self, mouse_position: Vec2<f64>, spawn_player: bool) {
        let tile_pos = tile_pos(self.camera.screen_to_world(
            self.framebuffer_size.map(|x| x as f32),
            mouse_position.map(|x| x as f32),
        ));
        match self.selected_entity.clone() {
            Some(selected_entity) => {
                self.level.set_entity(Entity {
                    position: tile_pos,
                    entity_type: selected_entity,
                    controller: if spawn_player {
                        Some(EntityController {
                            next_move: Move::Wait,
                            controller_type: ControllerType::Player,
                        })
                    } else {
                        EntityController::from_entity_type(selected_entity)
                    },
                });
            }
            None => {
                self.level.remove_entity(tile_pos);
            }
        }
    }
}

impl geng::State for Editor {
    fn update(&mut self, delta_time: f64) {}
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        self.level_renderer
            .draw(&self.level, &self.camera, framebuffer);
    }
    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown {
                position,
                button: geng::MouseButton::Left,
            } => self.spawn_selected(position, false),
            geng::Event::MouseDown {
                position,
                button: geng::MouseButton::Right,
            } => self.spawn_selected(position, true),
            geng::Event::KeyDown { key } => match key {
                geng::Key::Num1 => self.selected_entity = None,
                geng::Key::Num2 => self.selected_entity = Some(EntityType::Bush),
                geng::Key::Num3 => self.selected_entity = Some(EntityType::Cat),
                geng::Key::Num4 => self.selected_entity = Some(EntityType::Dog),
                geng::Key::Num5 => self.selected_entity = Some(EntityType::Mouse),
                geng::Key::S => {
                    if self.geng.window().is_key_pressed(geng::Key::LCtrl) {
                        batbox::save_file(
                            "Save custom level",
                            "levels/custom/custom_level.json",
                            |writer| {
                                serde_json::to_writer(writer, &self.level)?;
                                Ok(())
                            },
                        )
                        .unwrap();
                    }
                }
                geng::Key::R => {
                    self.transition = Some(geng::Transition::Push(Box::new(GameState::new(
                        &self.geng,
                        &self.assets,
                        self.level.clone(),
                    ))));
                }
                _ => (),
            },
            _ => (),
        }
    }
    fn transition(&mut self) -> Option<geng::Transition> {
        self.transition.take()
    }
}
