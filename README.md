# SphereSea

This is a new ray tracer program implemented in [Rust](https://www.rust-lang.org/en-US/) based on [Ray Tracing in One Weekend](https://drive.google.com/drive/folders/14yayBb9XiL16lmuhbYhhvea8mKUUK77W)

[Ray Tracing Minibooks Book 1 on Amazon](https://www.amazon.com/Ray-Tracing-Weekend-Minibooks-Book-ebook/dp/B01B5AODD8/)

I upgraded to Rust 2018 while employing some new features, thereby reducing memory-related overhead. And I switched all vectors from f64 to f32, which may contribute to better performance. BTW, I made some benchmarks for this version.
## How to use

* First, *Rust* has to be installed as described in the [official tutorial](https://www.rust-lang.org/en-US/install.html).

* In this new version you should run this command to enable logging

  ```bash 
  RUST_LOG="raytracer=info" cargo run --release
  ```
  If succeed, you may see information like this

