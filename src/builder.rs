//! Utilities for constructing pixel buffers.
//!
//! This adds ergonomic ways to create and render a pixel buffer.
//! Alternatively [Bundle]s in [crate::bundle] can be used.

use std::fmt::Debug;

use crate::{
    bundle::PixelBufferBundle,
    pixel_buffer::{create_image, Fill, PixelBuffer, PixelBufferSize},
    prelude::{Frame, FrameEditExtension, GetFrame},
};
use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Anchor};

/// Render setup configuration
#[derive(Clone, Debug)]
pub enum RenderConfig {
    /// Set up a sprite and an optional 2D camera
    Sprite {
        /// Spawn a 2D camera
        spawn_camera: bool,
        /// Custom sprite bundle parameters.
        ///
        /// Different from [SpriteBundle] because [SpriteBundle] some extra fields that are not customisable.
        sprite_bundle: CustomSpriteBundle,
    },
}

/// Customisable params for the sprite bundle that will be rendered by [RenderConfig].
///
/// See [SpriteBundle] for docs.
#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct CustomSpriteBundle {
    pub sprite: CustomSprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

/// Customisable params for the sprite that will be rendered by [RenderConfig].
///
/// See [Sprite] for docs.
#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct CustomSprite {
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
    pub anchor: Anchor,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self::sprite()
    }
}

impl RenderConfig {
    /// Set up a 2D camera and a sprite
    pub fn sprite_and_camera() -> Self {
        Self::Sprite {
            spawn_camera: true,
            sprite_bundle: Default::default(),
        }
    }

    /// Set up the sprite.
    ///
    /// A camera also needs to be spawned to see the sprite.
    pub fn sprite() -> Self {
        Self::Sprite {
            spawn_camera: false,
            sprite_bundle: Default::default(),
        }
    }
}

/// Helper type to allow easy [RenderConfig] conversions inside the [PixelBufferBuilder].
///
/// It only wraps an [Option<RenderConfig>].
///
/// From a [RenderConfig] value, it will wrap it in [Some].
///
/// From [bool]:
/// - `true` is a "full" render setup
/// - `false` is no render at all
#[derive(Debug, Clone)]
pub struct RenderConfigBuilder(pub Option<RenderConfig>);

impl Default for RenderConfigBuilder {
    fn default() -> Self {
        Self(Some(RenderConfig::sprite_and_camera()))
    }
}

impl From<bool> for RenderConfigBuilder {
    fn from(v: bool) -> Self {
        Self(if v {
            Some(RenderConfig::sprite_and_camera())
        } else {
            None
        })
    }
}

impl From<Option<RenderConfig>> for RenderConfigBuilder {
    fn from(v: Option<RenderConfig>) -> Self {
        Self(v)
    }
}

impl From<RenderConfig> for RenderConfigBuilder {
    fn from(v: RenderConfig) -> Self {
        Self(Some(v))
    }
}

/// Helper type to create pixel buffers.
///
/// # Example
/// This system spawns a pixel buffer with a custom size.
/// ```
/// # use bevy::prelude::*;
/// # use bevy_pixel_buffer::builder::PixelBufferBuilder;
/// fn setup_pb(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
///     PixelBufferBuilder::new()
///         .with_size((400, 200))
///         .spawn(&mut commands, &mut images);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PixelBufferBuilder {
    /// Custom size
    pub size: PixelBufferSize,
    /// Fill behaviour
    pub fill: Fill,
    /// Set up rendering
    pub render: Option<RenderConfig>,
}

impl Default for PixelBufferBuilder {
    fn default() -> Self {
        Self {
            size: Default::default(),
            fill: Default::default(),
            render: Some(RenderConfig::sprite_and_camera()),
        }
    }
}

impl PixelBufferBuilder {
    /// Creates a new builder. All fields are initialized with their respective [Default::default] value except
    /// render that defaults to a complete renderer setup with a 2D camera and a sprite.
    pub fn new() -> Self {
        Self::default()
    }

    /// Use a custom size for the buffer. The size can be given in many ways, as it's exemplified in [PixelBufferSize].
    pub fn with_size(mut self, size: impl Into<PixelBufferSize>) -> Self {
        self.size = size.into();
        self
    }

    /// Set the [Fill] mode.
    pub fn with_fill(mut self, fill: impl Into<Fill>) -> Self {
        self.fill = fill.into();
        self
    }

    /// Set wether and how to render the pixel buffer using the bevy 2D renderer.
    ///
    /// The type [RenderConfigBuilder] allows for some ergnomics to build the [RenderConfig].
    pub fn with_render(mut self, render: impl Into<RenderConfigBuilder>) -> Self {
        self.render = render.into().0;
        self
    }

