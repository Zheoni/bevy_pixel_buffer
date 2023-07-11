use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_pixel_buffer::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PixelBufferPlugin,
            ComputeShaderPlugin::<GameOfLifeShader>::default(), // add a plugin to handle our shader
        ))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut cs: ResMut<Assets<GameOfLifeShader>>,
) {
    let size = PixelBufferSize {
        size: UVec2::new(600, 400),
        pixel_size: UVec2::new(2, 2),
    };

    PixelBufferBuilder::new()
        // custom size
        .with_size(size)
        .spawn(&mut commands, &mut images)
        // initialize the game of life with random cells
        .edit_frame(|frame| {
            frame.per_pixel(|_, _| {
                if rand::random::<f32>() > 0.9 {
                    Pixel::WHITE
                } else {
                    Pixel::TRANSPARENT
                }
            })
        })
        .entity()
        // insert the shader handle
        .insert(cs.add(GameOfLifeShader::default()));
}

#[derive(AsBindGroup, TypeUuid, TypePath, Clone, Debug, Default)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
struct GameOfLifeShader {}

impl ComputeShader for GameOfLifeShader {
    fn shader() -> ShaderRef {
        "game_of_life.wgsl".into()
    }

    fn entry_point() -> std::borrow::Cow<'static, str> {
        "update".into()
    }

    fn workgroups(texture_size: UVec2) -> UVec2 {
        texture_size / 8
    }
}
