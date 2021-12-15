//! A simple camera for properly displaying low resolution pixel perfect 2D games.
//!
//! ## Example
//! ```
//! // Sets up a camera to display 80 x 25 tiles. The viewport will be scaled up
//! // as much as possible to fit the window size and maintain the appearance of
//! // 8 pixels per tile.
//! let camera_bundle = TiledCameraBuilder::new()
//! .with_pixels_per_tile(8)
//! .with_tile_count((80,25).into())
//! .camera_bundle;
//!
//! commands.spawn(camera_bundle);
//! ```
use bevy::prelude::*;
use bevy::render::{
    camera::{self, Camera, CameraPlugin},
    primitives::Frustum,
    view::VisibleEntities,
};
pub use projection::TiledProjection;

pub mod projection;

pub struct TiledCameraPlugin;

/// Provides a simple way to set initial parameters for the tiled camera.
///
/// # Example
/// ```
/// // Sets up the camera to render 80 tiles by 25 tiles, with each tile
/// // being 8 pixels high.
/// let camera_bundle = TiledCameraBuilder::new()
/// .with_pixels_per_tile(8)
/// .with_tile_count((80,25).into())
/// .camera_bundle;
///
/// commands.spawn_bundle(camera_bundle);
/// ```
pub struct TiledCameraBuilder {
    pub camera_bundle: TiledCameraBundle,
}

#[derive(Bundle)]
pub struct TiledCameraBundle {
    pub camera: Camera,
    pub projection: TiledProjection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Plugin for TiledCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            camera::camera_system::<TiledProjection>,
        );
    }
}

impl Default for TiledCameraBundle {
    fn default() -> Self {
        TiledCameraBundle {
            camera: Camera {
                name: Some(CameraPlugin::CAMERA_2D.to_string()),
                ..Default::default()
            },
            projection: Default::default(),
            visible_entities: Default::default(),
            frustum: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

impl TiledCameraBuilder {
    pub fn new() -> Self {
        TiledCameraBuilder {
            camera_bundle: TiledCameraBundle {
                ..TiledCameraBundle::default()
            },
        }
    }

    pub fn with_tile_settings(self, pixels_per_tile: u32, tile_count: UVec2) -> Self {
        self.with_pixels_per_tile(pixels_per_tile)
            .with_tile_count(tile_count)
    }

    pub fn with_camera_position(mut self, x: f32, y: f32) -> Self {
        self.camera_bundle.transform.translation =
            Vec3::new(x, y, self.camera_bundle.transform.translation.z);
        self
    }

    pub fn with_centered(mut self, centered: bool) -> Self {
        self.camera_bundle.projection.centered = centered;
        self
    }

    /// Determines how much the view can scale, based on tile count, without deforming pixels.
    pub fn with_pixels_per_tile(mut self, pixels_per_tile: u32) -> Self {
        self.camera_bundle.projection.pixels_per_tile = pixels_per_tile;
        self
    }

    /// Camera will be scaled to be as close as possible to the given target resolution given
    /// pixels per tile.
    pub fn with_target_resolution(self, pixels_per_tile: u32, resolution: UVec2) -> Self {
        self.with_pixels_per_tile(pixels_per_tile)
            .with_tile_count(resolution / pixels_per_tile)
    }

    /// Determines how many tiles can fit in the viewport on either axis before it will be rescaled.
    pub fn with_tile_count(mut self, tile_count: UVec2) -> Self {
        self.camera_bundle.projection.target_tile_count = tile_count;
        self
    }
}

impl Default for TiledCameraBuilder {
    fn default() -> Self {
        TiledCameraBuilder::new()
    }
}
