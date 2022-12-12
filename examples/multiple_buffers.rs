use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_pixel_buffer::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugins(PixelBufferPlugins)
        .add_startup_system(setup)
        .add_system(update)
        .add_system(delete)
        .insert_resource(NextId(1))
        .run()
}

#[derive(Component)]
struct MyBuffer {
    shown: bool,
    id: usize,
}

#[derive(Resource)]
struct NextId(usize);

impl NextId {
    fn get(&mut self) -> usize {
        let id = self.0;
        self.0 += 1;
        id
    }
}

const SIZE: PixelBufferSize = PixelBufferSize {
    size: UVec2 { x: 9, y: 9 },
    pixel_size: UVec2 { x: 10, y: 10 },
};

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, mut id: ResMut<NextId>) {
    const COLORS: [[f32; 3]; 3] = [[1.0, 0.2, 0.2], [0.2, 1.0, 0.2], [0.2, 0.2, 1.0]];

    for i in 0..3 {
        insert_pixel_buffer(&mut commands, &mut images, &COLORS[i], id.get());
    }
}

fn update(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut images: ResMut<Assets<Image>>,
    mut buffers: Query<(&mut MyBuffer, &EguiTexture)>,
    mut id: ResMut<NextId>,
    mut new_color: Local<[f32; 3]>,
) {
    let ctx = egui_context.ctx_mut();

    egui::Window::new("Add new").show(ctx, |ui| {
        ui.color_edit_button_rgb(&mut new_color);
        if ui.button("Add new").clicked() {
            insert_pixel_buffer(&mut commands, &mut images, &new_color, id.get())
        }
    });

    for (mut buffer, texture) in buffers.iter_mut() {
        egui::Window::new(format!("Buffer {}", buffer.id))
            .collapsible(false)
            .open(&mut buffer.shown)
            .show(ctx, |ui| ui.image(texture.id, texture.size));
    }
}

fn delete(mut commands: Commands, buffers: Query<(Entity, &mut MyBuffer)>) {
    for (entity, buffer) in buffers.iter() {
        if !buffer.shown {
            commands.entity(entity).despawn();
        }
    }
}

fn insert_pixel_buffer(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    color: &[f32; 3],
    id: usize,
) {
    PixelBufferBuilder::new()
        .with_render(false)
        .with_size(SIZE)
        .spawn(commands, images)
        .edit_frame(|frame| {
            let size = frame.size();
            frame.per_pixel(|p, _| {
                let a = if (p.x + p.y * size.x) % 2 == 0 {
                    1.0
                } else {
                    0.8
                };
                Color::rgba_linear(color[0], color[1], color[2], a)
            });
        })
        .entity()
        .insert(MyBuffer { shown: true, id });
}
