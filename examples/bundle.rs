use bevy::prelude::*;
use bevy_pixel_buffer::{bundle::PixelBufferSpriteBundle, pixel_buffer::create_image, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelBufferPlugin)
        .add_startup_system(setup)
        .add_system(update)
        .run()
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let size = PixelBufferSize {
        size: UVec2::new(32, 32),
        pixel_size: UVec2::new(16, 16),
    };

    commands.spawn_bundle(PixelBufferSpriteBundle {
        pixel_buffer: PixelBuffer {
            size,
            fill: Fill::none(),
        },
        sprite: SpriteBundle {
            texture: create_image(&mut images, size.size), // <-- important, use `create_image`
            sprite: Sprite {
                color: Color::PINK,
                ..Default::default()
            },
            transform: Transform::from_xyz(-100.0, -100.0, 0.0),
            ..Default::default()
        },
    });
}

fn update(mut pb: QueryPixelBuffer) {
    pb.frame().per_pixel(|_, _| Pixel::random());
}
