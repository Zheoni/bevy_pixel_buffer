//! Core systems and components of the pixel buffer library

use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureUsages},
        texture::ImageSampler,
    },
    window::PrimaryWindow,
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
    /// Fill a window
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

    /// Fill the primary window
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

/// Parameters for [create_image].
pub struct CreateImageParams {
    /// Size of the image
    pub size: UVec2,
    /// wgpu label
    pub label: Option<&'static str>,
    /// Texture usages
    ///
    /// Has to include:
    /// - [TextureUsages::TEXTURE_BINDING]
    /// - [TextureUsages::COPY_DST]
    /// - [TextureUsages::STORAGE_BINDING]
    pub usage: TextureUsages,
    /// Texture sampler
    ///
    /// For pixelated images the sensible sampler is [ImageSampler::nearest()].
    pub sampler_descriptor: ImageSampler,
}

impl Default for CreateImageParams {
    fn default() -> Self {
        Self {
            size: UVec2 { x: 32, y: 32 },
            label: None,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::STORAGE_BINDING,
            sampler_descriptor: ImageSampler::nearest(),
        }
    }
}

impl From<UVec2> for CreateImageParams {
    fn from(size: UVec2) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }
}

/// Creates a compatible [Image] with the pixel buffer.
///
/// The image needs to be added to the image assets to get a handle.
///
/// The image data is set to 0.
///
/// The wgpu format of the image is [Pixel::FORMAT].
///
/// # Panics
/// - If the size is 0 in either dimension.
/// - If the usages do not contain [TextureUsages::TEXTURE_BINDING],  [TextureUsages::COPY_DST] and [TextureUsages::STORAGE_BINDING].
///
pub fn create_image(params: CreateImageParams) -> Image {
    let CreateImageParams {
        size,
        label,
        usage,
        sampler_descriptor,
    } = params;

    assert_ne!(size.x, 0);
    assert_ne!(size.y, 0);
    assert!(usage.contains(
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING
    ));

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label,
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
            view_formats: &[],
        },
        data: vec![],
        sampler_descriptor,
        texture_view_descriptor: None,
    };
    image.resize(image.texture_descriptor.size); // set image data to 0
    image
}

#[allow(rustdoc::broken_intra_doc_links)]
/// [Plugin group](PluginGroup) that adds the complete `bevy_pixel_buffer`
/// suite of plugins:
/// - [PixelBufferPlugin]
/// - [PixelBufferEguiPlugin](crate::egui::PixelBufferEguiPlugin) *requires `egui` feature*
pub struct PixelBufferPlugins;

impl PluginGroup for PixelBufferPlugins {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();

        let group = group.add(PixelBufferPlugin);
        #[cfg(feature = "egui")]
        let group = group.add(crate::egui::PixelBufferEguiPlugin);

        group
    }
}

/// [Plugin] that needs to be added to the app.
pub struct PixelBufferPlugin;

impl Plugin for PixelBufferPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fill.in_base_set(CoreSet::PreUpdate))
            .add_system(resize.after(fill).in_base_set(CoreSet::PreUpdate))
            .add_system(
                sprite_custom_size
                    .after(fill)
                    .in_base_set(CoreSet::PreUpdate),
            );
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
fn fill(
    mut pixel_buffer: Query<&mut PixelBuffer>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    for mut pb in pixel_buffer.iter_mut() {
        if let Some(fill_area) = get_fill_area(&pb, primary_window.get_single().ok()) {
            let PixelBuffer { size, fill } = pb.as_ref();

            let new_buffer_size = fill_area.as_uvec2() / size.pixel_size;
            // Truncate to the fill multiple
            let new_buffer_size = (new_buffer_size / fill.multiple) * fill.multiple;

            if new_buffer_size != size.size {
                pb.size.size = new_buffer_size;
            }
        }
    }
}

/// Changes the sprite custom size
#[allow(clippy::type_complexity)]
fn sprite_custom_size(
    mut pixel_buffer: Query<(&PixelBuffer, &mut Sprite)>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    for (pb, mut sprite) in pixel_buffer.iter_mut() {
        let mut new_size = pb.size.screen_size().as_vec2();

        // if the sprite needs to stretch
        if pb.fill.stretch {
            // set its size to the fill area
            if let Some(fill_area) = get_fill_area(pb, primary_window.get_single().ok()) {
                new_size = fill_area;
            }
        }

        let new_size = Some(new_size);
        // Make sure to not implicitly deref as mut
        if new_size != sprite.as_ref().custom_size {
            info!("Resizing sprite to: {:?}", new_size);
            sprite.custom_size = new_size;
        }
    }
}

