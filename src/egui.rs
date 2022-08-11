//! Adds integrations with [bevy_egui] and other [egui] types. This modules
//! requires the `"egui"` feature.

use bevy::{math::Vec2, prelude::*};
use bevy_egui::{
    egui::{self, Color32},
    EguiContext,
};

use crate::{
    pixel_buffer::{Fill, FillKind, PixelBufferSize},
    prelude::Pixel,
};

/// Component inserted by the [PixelBufferEguiPlugin]. Holds all the necesary
/// information to draw a egui image with the pixel buffer.
#[derive(Component, Clone, Copy, Debug)]
pub struct EguiTexture {
    /// Egui texture ID
    pub id: egui::TextureId,
    /// Natural Size of the texture
    pub size: egui::Vec2,
}

/// Plugin that adds a [EguiTexture] component to all pixel buffers and keeps them up to date
pub struct PixelBufferEguiPlugin;

impl Plugin for PixelBufferEguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::PreUpdate, // So EguiTexture updates after pixel buffer is done, but before user updates the frame
            "pixel_buffer_egui",
            SystemStage::single_threaded(),
        )
        .add_system_to_stage("pixel_buffer_egui", register_egui)
        .add_system_to_stage(
            "pixel_buffer_egui",
            update_egui_texture_size.after(register_egui),
        );
    }
}

#[allow(clippy::type_complexity)]
fn register_egui(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    pixel_buffer: Query<(Entity, &PixelBufferSize, &Handle<Image>), Added<Handle<Image>>>,
) {
    for (entity, size, image_handle) in pixel_buffer.iter() {
        let texture = EguiTexture {
            id: egui_ctx.add_image(image_handle.clone_weak()),
            size: size.egui_texture_size(),
        };
        commands.entity(entity).insert(texture);
    }
}

fn update_egui_texture_size(
    mut pixel_buffer: Query<(&PixelBufferSize, &mut EguiTexture), Changed<PixelBufferSize>>,
) {
    for (size, mut texture) in pixel_buffer.iter_mut() {
        texture.size = size.egui_texture_size();
    }
}

/*

EGUI EXTENSIONS FOR OTHER TYPES IN THE CRATE

*/

impl PixelBufferSize {
    /// Gets the size of the texture for the pixel buffer
    pub fn egui_texture_size(&self) -> egui::Vec2 {
        let sz = self.screen_size();
        egui::Vec2::new(sz.x as f32, sz.y as f32)
    }
}

impl Fill {
    /// Updates the [Fill] component to a [egui::Vec2] size.
    pub fn update_egui(&mut self, evec2: egui::Vec2) {
        self.kind = FillKind::Custom(Vec2::new(evec2.x, evec2.y));
    }
}

impl From<bevy_egui::egui::Color32> for Pixel {
    fn from(c: bevy_egui::egui::Color32) -> Self {
        c.to_array().into()
    }
}

impl Pixel {
    /// Gets the color as the egui Color32 sRGB color type
    pub fn as_egui_color32(self) -> egui::Color32 {
        Color32::from_rgba_unmultiplied(self.r, self.g, self.b, self.a)
    }

    /// Get the color as the egui RGBA linear color type
    pub fn as_egui_rgba(self) -> egui::Rgba {
        egui::Rgba::from_rgba_unmultiplied(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }
}
