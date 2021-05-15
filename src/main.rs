use geng::prelude::*;

mod camera;
mod model;
mod renderer;
mod game_state;

use game_state::*;
use camera::*;
use model::*;
use renderer::*;

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
                GameState::new(&geng, &Rc::new(assets))
            }
        }),
    );
}
