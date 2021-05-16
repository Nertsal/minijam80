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
