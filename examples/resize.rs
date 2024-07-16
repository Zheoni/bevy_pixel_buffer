use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;

#[derive(Deref, DerefMut, Resource)]
struct ResizeTimer(Timer);

fn main() {
    let size = PixelBufferSize {
        size: UVec2::new(32, 32),
        pixel_size: UVec2::new(16, 16),
    };

    App::new()
        .add_plugins((DefaultPlugins, PixelBufferPlugin))
        .add_systems(Startup, PixelBufferBuilder::new().with_size(size).setup())
        // Resize applies at the beginning of next frame, update the image and
        // prepare the resize for the next frame
        .add_systems(Update, (update, resize).chain())
        .insert_resource(ResizeTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .run();
}

// update pixels when pixel buffer changes
fn update(image: Query<&Handle<Image>, Changed<PixelBuffer>>, mut images: ResMut<Assets<Image>>) {
    if let Ok(image) = image.get_single() {
        Frame::extract(&mut images, image).per_pixel(|_, _| Pixel::random());
    }
}

fn resize(
    time: Res<Time>,
    mut timer: ResMut<ResizeTimer>,
    mut pb: Query<&mut PixelBuffer>,
    mut toggle: Local<bool>,
) {
    timer.tick(time.delta());

    if timer.finished() {
        let mut pb = pb.single_mut();
        pb.size = if *toggle {
            PixelBufferSize {
                size: UVec2::new(32, 32),
                pixel_size: UVec2::new(16, 16),
            }
        } else {
            PixelBufferSize {
                size: UVec2::new(16, 16),
                pixel_size: UVec2::new(32, 32),
            }
        };
        *toggle = !*toggle;
    }
}
