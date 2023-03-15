# bevy_pixel_buffer examples

Example | Description
--- | ---
[basic](./basic.rs) | Basic setup to just draw.
[fill_window](./fill_window.rs) | Dynamically resize the pixel buffer to fill the window.
[multiple_buffers](./multiple_buffers.rs)* | Draw multiple pixel buffers at once.
[game of life](./game_of_life.rs) | Game of life with with a compute shader.
[mandelbrot_set](./mandelbrot_set.rs)* | Interactive mandelbrot set with a compute shader.
[mandelbrot_set_cpu](./mandelbrot_set_cpu.rs)* | Interactive mandlebrot set calculated in the CPU.
[resize](./resize.rs) | Resize the pixel buffer programatically.
[bundle](./bundle.rs) | Manually create a pixel buffer with a bundle. Equivalent to [custom_sprite](./custom_sprite.rs).
[custom_sprite](./custom_sprite.rs) | Render as a sprite with custom parameters. Equivalent to [bundle](./bundle.rs).

\* Uses `egui` to demo, but is not required.

## egui integration

Example | Description
--- | ---
[basic_egui](./basic_egui.rs) |  Basic setup to just draw inside egui.
[fill_egui](./fill_egui.rs) | Dynamically resize the pixel buffer to fill an egui aera. The pixels also stretch.
