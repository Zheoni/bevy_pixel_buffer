use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
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
        .add_plugin(ComputeShaderPlugin::<MandelbrotSetShader>::default())
        .add_startup_system(setup)
        .add_system(process_input)
        .add_system(ui)
        .run()
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut cs: ResMut<Assets<MandelbrotSetShader>>,
) {
    PixelBufferBuilder::new()
        .with_size((1280, 720))
        .with_fill(Fill::window().with_stretch(true).with_scaling_multiple(8))
        .spawn(&mut commands, &mut images)
        .entity()
        .insert(cs.add(MandelbrotSetShader::default()));
}

fn process_input(
    pb: Query<&Handle<MandelbrotSetShader>>,
    mut cs: ResMut<Assets<MandelbrotSetShader>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let state = &mut cs.get_mut(pb.single()).unwrap().params;

    const MOVE_SPEED: f32 = 0.02;
    if keyboard_input.pressed(KeyCode::A) {
        state.center.x -= state.scale * MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::D) {
        state.center.x += state.scale * MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::W) {
        state.center.y += state.scale * MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::S) {
        state.center.y -= state.scale * MOVE_SPEED;
    }
    const SCALE_SPEED: f32 = 1.1;
    if keyboard_input.pressed(KeyCode::Q) {
        state.scale *= SCALE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::E) {
        state.scale /= SCALE_SPEED;
    }
}

fn ui(
    mut egui_ctx: EguiContexts,
    pb: Query<&Handle<MandelbrotSetShader>>,
    mut cs: ResMut<Assets<MandelbrotSetShader>>,
    diagnostics: Res<Diagnostics>,
) {
    let params = &mut cs.get_mut(pb.single()).unwrap().params;
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .unwrap()
        .average()
        .unwrap_or_default();

    let ctx = egui_ctx.ctx_mut();
    egui::Window::new("Mandelbrot set visualization").show(ctx, |ui| {
        ui.collapsing(RichText::new("About").heading(), |ui| {
            ui.label(concat!(
                "Mandelbrot set visuzalizer in the GPU. Fast. ",
                "CPU alternative can zoom more because it uses 64 bits of precision, ",
                "which is currently not supported by WGSL."
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

#[derive(AsBindGroup, TypeUuid, Clone, Debug, Default)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
struct MandelbrotSetShader {
    #[uniform(0)]
    params: Params,
}

#[derive(ShaderType, Clone, Debug)]
struct Params {
    max_iter: i32,
    scale: f32,
    center: Vec2,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            max_iter: 128,
            scale: 1.0,
            center: Vec2::ZERO,
        }
    }
}

impl ComputeShader for MandelbrotSetShader {
    fn shader() -> ShaderRef {
        "mandelbrot.wgsl".into()
    }

    fn entry_point() -> std::borrow::Cow<'static, str> {
        "update".into()
    }

    fn workgroups(texture_size: UVec2) -> UVec2 {
        texture_size / 8
    }
}
