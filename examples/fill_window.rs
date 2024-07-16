use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PixelBufferPlugin))
        .add_systems(
            Startup,
            PixelBufferBuilder::new()
                .with_size(PixelBufferSize::pixel_size((16, 16))) // only set pixel_size as size will be dynamically updated
                .with_fill(Fill::window()) // set fill to the window
                .setup(),
        )
        .add_systems(Update, update)
        .run();
}

fn update(mut pb: QueryPixelBuffer) {
    pb.frame().per_pixel(|_, _| Pixel::random());
}
