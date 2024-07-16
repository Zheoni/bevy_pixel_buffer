use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_pixel_buffer::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EguiPlugin, PixelBufferPlugins))
        .add_systems(
            Startup,
            PixelBufferBuilder::new()
                .with_size(PixelBufferSize::pixel_size((32, 32)))
                .with_render(false) // disable rendering, as we'll do in it egui
                .with_fill(Fill::stretch()) // Pixels will stretch to fill the area
                .setup(),
        )
        .add_systems(Update, update)
        .run();
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
        // update Fill component with available size
        pb.update_fill_egui(ui.available_size());

        // get the texture
        let texture = pb.egui_texture();

        // show it
        ui.image(egui::load::SizedTexture::new(texture.id, texture.size));
    });
}
