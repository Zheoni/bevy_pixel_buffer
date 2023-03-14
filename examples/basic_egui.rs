use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_pixel_buffer::prelude::*;

fn main() {
    let size = PixelBufferSize {
        size: UVec2::new(32, 32),
        pixel_size: UVec2::new(16, 16),
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugins(PixelBufferPlugins)
        .add_startup_system(
            PixelBufferBuilder::new()
                .with_size(size)
                .with_render(false) // disable rendering, as we'll do in it egui
                .setup(),
        )
        .add_system(update)
        .run()
}

fn update(mut egui_context: EguiContexts, mut pb: QueryPixelBuffer) {
    // update the frame
    pb.frame().per_pixel(|_, _| Pixel::random());

    // show ui
    let ctx = egui_context.ctx_mut();
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        ui.heading("My controls");
        ui.label("Look! Pixels!!")
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        // get the egui texture
        let texture = pb.egui_texture();

        // show the texture as an image
        ui.image(texture.id, texture.size);
    });
}
