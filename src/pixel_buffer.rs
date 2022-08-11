//! Core systems and components of the pixel buffer library

use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureUsages},
        texture::ImageSampler,
    },
    window::WindowResized,
};

use crate::prelude::{Frame, Pixel};

/// Marker component for a pixel buffer.
#[derive(Component, Default, Clone, Copy, Debug)]
pub struct PixelBuffer;

/// Size of a pixel buffer.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelBufferSize {
    /// Number of (editable) pixels in each dimension.
    pub size: UVec2,
    /// Number of physical pixels each editable pixel takes up in the screen.
    pub pixel_size: UVec2,
}

/// Fill behaviour of the pixel buffer, resizing it automatically
#[derive(Component, Debug, Clone, Copy, PartialEq)]
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

/// Render setup configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderConfig {
    /// Set up a sprite and an optional 2D camera
    AsSprite {
        /// Spawn a 2D camera
        spawn_camera: bool,
    },
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self::sprite_and_camera()
    }
}

impl RenderConfig {
    /// Set up a 2D camera and a sprite
    pub fn sprite_and_camera() -> Self {
        Self::AsSprite { spawn_camera: true }
    }

    /// Only set up the sprite
    pub fn sprite_only() -> Self {
        Self::AsSprite {
            spawn_camera: false,
        }
    }
}

/// Struct returned from [spawn_pixel_buffer] allowing to work with the new
/// buffer.
pub struct PixelBufferCommands<'w, 's, 'a> {
    images: &'a mut Assets<Image>,
    /// Image handle of the buffer
    pub image_handle: Handle<Image>,
    entity_commands: EntityCommands<'w, 's, 'a>,
}

impl<'w, 's, 'a> PixelBufferCommands<'w, 's, 'a> {
    /// Gets the frame to edit the buffer.
    pub fn get_frame(&mut self) -> Frame<'_> {
        Frame::extract(self.images, &self.image_handle)
    }

    /// Runs a given closure to initialize the buffer.
    pub fn init_frame(&'a mut self, f: impl Fn(&mut Frame)) -> &mut Self {
        f(&mut self.get_frame());
        self
    }

    /// Returns the [EntityCommands] struct to work with the buffer entity.
    pub fn entity(&mut self) -> &mut EntityCommands<'w, 's, 'a> {
        &mut self.entity_commands
    }
}

/// Spawns a new pixel buffer
///
/// Probably you don't need to use this directly but through [crate::builder].
pub fn spawn_pixel_buffer<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    images: &'a mut Assets<Image>,
    size: PixelBufferSize,
    fill: Fill,
    render: Option<RenderConfig>,
) -> PixelBufferCommands<'w, 's, 'a> {
    let usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING; // for compute shaders, maybe allow to toggle this last one off
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: Extent3d {
                width: size.size.x,
                height: size.size.y,
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
    let image_handle = images.add(image);
    let entity = commands
        .spawn_bundle((
            PixelBuffer,
            Name::new("Pixel buffer"),
            size,
            image_handle.clone(),
            fill,
        ))
        .id();

    if let Some(render) = render {
        match render {
            RenderConfig::AsSprite { spawn_camera } => {
                // Spawn a 2D camera if needed
                if spawn_camera {
                    commands.spawn_bundle(Camera2dBundle::default());
                }

                // Add a sprite with the image as texture

                // this also adds a image_handle, but just replacing the existing one
                // which is the same handle
                let sprite_bundle = SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(size.screen_size().as_vec2()),
                        ..Default::default()
                    },
                    texture: image_handle.clone(),
                    ..Default::default()
                };
                commands.entity(entity).insert_bundle(sprite_bundle);
            }
        }
    }

    PixelBufferCommands {
        images,
        image_handle: image_handle.clone_weak(),
        entity_commands: commands.entity(entity),
    }
}

/// [Plugin] that needs to be added to the app.
pub struct PixelBufferPlugin;

impl Plugin for PixelBufferPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, resize)
            .add_system_to_stage(CoreStage::PreUpdate, fill.before(resize));
    }
}

#[allow(clippy::type_complexity)]
fn resize(
    pixel_buffer: Query<
        (&Handle<Image>, &PixelBufferSize),
        (With<PixelBuffer>, Changed<PixelBufferSize>),
    >,
    mut images: ResMut<Assets<Image>>,
) {
    for (image, size) in pixel_buffer.iter() {
        if size.size.x == 0 || size.size.y == 0 || size.pixel_size.x == 0 || size.pixel_size.y == 0
        {
            warn!("Skipping resize, with and/or height are 0");
            return;
        }
        let image = images.get_mut(image).expect("pixel buffer image");
        if size.size == image.size().as_uvec2() {
            // resize not needed
            return;
        }
        info!("Resizing image to: {:?}", size);
        image.resize(Extent3d {
            width: size.size.x,
            height: size.size.y,
            depth_or_array_layers: 1,
        });
    }
}

fn fill(
    mut pixel_buffer: Query<(&mut PixelBufferSize, &Fill, Option<&mut Sprite>), With<PixelBuffer>>,
    mut window_resize: EventReader<WindowResized>,
) {
    for (mut size, fill, sprite) in pixel_buffer.iter_mut() {
        let available_size = match fill.kind {
            FillKind::Window => {
                if let Some(window_size) = window_resize.iter().last() {
                    Vec2::new(window_size.width, window_size.height)
                } else {
                    continue;
                }
            }
            FillKind::Custom(custom_size) => custom_size,
            _ => continue,
        };

        let new_buffer_size =
            ((available_size.as_uvec2() / size.pixel_size) / fill.multiple) * fill.multiple;
        if new_buffer_size != size.size {
            size.size = new_buffer_size;
        }

        if let Some(mut sprite) = sprite {
            let new_sprite_size = sprite_size(&size, available_size, fill.stretch);
            if new_sprite_size != sprite.custom_size {
                info!("Resized sprite: {:?}", new_sprite_size);
                sprite.custom_size = new_sprite_size;
            }
        }
    }
}

fn sprite_size(size: &PixelBufferSize, available_size: Vec2, stretch: bool) -> Option<Vec2> {
    let new_size = if stretch {
        available_size
    } else {
        size.screen_size().as_vec2()
    };
    Some(new_size)
}
