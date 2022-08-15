//! Adds utility types to use as systems parameters
//!
//! These are some common queries and resources grouped up, so everything
//! done here can be replicated with custom queries. You may not
//! need or want to use them but for quick prototyping they are usefull to
//! have and not pollute your systems with many and/or complex types.

use bevy::{ecs::query::QuerySingleError, prelude::*};

use crate::prelude::{Frame, PixelBuffer};

/// Premade query to work with a pixel buffer
pub type QueryPixelBuffer<'w, 's, 'a> = (
    ResMut<'w, Assets<Image>>,
    Query<'w, 's, (Entity, &'a Handle<Image>), With<PixelBuffer>>,
);

/// Implemeted by premade queries to get a frame to edit the pixel buffer
pub trait GetFrame {
    /// Get a frame to mutate the pixel buffer
    fn frame(&mut self) -> Frame<'_>;
}

impl GetFrame for QueryPixelBuffer<'_, '_, '_> {
    fn frame(&mut self) -> Frame<'_> {
        let (_, image_handle) = single(self.1.get_single());
        Frame::extract(&mut self.0, image_handle)
    }
}

/// Implemented bu premade queries to get the entity of the pixel buffer
pub trait GetEntity {
    /// Gets the entity from the query
    fn entity(&self) -> Entity;
}

impl GetEntity for QueryPixelBuffer<'_, '_, '_> {
    fn entity(&self) -> Entity {
        single(self.1.get_single()).0
    }
}

#[cfg(feature = "egui")]
mod egui_queries {
    use crate::egui::EguiTexture;
    use bevy_egui::egui;

    use super::*;

    /// Premade query to work with a pixel buffer displayed in egui
    pub type QueryPixelBufferEgui<'w, 's, 'a> = (
        ResMut<'w, Assets<Image>>,
        Query<
            'w,
            's,
            (
                Entity,
                &'a Handle<Image>,
                &'a EguiTexture,
                &'a mut PixelBuffer,
            ),
            With<PixelBuffer>,
        >,
    );

    impl GetFrame for QueryPixelBufferEgui<'_, '_, '_> {
        fn frame(&mut self) -> Frame<'_> {
            let q = single(self.1.get_single_mut());
            Frame::extract(&mut self.0, q.1)
        }
    }

    impl GetEntity for QueryPixelBufferEgui<'_, '_, '_> {
        fn entity(&self) -> Entity {
            single(self.1.get_single()).0
        }
    }

    /// Implemented by premade queries to work with a pixel buffer displayed in egui
    pub trait GetEgui {
        /// Extracts the egui texture from the query
        fn egui_texture(&mut self) -> EguiTexture;

        /// Sets the [Fill](crate::pixel_buffer::Fill) to an area given by egui
        fn update_fill_egui(&mut self, available_saize: egui::Vec2);
    }

    impl GetEgui for QueryPixelBufferEgui<'_, '_, '_> {
        fn egui_texture(&mut self) -> EguiTexture {
            *single(self.1.get_single()).2
        }

        fn update_fill_egui(&mut self, available_size: egui::Vec2) {
            if available_size.x == 0.0 || available_size.y == 0.0 {
                info!("Skipping egui fill update, widht or height are 0");
                return;
            }
            let mut pb = single(self.1.get_single_mut()).3;
            pb.fill.update_egui(available_size);
        }
    }
}

#[cfg(feature = "egui")]
pub use egui_queries::*;

fn single<T>(v: Result<T, QuerySingleError>) -> T {
    match v {
        Ok(r) => r,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Cannot use premade query when there are multiple pixel buffers.")
        }
        Err(QuerySingleError::NoEntities(_)) => {
            panic!("No pixel buffer found by premade query.")
        }
    }
}
