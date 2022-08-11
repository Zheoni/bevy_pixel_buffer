use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;

fn main() {
    let size = PixelBufferSize {
        size: UVec2::new(32, 32),
        pixel_size: UVec2::new(16, 16),
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelBufferPlugin)
        .add_startup_system(pixel_buffer_setup(size))
        .add_system(update)
        .run()
}

fn update(mut pb: QueryPixelBuffer) {
    pb.frame().per_pixel(|_, _| Pixel::random());
}
