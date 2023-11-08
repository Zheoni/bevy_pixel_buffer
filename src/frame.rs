//! Frame and frame utility functions that helps to draw things on raw image data.

use crate::pixel::Pixel;
use bevy::{prelude::*, render::render_resource::TextureUsages};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

/// Helper structure to edit a pixel buffer
pub struct Frame<'a> {
    /// Raw pixels of the frame
    pixels: &'a mut [Pixel],
    /// Size of the frame
    size: UVec2,
}

impl<'a> Frame<'a> {
    /// Access the pixels directly
    pub fn raw(&self) -> &[Pixel] {
        self.pixels
    }

    /// Access the pixels directly mutable
    pub fn raw_mut(&mut self) -> &mut [Pixel] {
        self.pixels
    }

    /// Gets the frame size
    pub fn size(&self) -> UVec2 {
        self.size
    }

    /// Runs a function once per pixel with 2 parameters:
    /// - The X and Y position, (0, 0) in the top left.
    /// - The current pixel value
    /// The returned value will be the new value for that pixel.
    ///
    /// # Example
    /// ```
    /// # use bevy::math::UVec2;
    /// # use bevy_pixel_buffer::prelude::*;
    /// let mut frame;
    /// // get a frame...
    /// # let mut pixels = vec![Pixel::BLACK; 10*10];
    /// # frame = Frame::from_raw_parts(&mut pixels, UVec2::new(10, 10));
    /// // set all pixels to red
    /// frame.per_pixel(|_, _| Pixel::RED);
    /// assert!(frame.raw().iter().all(|p| *p == Pixel::RED));
    /// ```
    pub fn per_pixel<P: Into<Pixel>>(&mut self, f: impl Fn(UVec2, Pixel) -> P) {
        for (idx, pixel) in self.pixels.iter_mut().enumerate() {
            let idx = idx as u32;
            let pos = UVec2::new(idx % self.size.x, idx / self.size.x);
            *pixel = f(pos, *pixel).into();
        }
    }

    /// Same as [Frame::per_pixel] but uses [rayon] to do it in parallel.
    #[cfg(feature = "rayon")]
    pub fn per_pixel_par<P: Into<Pixel>>(&mut self, f: impl Fn(UVec2, Pixel) -> P + Sync) {
        self.pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, pixel)| {
                let idx = idx as u32;
                let pos = UVec2::new(idx % self.size.x, idx / self.size.x);
                *pixel = f(pos, *pixel).into();
            });
    }

    /// Sets a pixel in the frame
    pub fn set(&mut self, location: impl Into<UVec2>, pixel: impl Into<Pixel>) -> FrameResult {
        let location: UVec2 = location.into();
        self.check_bounds(location)?;

        let index = location.x + location.y * self.size.x;
        self.pixels[index as usize] = pixel.into();

        Ok(())
    }

    fn check_bounds(&self, location: UVec2) -> FrameResult {
        if location.x >= self.size.x || location.y >= self.size.y {
            Err(FrameError::LocationOutOfBounds {
                location,
                size: self.size,
            })
        } else {
            Ok(())
        }
    }
}

/// Result type for some methods of [Frame]
pub type FrameResult = Result<(), FrameError>;

/// Error type for some methods of [Frame]
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum FrameError {
    /// The location is not inside the frame
    #[error(
        "location out of the bounds of the frame (location: {location:?}, frame size: {size:?}"
    )]
    LocationOutOfBounds {
        /// wrong location
        location: UVec2,
        /// frame size
        size: UVec2,
    },
}

impl<'a> Frame<'a> {
    /// Builds a frame from a bevy image
    pub fn get(image: &'a mut Image) -> Self {
        debug_assert_eq!(image.texture_descriptor.format, Pixel::FORMAT);
        debug_assert!(image
            .texture_descriptor
            .usage
            .contains(TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST));
        let size = image.size();
        let pixels = bytemuck::cast_slice_mut(&mut image.data);
        Self { pixels, size }
    }

    /// Builds a frame by extracting a bevy image from the assets.
    pub fn extract(images: &'a mut Assets<Image>, image_handle: &Handle<Image>) -> Self {
        Self::get(
            images
                .get_mut(image_handle)
                .expect("image when building frame"),
        )
    }

    /// Constructs a frame from raw parts
    ///
    /// # Example
    /// ```
    /// # use bevy::math::UVec2;
    /// # use bevy_pixel_buffer::prelude::{Frame, Pixel};
    /// let mut pixels = vec![Pixel::BLACK; 10*10];
    /// let frame = Frame::from_raw_parts(&mut pixels, UVec2::new(10, 10));
    /// ```
    ///
    /// # Panics
    /// If the length of the slice does not correspond with the given size
    pub fn from_raw_parts(pixels: &'a mut [Pixel], size: UVec2) -> Self {
        assert_eq!(pixels.len(), (size.x * size.y) as usize);
        Self { pixels, size }
    }
}

/// Convenience trait to get a [Frame]
pub trait GetFrame {
    /// Get a frame to mutate a pixel buffer
    fn frame(&mut self) -> Frame<'_>;
}

impl GetFrame for Image {
    #[inline(always)]
    fn frame(&mut self) -> Frame<'_> {
        Frame::get(self)
    }
}

/// Convenience trait to get a [Frame] from a [Handle] needs the [image](Image) [assets](Assets).
pub trait GetFrameFromHandle: AsImageHandle {
    /// Get a frame to mutate a pixel buffer
    fn frame<'a>(&self, images: &'a mut Assets<Image>) -> Frame<'a> {
        Frame::extract(images, self.as_image_handle())
    }
}

impl<T: AsImageHandle> GetFrameFromHandle for T {}

/// Convenience trait to get a frame from the [image](Image) [assets](Assets) with a [Handle]
pub trait GetFrameFromImages: AsMut<Assets<Image>> {
    /// Get a frame to mutate a pixel buffer
    fn frame(&mut self, image_handle: impl AsImageHandle) -> Frame<'_> {
        Frame::extract(self.as_mut(), image_handle.as_image_handle())
    }
}

impl<T: AsMut<Assets<Image>>> GetFrameFromImages for T {}

/// Used to get a reference to a [image](Image) [handle](Handle).
///
/// This is a workaround until `impl<T> AsRef<T> for &T` is stabilized.
pub trait AsImageHandle {
    /// Get a image handle from the type
    fn as_image_handle(&self) -> &Handle<Image>;
}

impl AsImageHandle for Handle<Image> {
    fn as_image_handle(&self) -> &Handle<Image> {
        self
    }
}

impl AsImageHandle for &Handle<Image> {
    fn as_image_handle(&self) -> &Handle<Image> {
        self
    }
}

/// Convenience trait to modify elements with a [Frame]
pub trait FrameEditExtension: GetFrame {
    /// Runs a given closure to modify the buffer.
    fn edit_frame(&mut self, f: impl Fn(&mut Frame)) {
        f(&mut self.frame())
    }
}

impl<T: GetFrame> FrameEditExtension for T {}
