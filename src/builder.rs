//! Utilities for constructing pixel buffers.
//!
//! This adds ergonomic ways to call [spawn_pixel_buffer](crate::pixel_buffer::spawn_pixel_buffer).

use crate::pixel_buffer::{
    spawn_pixel_buffer, Fill, PixelBufferCommands, PixelBufferSize, RenderConfig,
};
use bevy::prelude::*;

/// Helper type to allow easy [RenderConfig] conversions inside the [PixelBufferBuilder].
///
/// It only wraps an [Option<RenderConfig>].
///
/// Converting from [RenderConfig] value will wrap it in [Some].
/// ```
/// # use bevy_pixel_buffer::builder::RenderConfigBuilder;
/// # use bevy_pixel_buffer::pixel_buffer::RenderConfig;
/// let value: RenderConfigBuilder = RenderConfig::sprite_only().into();
/// assert_eq!(value, RenderConfigBuilder(Some(RenderConfig::sprite_only())));
/// ```
///
/// From [bool], `true` is a "full" render setup and `false` is no setup at all:
/// ```
/// # use bevy_pixel_buffer::builder::RenderConfigBuilder;
/// # use bevy_pixel_buffer::pixel_buffer::RenderConfig;
/// let value: RenderConfigBuilder = true.into();
/// assert_eq!(value, RenderConfig::sprite_and_camera().into());
/// ```
/// ```
/// # use bevy_pixel_buffer::builder::RenderConfigBuilder;
/// let value: RenderConfigBuilder = false.into();
/// assert_eq!(value, None.into());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderConfigBuilder(pub Option<RenderConfig>);

impl Default for RenderConfigBuilder {
    fn default() -> Self {
        Self(Some(RenderConfig::sprite_and_camera()))
    }
}

impl From<bool> for RenderConfigBuilder {
    fn from(v: bool) -> Self {
        Self(if v {
            Some(RenderConfig::AsSprite { spawn_camera: true })
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

/// Helper type to spawn pixel buffers.
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
#[derive(Debug, Clone, Copy, PartialEq)]
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

    /// Spawns a new pixel buffer with the builder's configuration.
    pub fn spawn<'w, 's, 'a>(
        self,
        commands: &'a mut Commands<'w, 's>,
        images: &'a mut Assets<Image>,
    ) -> PixelBufferCommands<'w, 's, 'a> {
        spawn_pixel_buffer(commands, images, self.size, self.fill, self.render)
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
    pub fn setup(self) -> impl Fn(Commands, ResMut<Assets<Image>>) {
        move |mut commands, mut images| {
            self.spawn(&mut commands, &mut images);
        }
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