    /// Spawns a new entity and inserts a pixel buffer with the builder's configuration to it.
    pub fn spawn<'w, 's, 'a>(
        self,
        commands: &'a mut Commands<'w, 's>,
        images: &'a mut Assets<Image>,
    ) -> PixelBufferCommands<'w, 's, 'a> {
        let entity = commands.spawn();
        create_pixel_buffer(entity, images, self.size, self.fill, self.render)
    }

    /// Inserts a new pixel buffer with the builder's configuration into an existing entity.
    pub fn insert<'w, 's, 'a>(
        self,
        commands: &'a mut Commands<'w, 's>,
        images: &'a mut Assets<Image>,
        entity: Entity,
    ) -> PixelBufferCommands<'w, 's, 'a> {
        let entity = commands.entity(entity);
        create_pixel_buffer(entity, images, self.size, self.fill, self.render)
    }

    /// Returns a system that spawns a pixel buffer with the builder's configuration.
    ///
    /// This allows to avoid writing a system just to create a [PixelBufferBuilder]
    /// and [spawn](PixelBufferBuilder::spawn) a pixel buffer.
    ///
    /// The system is the equivalent as the one shown in the [PixelBufferBuilder] example.
    ///
    /// # Example
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use bevy_pixel_buffer::prelude::*;
    /// fn main() {
    ///     App::new()
    ///         .add_plugins(DefaultPlugins)
    ///         .add_plugin(PixelBufferPlugin)
    ///         .add_startup_system(PixelBufferBuilder::new() // <--
    ///             .with_size((400, 200))
    ///             .setup())
    ///         .run()
    /// }
    /// ```
    pub fn setup(self) -> impl FnMut(Commands, ResMut<Assets<Image>>) {
        move |mut commands, mut images| {
            self.clone().spawn(&mut commands, &mut images);
        }
    }
}

fn create_pixel_buffer<'w, 's, 'a>(
    mut entity: EntityCommands<'w, 's, 'a>,
    images: &'a mut Assets<Image>,
    size: PixelBufferSize,
    fill: Fill,
    render: Option<RenderConfig>,
) -> PixelBufferCommands<'w, 's, 'a> {
    let image = images.add(create_image(size.size.into()));

    if let Some(render) = render {
        match render {
            RenderConfig::Sprite {
                spawn_camera,
                sprite_bundle,
            } => {
                // Spawn a 2D camera if needed
                if spawn_camera {
                    entity.commands().spawn_bundle(Camera2dBundle::default());
                }

                // Add a sprite with the image as texture

                // this also adds a image_handle, but just replacing the existing one
                // which is the same handle
                let sprite_bundle = SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(size.screen_size().as_vec2()),
                        color: sprite_bundle.sprite.color,
                        flip_x: sprite_bundle.sprite.flip_x,
                        flip_y: sprite_bundle.sprite.flip_y,
                        anchor: sprite_bundle.sprite.anchor,
                    },
                    texture: image.clone(),
                    transform: sprite_bundle.transform,
                    global_transform: sprite_bundle.global_transform,
                    visibility: sprite_bundle.visibility,
                    computed_visibility: sprite_bundle.computed_visibility,
                };
                entity.insert_bundle(sprite_bundle);
            }
        }
    }

    entity.insert_bundle(PixelBufferBundle {
        pixel_buffer: PixelBuffer { size, fill },
        image: image.clone(),
    });

    PixelBufferCommands {
        images,
        image_handle: image.clone_weak(),
        entity_commands: entity,
    }
}

/// Returns a system that spawns a pixel buffer with the given size.
///
/// # Example
/// This is equivalent to the application in the [PixelBufferBuilder::setup] example.
/// ```no_run
/// # use bevy::prelude::*;
/// # use bevy_pixel_buffer::prelude::*;
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugin(PixelBufferPlugin)
///         .add_startup_system(pixel_buffer_setup((400, 200))) // <--
///         .run()
/// }
/// ```
pub fn pixel_buffer_setup(
    size: impl Into<PixelBufferSize>,
) -> impl Fn(Commands, ResMut<Assets<Image>>) {
    let size = size.into();
    move |mut commands, mut images| {
        PixelBufferBuilder::new()
            .with_size(size)
            .spawn(&mut commands, &mut images);
    }
}

/// Struct returned from creating a pixel buffer with [PixelBufferBuilder]
/// allowing to work with the new buffer.
pub struct PixelBufferCommands<'w, 's, 'a> {
    images: &'a mut Assets<Image>,
    image_handle: Handle<Image>,
    entity_commands: EntityCommands<'w, 's, 'a>,
}

impl<'w, 's, 'a> PixelBufferCommands<'w, 's, 'a> {
    /// Runs a given closure to modify the buffer.
    ///
    /// This is just to allow chaining a call to [FrameEditExtension::edit_frame].
    pub fn edit_frame(&mut self, f: impl Fn(&mut Frame)) -> &mut Self
    where
        Self: FrameEditExtension,
    {
        <Self as FrameEditExtension>::edit_frame(self, f);
        self
    }

    /// Returns a **strong** handle to the underlying image.
    pub fn image(&self) -> Handle<Image> {
        self.image_handle.clone()
    }

    /// Returns a **weak** handle to the underlying image.
    pub fn image_weak(&self) -> Handle<Image> {
        self.image_handle.clone_weak()
    }

    /// Returns the [EntityCommands] struct to work with the buffer entity.
    pub fn entity(&mut self) -> &mut EntityCommands<'w, 's, 'a> {
        &mut self.entity_commands
    }
}

impl<'w, 's, 'a> GetFrame for PixelBufferCommands<'w, 's, 'a> {
    fn frame(&mut self) -> Frame<'_> {
        Frame::extract(self.images, &self.image_handle)
    }
}
