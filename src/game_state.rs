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

#[derive(geng::Assets)]
pub struct Assets {
    pub cat: Texture,
    pub mouse: Texture,
    pub dog: Texture,
    pub grass: Texture,
    pub bush: Texture,
    #[asset(path = "flower*.png", range = "1..=3")]
    pub flower: Vec<Texture>,
}

pub struct GameState {
    assets: Rc<Assets>,
    camera: Camera,
    model: Model,
    renderer: Renderer,
}

impl GameState {
    pub fn new(geng: &Rc<Geng>, assets: &Rc<Assets>) -> Self {
        Self {
            assets: assets.clone(),
            camera: Camera::new(10.0),
            renderer: Renderer::new(geng),
            model: Model::new(),
        }
    }
}

impl geng::State for GameState {
    fn update(&mut self, delta_time: f64) {
        self.camera.update(delta_time as f32);
        self.model.update(delta_time as f32);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        if let Some(level) = &self.model.level {
            for (tile_pos, tile) in &level.tiles {
                self.renderer.draw(
                    framebuffer,
                    &self.camera,
                    Mat4::translate(tile_pos.map(|x| x as f32).extend(0.0)),
                    &self.assets.grass,
                    Color::WHITE,
                );
                if let Some(overlay_texture) = match tile {
                    Tile::Bush => Some(&self.assets.bush),
                    Tile::Empty => None,
                } {
                    self.renderer.draw(
                        framebuffer,
                        &self.camera,
                        Mat4::translate(tile_pos.map(|x| x as f32).extend(0.0)),
                        overlay_texture,
                        Color::WHITE,
                    );
                }
            }
            for creature in &level.creatures {
                self.renderer.draw(
                    framebuffer,
                    &self.camera,
                    Mat4::translate(creature.position.map(|x| x as f32).extend(0.0)),
                    match &creature.creature_type {
                        model::CreatureType::Player => &self.assets.cat,
                        model::CreatureType::Dog => &self.assets.dog,
                    },
                    Color::WHITE,
                );
            }
        }
    }
    fn handle_event(&mut self, event: geng::Event) {
        match event {
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
        }
    }
}
