use enum_iterator::IntoEnumIterator;
use geng::prelude::*;

mod assets;
mod camera;
mod editor;
mod game_state;
mod level;
mod model_renderer;
mod renderer;

use assets::*;
use camera::*;
use editor::*;
use game_state::*;
use level::*;
use model_renderer::*;
use renderer::*;

fn tile_pos(pos: Vec2<f32>) -> Vec2<i32> {
    pos.map(|x| x.floor() as i32)
}

impl Camera {
    pub fn optimize(&mut self, level: &Level) {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        for entity in level.entities.values() {
            min_x = min_x.min(entity.position.x);
            min_y = min_y.min(entity.position.y);
            max_x = max_x.max(entity.position.x);
            max_y = max_y.max(entity.position.y);
        }
        if min_x <= max_x {
            max_x += 1;
            max_y += 1;
            self.fov = (max_y - min_y + 5) as f32;
            self.center = vec2(min_x + max_x, min_y + max_y).map(|x| x as f32) / 2.0;
        }
    }
}

fn main() {
    geng::setup_panic_handler();
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        std::env::set_current_dir(std::path::Path::new(&dir).join("static")).unwrap();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = std::env::current_exe().unwrap().parent() {
                std::env::set_current_dir(path).unwrap();
            }
        }
    }
    let geng = Rc::new(Geng::new(default()));
    let assets = <Assets as geng::LoadAsset>::load(&geng, ".");
    geng::run(
        geng.clone(),
        geng::LoadingScreen::new(&geng, geng::EmptyLoadingScreen, assets, {
            let geng = geng.clone();
            move |assets| {
                let assets = assets.unwrap();
                let assets = Rc::new(assets);
                if std::env::args().any(|arg| arg == "editor") {
                    Box::new(Editor::new(&geng, &assets)) as Box<dyn geng::State>
                } else {
                    Box::new(GameState::new(
                        &geng,
                        &assets,
                        assets.levels[0].clone(),
                        Some(1),
                    )) as Box<dyn geng::State>
                }
            }
        }),
    );
}
