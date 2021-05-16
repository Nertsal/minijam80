use super::*;

pub struct LevelRenderer {
    pub renderer: Renderer,
    noise: noise::OpenSimplex,
    assets: Rc<Assets>,
}

impl LevelRenderer {
    pub fn new(geng: &Rc<Geng>, assets: &Rc<Assets>) -> Self {
        Self {
            renderer: Renderer::new(geng),
            assets: assets.clone(),
            noise: noise::OpenSimplex::new(),
        }
    }
    pub fn draw(&self, level: &Level, camera: &Camera, framebuffer: &mut ugli::Framebuffer) {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        let tile_low_left_pos = tile_pos(camera.screen_to_world(framebuffer_size, vec2(0.0, 0.0)));
        let tile_top_right_pos =
            tile_pos(camera.screen_to_world(framebuffer_size, framebuffer_size));
        for x in tile_low_left_pos.x..=tile_top_right_pos.x {
            for y in tile_low_left_pos.y..=tile_top_right_pos.y {
                let tile_pos = vec2(x, y);
                self.renderer.draw(
                    framebuffer,
                    &camera,
                    Mat4::translate(tile_pos.map(|x| x as f32).extend(0.0)),
                    Mat4::identity(),
                    &self.assets.grass,
                    Color::WHITE,
                );
                if true || level.get_entity(tile_pos).is_none() {
                    let nv = noise::NoiseFn::get(&self.noise, [x as f64 + 0.5, y as f64 + 0.5]);
                    let nv = nv / 0.55;
                    let mx = self.assets.flower.len() as i32 + 10;
                    let idx = clamp(((nv + 1.0) / 2.0 * mx as f64) as i32, 0..=mx);
                    if let Some(texture) = self.assets.flower.get(idx as usize) {
                        self.renderer.draw(
                            framebuffer,
                            &camera,
                            Mat4::translate(tile_pos.map(|x| x as f32).extend(0.0)),
                            // * Mat4::scale_uniform(0.3),
                            Mat4::identity(),
                            texture,
                            Color::WHITE,
                        );
                    }
                }
            }
        }

        for entity in level.entities.values() {
            self.renderer.draw(
                framebuffer,
                &camera,
                Mat4::translate(entity.render_pos.extend(0.0)),
                Mat4::identity(),
                self.assets.entity(entity.entity_type),
                Color::WHITE,
            );
        }
    }
}
