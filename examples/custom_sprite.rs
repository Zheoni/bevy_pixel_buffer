use bevy::prelude::*;
use bevy_pixel_buffer::{
    builder::{CustomSprite, CustomSpriteBundle},
    prelude::*,
};

fn main() {
    let size = PixelBufferSize {
        size: UVec2::new(32, 32),
        pixel_size: UVec2::new(16, 16),
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelBufferPlugin)
        .add_startup_system(
            PixelBufferBuilder::new()
                .with_size(size)
                .with_render(RenderConfig::Sprite {
                    spawn_camera: true,
                    sprite_bundle: CustomSpriteBundle {
                        sprite: CustomSprite {
                            color: Color::PINK,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(-100.0, -100.0, 0.0),
                        ..Default::default()
                    },
                })
                .setup(),
        )
        .add_system(update)
        .run()
}

fn update(mut pb: QueryPixelBuffer) {
    pb.frame().per_pixel(|_, _| Pixel::random());
}
