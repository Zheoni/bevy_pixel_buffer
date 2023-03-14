use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    math::{DVec2, DVec4},
    prelude::*,
};
use bevy_egui::{
    egui::{self, RichText},
    EguiContexts, EguiPlugin,
};
use bevy_pixel_buffer::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PixelBufferPlugin)
        .add_startup_system(
            PixelBufferBuilder::new()
                .with_size((1280, 720))
                .with_fill(Fill::window().with_stretch(true))
                .setup(),
        )
        .add_system(process_input)
        .add_system(ui)
        .add_system(render.after(process_input))
        .insert_resource(Params::default())
        .run()
}

fn process_input(mut params: ResMut<Params>, keyboard_input: Res<Input<KeyCode>>, time: Res<Time>) {
    let state = params.as_mut();
    let delta = time.delta().as_secs_f64();
    const MOVE_SPEED: f64 = 0.2;
    if keyboard_input.pressed(KeyCode::A) {
        state.center.x -= state.scale * MOVE_SPEED * delta;
    }
    if keyboard_input.pressed(KeyCode::D) {
        state.center.x += state.scale * MOVE_SPEED * delta;
    }
    if keyboard_input.pressed(KeyCode::W) {
        state.center.y += state.scale * MOVE_SPEED * delta;
    }
    if keyboard_input.pressed(KeyCode::S) {
        state.center.y -= state.scale * MOVE_SPEED * delta;
    }
    const SCALE_SPEED: f64 = 1.1;
    if keyboard_input.pressed(KeyCode::Q) {
        state.scale *= SCALE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::E) {
        state.scale /= SCALE_SPEED;
    }
}

fn ui(mut egui_ctx: EguiContexts, diagnostics: Res<Diagnostics>, mut params: ResMut<Params>) {
    let params = params.as_mut();
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .unwrap()
        .average()
        .unwrap_or_default();

    let ctx = egui_ctx.ctx_mut();
    egui::Window::new("CPU Mandelbrot set visualization").show(ctx, |ui| {
        ui.collapsing(RichText::new("About").heading(), |ui| {
            ui.label(concat!(
                "Mandelbrot set visuzalizer in the CPU. Slower than the ",
                "GPU alternative, but can zoom more because it uses 64 bits of ",
                "precision, currently not supported in WGSL."
            ));
        });
        ui.heading("Controls");
        egui::Grid::new("controls")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Move");
                ui.label("WASD / Arrows");
                ui.end_row();

                ui.label("Zoom in");
                ui.label("Q");
                ui.end_row();
                ui.label("Zoom out");
                ui.label("E");
                ui.end_row();
            });

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("FPS: ");
            ui.label(RichText::new(format!("{fps:.1}")).code());
        });
        ui.horizontal(|ui| {
            ui.label("Max iterations");
            ui.add(egui::Slider::new(&mut params.max_iter, 64..=2048));
        });
    });
}

fn render(mut pb: QueryPixelBuffer, params: Res<Params>) {
    fn square_complex(c: DVec2) -> DVec2 {
        DVec2::new(c.x * c.x - c.y * c.y, 2.0 * c.x * c.y)
    }

    let mut frame = pb.frame();
    let dimensions = frame.size();
    frame.per_pixel_par(|pos, _| {
        let max_iter = params.max_iter;
        let center = params.center;
        let scale = params.scale;

        let w = dimensions.x as f64;
        let h = dimensions.y as f64;
        let aspect = w / h;

        let x = scale * aspect * (pos.x as f64 - 0.5 * w) / w + center.x;
        let y = -scale * (pos.y as f64 - 0.5 * h) / h + center.y;

        let c = DVec2::new(x, y);
        let mut z = DVec2::ZERO;

        let mut b = -1;
        for i in 0..max_iter {
            if (z.x * z.x + z.y * z.y) > 4.0 {
                b = i;
                break;
            }
            z = square_complex(z) + c;
        }
        if b == -1 {
            b = max_iter;
        }
        let intensity = b as f64 / max_iter as f64;
        let intensity = (2.0 * intensity) / (intensity.abs() + 1.0);
        let r = f64::max(0.0, 2.0 * intensity - 1.0);

        DVec4::new(r, intensity, intensity, 1.0)
    });
}

#[derive(Clone, Debug, Resource)]
struct Params {
    max_iter: i32,
    scale: f64,
    center: DVec2,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            max_iter: 128,
            scale: 1.0,
            center: DVec2::ZERO,
        }
    }
}
