use bevy::prelude::*;
use bevy_pixel_buffer::{
    bundle::PixelBufferSpriteBundle,
    pixel_buffer::{create_image, CreateImageParams},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelBufferPlugin)
        .add_startup_system(setup)
        .add_system(update)
        .run()
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());

    let size = PixelBufferSize {
        size: UVec2::new(32, 32),
        pixel_size: UVec2::new(16, 16),
    };

    commands.spawn(PixelBufferSpriteBundle {
        pixel_buffer: PixelBuffer {
            size,
            fill: Fill::none(),
        },
        sprite_bundle: SpriteBundle {
            //important, use `create_image`
            texture: images.add(create_image(CreateImageParams {
                size: size.size,
                ..Default::default()
            })),
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
