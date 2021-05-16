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
                            last_attractor_pos: None,
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
    fn select_delta(&mut self, delta: i32) {
        let options: Vec<_> = EntityType::into_enum_iter().collect();
        let idx = options
            .iter()
            .position(|var| Some(*var) == self.selected_entity)
            .unwrap_or(options.len());
        self.selected_entity = options
            .get((idx as i32 + delta + options.len() as i32 + 1) as usize % (options.len() + 1))
            .copied();
    }
}

const BUTTON_SIZE: f32 = 32.0;

impl geng::State for Editor {
    fn update(&mut self, delta_time: f64) {}
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        self.camera.optimize(&self.level);
        self.level_renderer
            .draw(&self.level, &self.camera, framebuffer);
        self.geng.default_font().draw(
            framebuffer,
            &format!("{:?}", self.selected_entity),
            vec2(0.0, 0.0),
            32.0,
            Color::BLACK,
        );
        for (idx, entity) in EntityType::into_enum_iter().enumerate() {
            self.geng.draw_2d().textured_quad(
                framebuffer,
                AABB::pos_size(
                    vec2(idx as f32 * BUTTON_SIZE, BUTTON_SIZE),
                    vec2(BUTTON_SIZE, BUTTON_SIZE),
                ),
                self.assets.entity(entity),
                Color::WHITE,
            );
        }
    }
    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseDown {
                position,
                button: geng::MouseButton::Left,
            } => {
                for (idx, entity) in EntityType::into_enum_iter().enumerate() {
                    if AABB::pos_size(
                        vec2(idx as f32 * BUTTON_SIZE, BUTTON_SIZE),
                        vec2(BUTTON_SIZE, BUTTON_SIZE),
                    )
                    .contains(position.map(|x| x as f32))
                    {
                        if self.selected_entity == Some(entity) {
                            self.selected_entity = None;
                        } else {
                            self.selected_entity = Some(entity);
                        }
                        return;
                    }
                }
                self.spawn_selected(position, false)
            }
            geng::Event::MouseDown {
                position,
                button: geng::MouseButton::Right,
            } => self.spawn_selected(position, true),
            geng::Event::Wheel { delta } => {
                self.select_delta(if delta > 0.0 { 1 } else { -1 });
            }
            geng::Event::KeyDown { key } => match key {
                geng::Key::Num1 => self.selected_entity = None,
                geng::Key::Num2 => self.selected_entity = Some(EntityType::Bush),
                geng::Key::Num3 => self.selected_entity = Some(EntityType::Cat),
                geng::Key::Num4 => self.selected_entity = Some(EntityType::Dog),
                geng::Key::Num5 => self.selected_entity = Some(EntityType::Mouse),
                geng::Key::Num6 => self.selected_entity = Some(EntityType::Box),
                geng::Key::Num7 => self.selected_entity = Some(EntityType::Cheese),
                geng::Key::Num8 => self.selected_entity = Some(EntityType::Bone),
                geng::Key::S if self.geng.window().is_key_pressed(geng::Key::LCtrl) => {
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
                geng::Key::O if self.geng.window().is_key_pressed(geng::Key::LCtrl) => {
                    if let Some(path) = batbox::select_file("Load level") {
                        self.level =
                            serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();
                    }
                }
                geng::Key::R => {
                    self.transition = Some(geng::Transition::Push(Box::new(GameState::new(
                        &self.geng,
                        &self.assets,
                        self.level.clone(),
                        None,
                    ))));
                }
                geng::Key::PageUp => self.select_delta(1),
                geng::Key::PageDown => self.select_delta(-1),
                _ => (),
            },
            _ => (),
        }
    }
    fn transition(&mut self) -> Option<geng::Transition> {
        self.transition.take()
    }
}
