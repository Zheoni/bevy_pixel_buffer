//! Adds utility queries
//!
//! These are common queries and resources grouped up, so everything
//! done here can be replicated with a normal query. You may not
//! need or want to use them but for quick prototyping they are useful to
//! have and not pollute your systems with many and/or complex types.
//!
//! [PixelBuffers] is a [WorldQuery] intented for more than one pixel buffer.
//!
//! [QueryPixelBuffer] is a [SystemParam] that groups the [PixelBuffers] query and
//! the [image](Image) [assets](Assets) resource. It has some convenience methods
//! when working with a single pixel buffer.
//!
//! # Examples
//!
//! For many pixel buffers
//! ```
//! # use bevy::prelude::*;
//! # use bevy_pixel_buffer::prelude::*;
//! fn example_system(mut images: ResMut<Assets<Image>>, pixel_buffers: Query<PixelBuffers>) {
//!     for item in pixel_buffers.iter() {
//!         item.frame(&mut images).per_pixel(|_, _| Pixel::random())
//!     }
//! }
//! # bevy::ecs::system::assert_is_system(example_system);
//! ```
//! Is equivalent to
//! ```
//! # use bevy::prelude::*;
//! # use bevy_pixel_buffer::prelude::*;
//! fn example_system(pixel_buffers: QueryPixelBuffer) {
//!     let (query, mut images) = pixel_buffers.split();
//!     for item in query.iter() {
//!         item.frame(&mut images).per_pixel(|_, _| Pixel::random())
//!     }
//! }
//! # bevy::ecs::system::assert_is_system(example_system);
//! ```
//! ---
//! For a single pixel buffer
//!
//! ```
//! # use bevy::prelude::*;
//! # use bevy_pixel_buffer::prelude::*;
//! fn example_system(mut pb: QueryPixelBuffer) {
//!     pb.frame().per_pixel(|_, _| Pixel::random());
//! }
//! # bevy::ecs::system::assert_is_system(example_system);
//! ```

use std::ops::{Deref, DerefMut};

use bevy::{
    ecs::{query::WorldQuery, system::SystemParam},
    prelude::*,
};

use crate::{
    frame::{Frame, GetFrame, GetFrameFromHandle},
    pixel_buffer::PixelBuffer,
};

// #[derive(WorldQuery)] generates structs without documentation, put them inside
// here to allow that
mod queries {
    #![allow(missing_docs)]

    use super::*;
    // cannot use #[cfg(feature = "egui")] inside the derive

    #[cfg(not(feature = "egui"))]
    /// Query to get the pixel buffers
    ///
    /// See [module documentation](crate::query).
    #[derive(WorldQuery)]
    #[world_query(mutable, derive(Debug))]
    pub struct PixelBuffers {
        /// [Entity] of the pixel buffer
        pub entity: Entity,
        /// [PixelBuffer] component
        pub pixel_buffer: &'static mut PixelBuffer,
        /// Image handle
        pub image_handle: &'static Handle<Image>,
    }

    #[cfg(feature = "egui")]
    /// Query to get the pixel buffers.
    ///
    /// See [module documentation](crate::query).
    #[derive(WorldQuery)]
    #[world_query(mutable, derive(Debug))]
    pub struct PixelBuffers {
        /// [Entity] of the pixel buffer
        pub entity: Entity,
        /// [PixelBuffer] component
        pub pixel_buffer: &'static mut PixelBuffer,
        /// Image handle
        pub image_handle: &'static Handle<Image>,
        /// [EguiTexture](crate::egui::EguiTexture) component.
        ///
        /// Only available with the `egui` feature.
        ///
        /// If the [PixelBufferEguiPlugin](crate::egui::PixelBufferEguiPlugin) is added
        /// it will always be [Some].
        pub egui_texture: Option<&'static crate::egui::EguiTexture>,
    }
}

pub use queries::*;

impl<'w> GetFrameFromHandle for PixelBuffersReadOnlyItem<'w> {
    fn image_handle(&self) -> &Handle<Image> {
        self.image_handle
    }
}

impl<'w> GetFrameFromHandle for PixelBuffersItem<'w> {
    fn image_handle(&self) -> &Handle<Image> {
        self.image_handle
    }
}

/// System parameter to use in systems
#[derive(SystemParam)]
pub struct QueryPixelBuffer<'w, 's> {
    pub(crate) query: Query<'w, 's, PixelBuffers>,
    pub(crate) images: ResMut<'w, Assets<Image>>,
}

impl<'w, 's> Deref for QueryPixelBuffer<'w, 's> {
    type Target = Query<'w, 's, PixelBuffers>;

    fn deref(&self) -> &Self::Target {
        &self.query
    }
}

impl<'w, 's> DerefMut for QueryPixelBuffer<'w, 's> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.query
    }
}

// Zheoni: Help, I can't make a way to iterate over Frame s... lifetimes
//   and so many other problems :(

impl<'w, 's> QueryPixelBuffer<'w, 's> {
    /// Get the image assets resource.
    pub fn images(&mut self) -> &mut Assets<Image> {
        &mut self.images
    }

    /// Gets the query and images resource
    pub fn split(self) -> (Query<'w, 's, PixelBuffers>, ResMut<'w, Assets<Image>>) {
        (self.query, self.images)
    }
}

impl<'w, 's> GetFrame for QueryPixelBuffer<'w, 's> {
    fn frame(&mut self) -> Frame<'_> {
        let image_handle = self.query.single().image_handle;
        Frame::extract(&mut self.images, image_handle)
    }
}
