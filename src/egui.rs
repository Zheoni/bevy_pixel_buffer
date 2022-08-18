//! Adds integrations with [bevy_egui] and other [egui] types. This modules
//! requires the `egui` feature.

use bevy::{math::Vec2, prelude::*};
use bevy_egui::{
    egui::{self, Color32},
    EguiContext,
};

use crate::{
    pixel::Pixel,
    pixel_buffer::{Fill, FillKind, PixelBuffer, PixelBufferSize},
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
    pixel_buffer: Query<(Entity, &PixelBuffer, &Handle<Image>), Added<Handle<Image>>>,
) {
    for (entity, pb, image_handle) in pixel_buffer.iter() {
        let texture = EguiTexture {
            id: egui_ctx.add_image(image_handle.clone_weak()),
            size: pb.size.egui_texture_size(),
        };
        commands.entity(entity).insert(texture);
    }
}

fn update_egui_texture_size(
    mut pixel_buffer: Query<(&PixelBuffer, &mut EguiTexture), Changed<PixelBuffer>>,
) {
    for (pb, mut texture) in pixel_buffer.iter_mut() {
        texture.size = pb.size.egui_texture_size();
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

impl crate::query::PixelBuffersItem<'_> {
    /// Update the [PixelBuffer::fill] to an egui area
    pub fn update_fill_egui(&mut self, available_size: egui::Vec2) {
        self.pixel_buffer.fill.update_egui(available_size);
    }

    /// Gets the [EguiTexture] component of the query item.
    ///
    /// This is a shorthand for `item.egui_texture.unwrap()`. The `unwrap` will
    /// only panic if the [PixelBufferEguiPlugin] is not enabled.
    ///
    /// # Panics
    /// If the entity did not have an [EguiTexture] component because the
    /// [PixelBufferEguiPlugin] was not added to the app.
    pub fn egui_texture(&self) -> &EguiTexture {
        self.egui_texture.unwrap()
    }
}

impl crate::query::PixelBuffersReadOnlyItem<'_> {
    /// Gets the [EguiTexture] component of the query item.
    ///
    /// This is a shorthand for `item.egui_texture.unwrap()`. The `unwrap` will
    /// only panic if the [PixelBufferEguiPlugin] is not enabled.
    ///
    /// # Panics
    /// If the entity did not have an [EguiTexture] component because the
    /// [PixelBufferEguiPlugin] was not added to the app.
    pub fn egui_texture(&self) -> &EguiTexture {
        self.egui_texture.unwrap()
    }
}

impl<'w, 's> crate::query::QueryPixelBuffer<'w, 's> {
    /// Update the [PixelBuffer::fill] to an egui area
    ///
    /// # Panics
    /// If there are none or more than one pixel buffers. This method is
    /// intented to be used when there's only one pixel buffer.
    pub fn update_fill_egui(&mut self, available_size: egui::Vec2) {
        self.query
            .single_mut()
            .pixel_buffer
            .fill
            .update_egui(available_size);
    }

    /// Get the [EguiTexture] component.
    ///
    /// # Panics
    /// - If there are none or more than one pixel buffers. This method is
    /// intented to be used when there's only one pixel buffer.
    ///
    /// - If the entity did not have an [EguiTexture] component because the
    /// [PixelBufferEguiPlugin] was not added to the app.
    pub fn egui_texture(&self) -> &EguiTexture {
        self.query.single().egui_texture.unwrap()
    }
}
