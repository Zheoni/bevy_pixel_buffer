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
        .add_systems(PostStartup, draw_random)
        .add_systems(Update, update)
        .run()
}

fn draw_random(mut pb: QueryPixelBuffer) {
    pb.frame().per_pixel(|_, _| Pixel::random());
}

fn update(mut q: Query<&mut Transform, With<PixelBuffer>>, keys: Res<Input<KeyCode>>) {
    let movement = if keys.just_pressed(KeyCode::Left) {
        Some((-1, 0))
    } else if keys.just_pressed(KeyCode::Right) {
        Some((1, 0))
    } else if keys.just_pressed(KeyCode::Up) {
        Some((0, 1))
    } else if keys.just_pressed(KeyCode::Down) {
        Some((0, -1))
    } else {
        None
    };

    if let Some((x, y)) = movement {
        const M: f32 = 10.0;

        let mut transform = q.single_mut();
        transform.translation.x += x as f32 * M;
        transform.translation.y += y as f32 * M;
    }
}
