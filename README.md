<div align="center">
  <h1> Rayrust </h1>

  <p>
    Implementation of
<a href="https://raytracing.github.io/books/RayTracingInOneWeekend.html">
    <cite>Ray Tracing in One Weekend</cite>
</a>
    in Rust
  <p>
</div>

![Final Scene](images/final_image.png)

### Usage

Clone the repository:

```
git clone git@github.com:liskajiri/rayrust.git
```

Compile and run using:

```
cargo run --release
```

### Dependencies

- [rand](https://docs.rs/rand/latest/rand/) = Random number generator
- [image](https://docs.rs/image/latest/image/) = Saving images as png
- [rayon](https://docs.rs/rayon/latest/rayon/) = Parallelism
