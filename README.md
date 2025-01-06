# bevy_pixel_buffer

[![Crates.io](https://img.shields.io/crates/v/bevy_pixel_buffer)](https://crates.io/crates/bevy_pixel_buffer)
[![docs.rs](https://img.shields.io/docsrs/bevy_pixel_buffer)](https://docs.rs/bevy_pixel_buffer/)
![Crates.io](https://img.shields.io/crates/l/bevy_pixel_buffer)

A library to draw pixels in [bevy](https://crates.io/crates/bevy).

- Easy to set up and use.
- Can be integrated into an existing project.
- Allows dynamic resize of the pixel buffer to fill an area such as the window.
- Support for multiple pixel buffers.
- Allows to easily attach a compute shader to update the pixels.
- [egui](https://crates.io/crates/egui) integration (through
  [bevy_egui](https://crates.io/crates/bevy_egui)) to show the pixels inside the
  UI.

## [Examples](./examples/)

A basic example,

```rust
use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;

fn main() {
    let size = PixelBufferSize {
        size: UVec2::new(32, 32),
        pixel_size: UVec2::new(16, 16),
    };

    App::new()
        .add_plugins((DefaultPlugins, PixelBufferPlugin))
        .add_systems(Startup, pixel_buffer_setup(size))
        .add_systems(Update, update)
        .run();
}

fn update(mut pb: QueryPixelBuffer) {
    pb.frame().per_pixel(|_, _| Pixel::random());
}
```

![Basic example output](./images/basic.png)

[More examples](./examples/)

## Features

- `egui`\*. Egui integration.
- `rayon`. Enables extra alternative functions that use rayon.
- `rand`. Enables extra functionality related to random values.

\* Disabled by default.

### Using the Egui integration

To use the Egui integration, depend on `bevy_egui` and enable the feature by passing `--features "egui"` to your cargo calls, or permanently add the feature through your `Cargo.toml` like this

```
[dependencies]
bevy_pixel_buffer = { version = "*", features = ["egui"] }
bevy_egui = "0.30" # (for bevy 14)
```

## Documentation

Further rendered documentation can be found at [docs.rs/bevy_pixel_buffer](https://docs.rs/bevy_pixel_buffer/latest/bevy_pixel_buffer/).

## Bevy versions

Version compatibility table.

| `bevy` | `bevy_pixel_buffer` |
| ------ | ------------------- |
| `0.14` | `0.8`               |
| `0.13` | `0.7`               |
| `0.12` | `0.6`               |
| `0.11` | `0.5`               |
| `0.10` | `0.4`               |
| `0.9`  | `0.3`               |
| `0.8`  | `0.2`               |
