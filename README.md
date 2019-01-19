# Spheres

[![Build Status](https://www.travis-ci.org/m0dulo/Spheres.svg?branch=master)](https://www.travis-ci.org/m0dulo/Spheres)

This is a new ray tracer program implemented in [Rust](https://www.rust-lang.org/en-US/) based on [Ray Tracing in One Weekend](http://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf) and [Ray Tracing: The Next Week](http://www.realtimerendering.com/raytracing/Ray%20Tracing_%20The%20Next%20Week.pdf)

[Ray Tracing Minibooks Book 1 on Amazon](https://www.amazon.com/Ray-Tracing-Weekend-Minibooks-Book-ebook/dp/B01B5AODD8/)

[Ray Tracing Minibooks Book 2 on Amazon](https://www.amazon.com/Ray-Tracing-Next-Week-Minibooks-ebook/dp/B01CO7PQ8C)

I upgraded to Rust 2018 while employing some new features, thereby reducing memory-related overhead. And I switched all vectors from `f64` to `f32`, which may contribute to better performance. BTW, I made some benchmarks for this version.

## Showcases
### Sea

Generate 300 spheres (no overlap) in the space with different materials and settings.

It takes ~560 secs to render.

`scenes/complex_scene.rs:complex_scene_1`

![complex_scene_1](https://user-images.githubusercontent.com/17985352/51424659-eee95d80-1c0b-11e9-9bd5-e6de04fd096e.png)


`scenes/complex_scene.rs:complex_scene_2`

![complex_scene_2](https://user-images.githubusercontent.com/17985352/51424664-f90b5c00-1c0b-11e9-85f2-34694607b3c4.png)

### Textures
#### Light
`scenes/legacy_scene.rs:legacy_scene_light
`

![legacy_scene_light](https://user-images.githubusercontent.com/17985352/51424667-058fb480-1c0c-11e9-83de-e3b1edfb1eed.png)

#### Checker Texture
`scenes/legacy_scene.rs:legacy_scene_texture`

![legacy_scene_texture](https://user-images.githubusercontent.com/17985352/51424672-117b7680-1c0c-11e9-9824-4d94f717bfb0.png)
#### Perlin Noise
`scenes/simple_scene.rs:simple_scene_perlin_noise`

![simple_scene_perlin_noise](https://user-images.githubusercontent.com/17985352/51424673-12aca380-1c0c-11e9-94b9-0d2d55382be1.png)

### [Cornell Box](https://en.wikipedia.org/wiki/Cornell_box)
#### Spherical Harmonics
This image was computed by Francois Sillion, et al. as part of a research into using spherical harmonics to represent Bi-directional Reflectance Distribution Functions (BRDF) for surfaces. The reflectance properties of a green wall yield more balanced solutions.

`scenes/cornell_box.rs:cornell_box`

![cornell_box](https://user-images.githubusercontent.com/17985352/51424624-581ca100-1c0b-11e9-947d-5579cfafaeb4.png)
## Usage

* First, Rust has to be installed as described in the [official tutorial](https://www.rust-lang.org/en-US/install.html).
* Write your Write your specifications in `main.rs`:

  ```rust
  use self::renderer::utils::render_high_quality as render;
  use self::scenes::simple_scene::simple_scene_perlin_noise as scene;
   ```
   Here you can change `render_high_quality` to `render_preview` to render faster. And you can select from examples scenes by changing `self::scenes::****::****`.

   ```rust
   render(hitable_list, camera, "scene.png", true)?;
   ```
   The third parameter indicates that with previous rendering settings, the image will be rendered to `/output`.

   The fourth parameter indicates whether to enable ambient light or not.

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
