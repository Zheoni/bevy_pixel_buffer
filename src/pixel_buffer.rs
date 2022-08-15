//! Core systems and components of the pixel buffer library

use bevy::{
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureUsages},
        texture::ImageSampler,
    },
};

use crate::prelude::Pixel;

/// Component defining a pixel buffer.
///
/// An [image handle](Handle<Image>) component is also
/// needed for most operations, but can be added later.
#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct PixelBuffer {
    /// Size of the pixel buffer
    pub size: PixelBufferSize,
    /// Fill mode
    pub fill: Fill,
}

/// Size of a pixel buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelBufferSize {
    /// Number of (editable) pixels in each dimension.
    pub size: UVec2,
    /// Number of physical pixels each editable pixel takes up in the screen.
    pub pixel_size: UVec2,
}

/// Fill behaviour of the pixel buffer, resizing it automatically
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fill {
    pub(crate) kind: FillKind,
    pub(crate) stretch: bool,
    pub(crate) multiple: u32,
}

/// What to fill
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FillKind {
    /// Fill disabled
    None,
    /// Fill the window
    Window,
    /// Fill a customs size
    Custom(Vec2),
}

impl Default for Fill {
    fn default() -> Self {
        Self {
            kind: FillKind::None,
            stretch: false,
            multiple: 1,
        }
    }
}

impl Fill {
    /// Fill disabled
    pub fn none() -> Self {
        Self {
            kind: FillKind::None,
            ..Default::default()
        }
    }

    /// Fill the window
    pub fn window() -> Self {
        Self {
            kind: FillKind::Window,
            ..Default::default()
        }
    }

    /// Fill a custom size
    pub fn custom(s: impl Into<Vec2>) -> Self {
        Self {
            kind: FillKind::Custom(s.into()),
            ..Default::default()
        }
    }

    /// Wether to stretch the rendering sprite to fill the area
    pub fn with_stretch(mut self, stretch: bool) -> Self {
        self.stretch = stretch;
        self
    }

    /// Keep the size of the buffer a multiple of a value.
    ///
    /// Usefull for [ComputeShader](crate::compute_shader::ComputeShader)
    pub fn with_scaling_multiple(mut self, multiple: u32) -> Self {
        self.multiple = multiple;
        self
    }
}

impl From<FillKind> for Fill {
    fn from(f: FillKind) -> Self {
        Self {
            kind: f,
            ..Default::default()
        }
    }
}

impl From<(u32, u32)> for PixelBufferSize {
    fn from(v: (u32, u32)) -> Self {
        Self {
            size: v.into(),
            ..Default::default()
        }
    }
}

impl From<((u32, u32), (u32, u32))> for PixelBufferSize {
    fn from((size, pixel_size): ((u32, u32), (u32, u32))) -> Self {
        Self {
            size: size.into(),
            pixel_size: pixel_size.into(),
        }
    }
}

impl PixelBufferSize {
    /// New default size.
    ///
    /// - size: `32x32`
    /// - pixel_size: `1x1`
    pub fn new() -> Self {
        Self {
            size: UVec2::new(32, 32),
            pixel_size: UVec2::ONE,
        }
    }

    /// New with a custom size but default pixel_size
    pub fn size(size: impl Into<UVec2>) -> Self {
        Self {
            size: size.into(),
            ..Default::default()
        }
    }

    /// New with a custom pixel_size but default size.
    ///
    /// Usefull combined with [Fill] as the size will be dynamically changed.
    pub fn pixel_size(pixel_size: impl Into<UVec2>) -> Self {
        Self {
            pixel_size: pixel_size.into(),
            ..Default::default()
        }
    }

    /// Returns how many physical pixels are necessary to draw the buffer.
    pub fn screen_size(&self) -> UVec2 {
        self.size * self.pixel_size
    }
}

impl Default for PixelBufferSize {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a compatible [Image] with the pixel buffer, adds it to the app assets and returns a handle
///
/// The image data is set to 0.
pub fn create_image(images: &mut Assets<Image>, size: UVec2) -> Handle<Image> {
    let usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING; // for compute shaders, maybe allow to toggle this last one off
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: Pixel::FORMAT,
            usage,
        },
        data: vec![],
        sampler_descriptor: ImageSampler::nearest(),
        texture_view_descriptor: None,
    };
    image.resize(image.texture_descriptor.size); // set image data to 0
    images.add(image)
}

/// [Plugin] that needs to be added to the app.
pub struct PixelBufferPlugin;

impl Plugin for PixelBufferPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, fill)
            .add_system_to_stage(CoreStage::PreUpdate, resize.after(fill))
            .add_system_to_stage(CoreStage::PreUpdate, sprite_custom_size.after(fill));
    }
}

/// Keeps the size in [PixelBuffer] in sync with the size of the underlying image.
#[allow(clippy::type_complexity)]
fn resize(
    pixel_buffer: Query<
        (&Handle<Image>, &PixelBuffer),
        Or<(Changed<PixelBuffer>, Added<Handle<Image>>)>,
    >,
    mut images: ResMut<Assets<Image>>,
) {
    for (image, pb) in pixel_buffer.iter() {
        let PixelBuffer { size, .. } = pb;

        if size.size.x == 0 || size.size.y == 0 || size.pixel_size.x == 0 || size.pixel_size.y == 0
        {
            warn!("Skipping resize, with and/or height are 0");
            return;
        }

        let image = images.get_mut(image).expect("pixel buffer image");
        if size.size != image.size().as_uvec2() {
            info!("Resizing image to: {:?}", size);
            image.resize(Extent3d {
                width: size.size.x,
                height: size.size.y,
                depth_or_array_layers: 1,
            });
        }
    }
}

/// Changes the size of the pixel buffer to match the fill
fn fill(mut pixel_buffer: Query<&mut PixelBuffer>, windows: Res<Windows>) {
    for mut pb in pixel_buffer.iter_mut() {
        if let Some(fill_area) = get_fill_area(&pb, &windows) {
            let PixelBuffer { size, fill } = &mut *pb;

            let new_buffer_size = fill_area.as_uvec2() / size.pixel_size;
            // Truncate to the fill multiple
            let new_buffer_size = (new_buffer_size / fill.multiple) * fill.multiple;

            if new_buffer_size != size.size {
                size.size = new_buffer_size;
            }
        }
    }
}

/// Changes the sprite custom size
fn sprite_custom_size(
    mut pixel_buffer: Query<(&PixelBuffer, &mut Sprite), Or<(Changed<PixelBuffer>, Added<Sprite>)>>,
    windows: Res<Windows>,
) {
    for (pb, mut sprite) in pixel_buffer.iter_mut() {
        let mut new_size = pb.size.screen_size().as_vec2();

        // if the sprite needs to stretch
        if pb.fill.stretch {
            // set its size to the fill area
            if let Some(fill_area) = get_fill_area(pb, &windows) {
                new_size = fill_area;
            }
        }

        info!("Resizing sprite to: {:?}", new_size);
        sprite.custom_size = Some(new_size);
    }
}

fn get_fill_area(pb: &PixelBuffer, windows: &Windows) -> Option<Vec2> {
    match pb.fill.kind {
        FillKind::None => None,
        FillKind::Window => windows
            .get_primary()
            .map(|window| Vec2::new(window.width(), window.height())),
        FillKind::Custom(custom_size) => Some(custom_size),
    }
}
