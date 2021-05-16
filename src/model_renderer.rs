use super::*;

pub struct LevelRenderer {
    pub renderer: Renderer,
    assets: Rc<Assets>,
}

impl LevelRenderer {
    pub fn new(geng: &Rc<Geng>, assets: &Rc<Assets>) -> Self {
        Self {
            renderer: Renderer::new(geng),
            assets: assets.clone(),
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
                )
            }
        }

        for entity in level.entities.values() {
            self.renderer.draw(
                framebuffer,
                &camera,
                Mat4::translate(entity.position.map(|x| x as f32).extend(0.0)),
                Mat4::identity(),
                self.assets.entity(entity.entity_type),
                Color::WHITE,
            );
        }
    }
}
