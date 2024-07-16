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
//!         .add_plugins((DefaultPlugins, PixelBufferPlugin))
//!         .add_systems(Startup, pixel_buffer_setup(size))
//!         .add_systems(Update, update)
//!         .run();
//! }
//!
//! fn update(mut pb: QueryPixelBuffer) {
//!     pb.frame().per_pixel(|_, _| Pixel::random());
//! }
//! ```
//! See other [examples](https://github.com/Zheoni/bevy_pixel_buffer/tree/main/examples).
//!
//! ## Quick how to
//!
//! ### Create a pixel buffer
//! Probably [PixelBufferBuilder](crate::builder::PixelBufferBuilder) is your friend, but if you like the
//! more *bevy like* experience of working with bundles, see the [bundle] module.
//!
//! ### Get the pixel buffer in your systems
//! There are 2 approaches:
//! - Create your custom bevy queries. A pixel buffer is composed of a
//! [PixelBuffer](crate::pixel_buffer::PixelBuffer),
//! [Handle](bevy::asset::Handle)<[Image](bevy::render::prelude::Image)> and optionally a
//! [EguiTexture](crate::egui::EguiTexture) and
//! [Handle](bevy::asset::Handle)<[ComputeShader](crate::compute_shader::ComputeShader)> components.
//! - Use the premade queries in the [query] module. This exist for quick prototyping and common
//! queries related to one or more pixel buffers.
//!
//! ### Modify a pixel buffer
//! The data of the pixels lives inside a bevy [Image](bevy::prelude::Image). To edit it exists the
//! [Frame](crate::frame::Frame) struct. There are many ways to get a [Frame](crate::frame::Frame).
//!
//! Once you have a [Frame](crate::frame::Frame) it offers methods to edit the [Pixel](crate::pixel::Pixel)s.
//! The crate does not offer drawing behaviour (yet) for shapes like triangles, quads or anything like that,
//! but with [Frame::raw_mut](crate::frame::Frame::raw_mut) you can implement any behaviour you want.
//!

#![deny(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

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
