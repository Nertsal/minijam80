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
    #[asset(path = "flower*.png", range = "1..=3")]
    pub flower: Vec<Texture>,
}

#[derive(ugli::Vertex, Clone)]
pub struct Vertex {
    pub a_pos: Vec2<f32>,
}

pub struct Renderer {
    quad: ugli::VertexBuffer<Vertex>,
    program: ugli::Program,
    camera: Camera,
    assets: Rc<Assets>,
}

impl Renderer {
    pub fn new(geng: &Rc<Geng>, assets: &Rc<Assets>) -> Self {
        Self {
            quad: ugli::VertexBuffer::new_static(
                geng.ugli(),
                vec![
                    Vertex {
                        a_pos: vec2(0.0, 0.0),
                    },
                    Vertex {
                        a_pos: vec2(1.0, 0.0),
                    },
                    Vertex {
                        a_pos: vec2(1.0, 1.0),
                    },
                    Vertex {
                        a_pos: vec2(0.0, 1.0),
                    },
                ],
            ),
            program: geng
                .shader_lib()
                .compile(include_str!("program.glsl"))
                .unwrap(),
            camera: Camera::new(10.0),
            assets: assets.clone(),
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        self.camera.update(delta_time);
    }
    pub fn handle_event(&mut self, event: &geng::Event) {}
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer, model: &model::Model) {
        for (tile_pos, tile) in &model.tiles {
            self.draw_texture(
                framebuffer,
                &self.camera,
                Mat4::translate(tile_pos.map(|x| x as f32).extend(0.0)),
                &self.assets.grass,
                Color::WHITE,
            )
        }
        for creature in &model.creatures {
            self.draw_texture(
                framebuffer,
                &self.camera,
                Mat4::translate(creature.position.map(|x| x as f32).extend(0.0)),
                match &creature.creature_type {
                    model::CreatureType::Player => &self.assets.cat,
                    model::CreatureType::Dog => &self.assets.dog,
                },
                Color::WHITE,
            )
        }
    }
    fn draw_texture(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        matrix: Mat4<f32>,
        texture: &ugli::Texture,
        color: Color<f32>,
    ) {
        let camera_uniforms = camera.uniforms(framebuffer.size().map(|x| x as f32));
        let uniforms = (
            camera_uniforms,
            ugli::uniforms! {
                u_model_matrix: matrix,
                u_texture: texture,
                u_color: color,
            },
        );
        ugli::draw(
            framebuffer,
            &self.program,
            ugli::DrawMode::TriangleFan,
            &self.quad,
            uniforms,
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        );
    }
}
