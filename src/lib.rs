//! A bevy library to manage and draw a bunch of pixels.
//!
//! ## Example
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_pixel_buffer::prelude::*;
//!
//! fn main() {
//!     let size = PixelBufferSize {
//!         size: UVec2::new(32, 32),
//!         pixel_size: UVec2::new(16, 16),
//!     };
//!
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugin(PixelBufferPlugin)
//!         .add_startup_system(pixel_buffer_setup(size))
//!         .add_system(update)
//!         .run()
//! }
//!
//! fn update(mut pb: QueryPixelBuffer) {
//!     pb.frame().per_pixel(|_, _| Pixel::random());
//! }
//! ```

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod builder;
pub mod bundle;
pub mod compute_shader;
#[cfg(feature = "egui")]
pub mod egui;
pub mod frame;
pub mod pixel;
pub mod pixel_buffer;
pub mod query;

pub mod prelude {
    //! Common imports
    pub use crate::builder::{pixel_buffer_setup, PixelBufferBuilder, RenderConfig};
    pub use crate::compute_shader::{ComputeShader, ComputeShaderPlugin};
    #[cfg(feature = "egui")]
    pub use crate::egui::{EguiTexture, PixelBufferEguiPlugin};
    pub use crate::frame::{
        Frame, FrameEditExtension, GetFrame, GetFrameFromHandle, GetFrameFromImages,
    };
    pub use crate::pixel::Pixel;
    pub use crate::pixel_buffer::{
        Fill, FillKind, PixelBuffer, PixelBufferPlugin, PixelBufferPlugins, PixelBufferSize,
    };
    pub use crate::query::*;
}

#[cfg(feature = "egui")]
pub use bevy_egui;
