use super::*;

#[derive(ugli::Vertex, Clone)]
pub struct Vertex {
    pub a_pos: Vec2<f32>,
}

pub struct Renderer {
    quad: ugli::VertexBuffer<Vertex>,
    program: ugli::Program,
}

impl Renderer {
    pub fn new(geng: &Rc<Geng>) -> Self {
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
        }
    }
    pub fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        matrix: Mat4<f32>,
        texture_matrix: Mat4<f32>,
        texture: &ugli::Texture,
        color: Color<f32>,
    ) {
        let camera_uniforms = camera.uniforms(framebuffer.size().map(|x| x as f32));
        let uniforms = (
            camera_uniforms,
            ugli::uniforms! {
                u_model_matrix: matrix,
                u_texture: texture,
                u_texture_matrix: texture_matrix,
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
    pub fn draw_text(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        text: &str,
        pos: Vec2<f32>,
        align: f32,
        size: f32,
        font: &ugli::Texture,
        color: Color<f32>,
    ) {
        let mut x = pos.x - text.len() as f32 * align * size;
        const CHARS: &str = "abcdefghijklmnopqrstuvwxyz0123456789";
        for c in text.chars() {
            if c != ' ' {
                let idx = CHARS.find(c).unwrap();
                let ty = idx / 6;
                let tx = idx % 6;
                self.draw(
                    framebuffer,
                    camera,
                    Mat4::translate(vec3(x, pos.y, 0.0)) * Mat4::scale_uniform(size),
                    Mat4::scale_uniform(1.0 / 6.0)
                        * Mat4::translate(vec3(tx as f32, ty as f32, 0.0)),
                    font,
                    color,
                );
            }
            x += size;
        }
    }
}
