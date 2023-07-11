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
        .run()
}

fn update(mut pb: QueryPixelBuffer) {
    pb.frame().per_pixel(|_, _| Pixel::random());
}
