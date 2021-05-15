use super::*;

#[derive(Deref)]
pub struct Texture {
    #[deref]
    inner: ugli::Texture,
}

impl geng::LoadAsset for Texture {
    fn load(geng: &Rc<Geng>, path: &str) -> geng::AssetFuture<Self> {
        let geng = geng.clone();
        <ugli::Texture as geng::LoadAsset>::load(&geng, path)
            .map(move |data| {
                let mut data = data?;
                data.set_filter(ugli::Filter::Nearest);
                Ok(Self { inner: data })
            })
            .boxed_local()
    }

    const DEFAULT_EXT: Option<&'static str> = Some("png");
}

#[derive(Deref)]
pub struct Font {
    #[deref]
    inner: Rc<geng::Font>,
}

impl geng::LoadAsset for Font {
    fn load(geng: &Rc<Geng>, path: &str) -> geng::AssetFuture<Self> {
        let geng = geng.clone();
        <Vec<u8> as geng::LoadAsset>::load(&geng, path)
            .map(move |data| {
                Ok(Font {
                    inner: Rc::new(geng::Font::new(&geng, data?)?),
                })
            })
            .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("ttf");
}

#[derive(geng::Assets)]
pub struct Assets {
    pub cat: Texture,
    pub mouse: Texture,
    pub dog: Texture,
    pub grass: Texture,
    pub bush: Texture,
    #[asset(path = "flower*.png", range = "1..=3")]
    pub flower: Vec<Texture>,
    pub bone: Texture,
    #[asset(path = "box.png")]
    pub box_asset: Texture,
    pub cheese: Texture,
    pub doghouse: Texture,
    pub fence: Texture,
    pub wall: Texture,
    pub water: Texture,
}

pub struct GameState {
    geng: Rc<Geng>,
    assets: Rc<Assets>,
    camera: Camera,
    model: Model,
    renderer: Renderer,
    framebuffer_size: Vec2<f32>,
    selected_entity: Option<EntityType>,
}

impl GameState {
    pub fn new(geng: &Rc<Geng>, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: Camera::new(10.0),
            renderer: Renderer::new(geng),
            model: Model::new(),
            framebuffer_size: vec2(1.0, 1.0),
            selected_entity: None,
        }
    }
    fn camera_to_tile_pos(&self, position: Vec2<f32>) -> Vec2<i32> {
        self.camera
            .screen_to_world(self.framebuffer_size, position.map(|x| x as f32))
            .map(|x| x.floor() as i32)
    }
    fn draw_tile(&self, framebuffer: &mut ugli::Framebuffer, tile_pos: Vec2<i32>) {
        self.renderer.draw(
            framebuffer,
            &self.camera,
            Mat4::translate(tile_pos.map(|x| x as f32).extend(0.0)),
            &self.assets.grass,
            Color::WHITE,
        )
    }
    fn draw_entity(&self, framebuffer: &mut ugli::Framebuffer, entity: &Entity) {
        self.renderer.draw(
            framebuffer,
            &self.camera,
            Mat4::translate(entity.position.map(|x| x as f32).extend(0.0)),
            match &entity.entity_type {
                model::EntityType::Bush => &self.assets.bush,
                model::EntityType::Doghouse => &self.assets.doghouse,
                model::EntityType::Cat => &self.assets.cat,
                model::EntityType::Dog => &self.assets.dog,
                model::EntityType::Mouse => &self.assets.mouse,
            },
            Color::WHITE,
        );
    }
    fn spawn_selected(&mut self, mouse_position: Vec2<f64>, spawn_player: bool) {
        let tile_pos = self.camera_to_tile_pos(mouse_position.map(|x| x as f32));
        match self.selected_entity.clone() {
            Some(selected_entity) => self.model.set_entity(Entity {
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
            }),
            None => self.model.remove_entity(tile_pos),
        }
    }
}

impl geng::State for GameState {
    fn update(&mut self, delta_time: f64) {
        self.camera.update(delta_time as f32);
        self.model.update(delta_time as f32);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        if let Some(level) = &self.model.level {
            let tile_low_left_pos = self.camera_to_tile_pos(vec2(0.0, 0.0));
            let tile_top_right_pos = self.camera_to_tile_pos(self.framebuffer_size);
            for x in tile_low_left_pos.x..=tile_top_right_pos.x {
                for y in tile_low_left_pos.y..=tile_top_right_pos.y {
                    let tile_pos = vec2(x, y);
                    self.draw_tile(framebuffer, tile_pos);
                }
            }

            for entity in &level.entities {
                self.draw_entity(framebuffer, entity);
            }
        }
    }
    fn handle_event(&mut self, event: geng::Event) {
        match self.model.mode {
            Mode::Play => match event {
                geng::Event::KeyDown { key } => {
                    if let Some(player_move) = match key {
                        geng::Key::Right => Some(Move::Right),
                        geng::Key::Left => Some(Move::Left),
                        geng::Key::Up => Some(Move::Up),
                        geng::Key::Down => Some(Move::Down),
                        geng::Key::Space => Some(Move::Wait),
                        _ => None,
                    } {
                        self.model.make_move(player_move);
                    }
                }
                _ => (),
            },
            Mode::Edit => match event {
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
                            self.model.save_level();
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
        }

        if let geng::Event::KeyDown { key: geng::Key::R } = event {
            self.model.mode = match self.model.mode {
                Mode::Edit => Mode::Play,
                Mode::Play => Mode::Edit,
            }
        }
    }
}
