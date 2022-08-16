//! [Bundle]s that can be used to manually create a pixel buffer.
//!
//! As the image of a pixel buffer has to be created with specific parameters,
//! [create_image](crate::pixel_buffer::create_image) should be used to obtain it. Because of
//! this, the usage of [PixelBufferBuilder](crate::builder::PixelBufferBuilder) is recommended but
//! not required.

use crate::pixel_buffer::PixelBuffer;
use bevy::prelude::*;

/// [Bundle] to create a pixel buffer manually.
///
/// Gives the possibility to insert the [Bundle] instead of
/// abstracting it with [PixelBufferBuilder](crate::builder::PixelBufferBuilder).
///
/// The [PixelBufferBundle::image] handle should be obtained from [create_image](crate::pixel_buffer::create_image).
#[derive(Bundle)]
pub struct PixelBufferBundle {
    /// Pixel buffer component.
    pub pixel_buffer: PixelBuffer,
    /// Image handle obtained with [create_image](crate::pixel_buffer::create_image).
    pub image: Handle<Image>,
}

/// Bundle to create a pixel buffer with a sprite manually.
///
/// Gives the possibility to insert the [Bundle] instead of
/// abstracting it with [PixelBufferBuilder](crate::builder::PixelBufferBuilder).
///
///
#[derive(Bundle)]
pub struct PixelBufferSpriteBundle {
    /// Pixel buffer component
    pub pixel_buffer: PixelBuffer,
    /// Sprite bundle to render the pixel buffer.
    ///
    /// [SpriteBundle::texture] should be obtained from [create_image](crate::pixel_buffer::create_image).
    /// [Sprite::custom_size] in [SpriteBundle::sprite] will be ignored. To set a size modify [PixelBuffer::size].
    #[bundle]
    pub sprite_bundle: SpriteBundle,
}
