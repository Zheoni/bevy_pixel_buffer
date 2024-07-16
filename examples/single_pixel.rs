use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;
use rand::Rng;

fn main() {
    let size = PixelBufferSize {
        size: UVec2::new(16, 16),
        pixel_size: UVec2::new(32, 32),
    };

    App::new()
        .add_plugins((DefaultPlugins, PixelBufferPlugin))
        .add_systems(Startup, pixel_buffer_setup(size))
        .add_systems(Update, update)
        .run();
}

fn update(mut pb: QueryPixelBuffer) {
    let mut frame = pb.frame();
    let mut rng = rand::thread_rng();
    let pos = (
        rng.gen_range(0..frame.size().x),
        rng.gen_range(0..frame.size().y),
    );
    frame.set(pos, Pixel::random()).expect("out of bounds");
}
