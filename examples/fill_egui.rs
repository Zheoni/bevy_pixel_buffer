use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_pixel_buffer::{egui::PixelBufferEguiPlugin, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(PixelBufferPlugin)
        .add_plugin(PixelBufferEguiPlugin)
        .add_startup_system(
            PixelBufferBuilder::new()
                .with_size(PixelBufferSize::pixel_size((16, 16)))
                .with_render(false) // disable rendering, as we'll do in it egui
                .setup(),
        )
        .add_system(update)
        .run()
}

fn update(mut egui_context: ResMut<EguiContext>, mut pb: QueryPixelBufferEgui) {
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
        ui.image(texture.id, texture.size);
    });
}
