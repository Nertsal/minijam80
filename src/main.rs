use geng::prelude::*;

mod camera;
mod model;
mod renderer;

use camera::*;
use model::*;
use renderer::*;

struct State {
    model: Model,
    renderer: Renderer,
}

impl State {
    fn new(geng: &Rc<Geng>, assets: &Rc<Assets>) -> Self {
        Self {
            renderer: Renderer::new(geng, assets),
            model: Model::new(),
        }
    }
}

impl geng::State for State {
    fn update(&mut self, delta_time: f64) {
        self.model.update(delta_time as f32);
        self.renderer.update(delta_time as f32);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.renderer.draw(framebuffer, &self.model);
    }
    fn handle_event(&mut self, event: geng::Event) {
        self.model.handle_event(&event);
        self.renderer.handle_event(&event);
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
                State::new(&geng, &Rc::new(assets))
            }
        }),
    );
}