fn get_fill_area(pb: &PixelBuffer, window: Option<&Window>) -> Option<Vec2> {
    match pb.fill.kind {
        FillKind::None => None,
        FillKind::Window => window.map(|window| Vec2::new(window.width(), window.height())),
        FillKind::Custom(custom_size) => Some(custom_size),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle::{PixelBufferBundle, PixelBufferSpriteBundle};

    #[test]
    fn do_resize_image() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins)
            .add_plugin(bevy::asset::AssetPlugin::default())
            .add_plugin(bevy::window::WindowPlugin::default())
            .add_plugin(bevy::render::RenderPlugin::default())
            .add_plugin(bevy::render::texture::ImagePlugin::default());

        app.add_system(resize);

        let initial_size = UVec2::new(5, 5);
        let set_size = UVec2::new(10, 10);

        assert_ne!(initial_size, set_size);

        let mut images = app.world.resource_mut::<Assets<Image>>();
        let image = images.add(create_image(initial_size.into()));

        let pb_id = app
            .world
            .spawn(PixelBufferBundle {
                pixel_buffer: PixelBuffer {
                    size: PixelBufferSize::size(set_size),
                    fill: Fill::none(),
                },
                image,
            })
            .id();

        app.update();

        let set_size = app.world.get::<PixelBuffer>(pb_id).unwrap().size.size;
        let image_handle = app.world.get::<Handle<Image>>(pb_id).unwrap();
        let images = app.world.resource::<Assets<Image>>();
        let image_size = images.get(image_handle).unwrap().size().as_uvec2();

        assert_eq!(set_size, image_size);
    }

    #[test]
    fn do_resize_sprite() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins)
            .add_plugin(bevy::asset::AssetPlugin::default())
            .add_plugin(bevy::window::WindowPlugin::default())
            .add_plugin(bevy::render::RenderPlugin::default())
            .add_plugin(bevy::render::texture::ImagePlugin::default())
            .add_plugin(bevy::core_pipeline::CorePipelinePlugin)
            .add_plugin(bevy::sprite::SpritePlugin);

        app.add_system(sprite_custom_size);

        let set_size = UVec2::new(10, 10);

        let mut images = app.world.resource_mut::<Assets<Image>>();
        let image = images.add(create_image(set_size.into()));

        let pb_id = app
            .world
            .spawn(PixelBufferSpriteBundle {
                pixel_buffer: PixelBuffer {
                    size: PixelBufferSize::size(set_size),
                    fill: Fill::none(),
                },
                sprite_bundle: SpriteBundle {
                    sprite: Sprite {
                        custom_size: None,
                        ..Default::default()
                    },
                    texture: image,
                    ..Default::default()
                },
            })
            .id();

        app.update();

        let size = app.world.get::<PixelBuffer>(pb_id).unwrap().size;
        let sprite = app.world.get::<Sprite>(pb_id).unwrap();

        assert!(sprite.custom_size.is_some());
        assert_eq!(size.screen_size(), sprite.custom_size.unwrap().as_uvec2());
    }

    #[test]
    fn do_fill() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins)
            .add_plugin(bevy::asset::AssetPlugin::default())
            .add_plugin(bevy::window::WindowPlugin::default())
            .add_plugin(bevy::render::RenderPlugin::default())
            .add_plugin(bevy::render::texture::ImagePlugin::default());

        app.add_system(fill);

        let set_size = UVec2::new(5, 5);
        let fill_area = Vec2::new(10.5, 10.4);

        let mut images = app.world.resource_mut::<Assets<Image>>();
        let image = images.add(create_image(set_size.into()));

        let pb_id = app
            .world
            .spawn(PixelBufferBundle {
                pixel_buffer: PixelBuffer {
                    size: PixelBufferSize::size(set_size),
                    fill: Fill::custom(fill_area),
                },
                image,
            })
            .id();

        app.update();

        let size = app.world.get::<PixelBuffer>(pb_id).unwrap().size.size;
        assert_eq!(size, UVec2::new(10, 10));
    }
}
