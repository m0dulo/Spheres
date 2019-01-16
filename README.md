# Spheres

This is a new ray tracer program implemented in [Rust](https://www.rust-lang.org/en-US/) based on [Ray Tracing in One Weekend](http://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf) and [Ray Tracing: The Next Week](http://www.realtimerendering.com/raytracing/Ray%20Tracing_%20The%20Next%20Week.pdf)

[Ray Tracing Minibooks Book 1 on Amazon](https://www.amazon.com/Ray-Tracing-Weekend-Minibooks-Book-ebook/dp/B01B5AODD8/)

[Ray Tracing Minibooks Book 2 on Amazon](https://www.amazon.com/Ray-Tracing-Next-Week-Minibooks-ebook/dp/B01CO7PQ8C)

I upgraded to Rust 2018 while employing some new features, thereby reducing memory-related overhead. And I switched all vectors from `f64` to `f32`, which may contribute to better performance. BTW, I made some benchmarks for this version.

## Showcases
### Sea

Generate 300 spheres (no overlap) in the space with different materials and settings.

It takes ~560 secs to render.

`scenes/complex_scene.rs:complex_scene_1`

![complex_scene_1.png](https://i.loli.net/2019/01/15/5c3dceceb412c.png)

`scenes/complex_scene.rs:complex_scene_2`

![complex_scene_2.png](https://i.loli.net/2019/01/15/5c3dcef4cf23e.png)

### Textures
#### Checker Texture
`scenes/legacy_scene.rs:legacy_scene_texture`

![checker_texture.png](https://i.loli.net/2019/01/16/5c3f2545e942b.png)
#### Perlin Noise
`scenes/simple_scene.rs:simple_scene_perlin_noise`

![simple_scene_perlin_noise.png](https://i.loli.net/2019/01/16/5c3f256b9af6d.png)
## How to use

* First, Rust has to be installed as described in the [official tutorial](https://www.rust-lang.org/en-US/install.html).
* Write your Write your specifications in `main.rs`:

  ```rust
  use self::renderer::utils::render_high_quality as render;
  use self::scenes::simple_scene::simple_scene_perlin_noise as scene;
   ```
   Here you can change `render_high_quality` to `render_preview` to render faster. And you can select from examples scenes by changing `self::scenes::****::****`.

   ```rust
   render(world, camera, "scene.png")?;
   ```
   The third parameter indicates that with previous rendering settings, the image will be rendered to `/output`.
* Run:

  ```bash 
  RUST_LOG="raytracer=info" cargo run --release
  ```
  If succeed, you may see information like this

  ![logging](https://i.loli.net/2019/01/15/5c3dcb929c472.png)

  the image will be generated at `/output`.

## Tests and Benchmarks

```bash
cargo test
cargo bench
```
