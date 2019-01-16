#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

mod renderer;
mod scenes;
mod tracer;

use self::renderer::utils::render_high_quality as render;
use self::scenes::simple_scene::simple_scene_perlin_noise as scene;
use std::env;

fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();
    info!("generating scene...");
    let (world, camera) = scene();
    render(world, camera, "simple_scene_perlin_noise.png")?;
    Ok(())
}
