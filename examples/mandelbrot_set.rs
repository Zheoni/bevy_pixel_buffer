use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};
use bevy_egui::{
    egui::{self, RichText},
    EguiContexts, EguiPlugin,
};
use bevy_pixel_buffer::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin,
            FrameTimeDiagnosticsPlugin::default(),
            PixelBufferPlugin,
            ComputeShaderPlugin::<MandelbrotSetShader>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (process_input, ui))
        .run();
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let state = &mut cs.get_mut(pb.single()).unwrap().params;

    const MOVE_SPEED: f32 = 0.02;
    if keyboard_input.pressed(KeyCode::KeyA) {
        state.center.x -= state.scale * MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        state.center.x += state.scale * MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        state.center.y += state.scale * MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        state.center.y -= state.scale * MOVE_SPEED;
    }
    const SCALE_SPEED: f32 = 1.1;
    if keyboard_input.pressed(KeyCode::KeyQ) {
        state.scale *= SCALE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        state.scale /= SCALE_SPEED;
    }
}

fn ui(
    mut egui_ctx: EguiContexts,
    pb: Query<&Handle<MandelbrotSetShader>>,
    mut cs: ResMut<Assets<MandelbrotSetShader>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let params = &mut cs.get_mut(pb.single()).unwrap().params;
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
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

#[derive(Asset, AsBindGroup, TypePath, Clone, Debug, Default)]
#[type_path = "example::mandelbrot_set_shader"]
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
