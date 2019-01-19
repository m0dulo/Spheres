#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

mod renderer;
mod scenes;
mod tracer;

use self::renderer::utils::render_preview as render;
use self::scenes::cornell_box::cornell_box as scene;
use std::env;

fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();
    info!("generating scene...");
    let (hitable_list, camera) = scene();
    render(hitable_list, camera, "cornell_box.png", false)?;
    Ok(())
}
