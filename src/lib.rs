//! A simple camera for properly displaying low resolution pixel perfect 2D games in bevy.
//!
//! This camera will scale up the viewport as much as possible while mainting your target
//! resolution and avoiding pixel artifacts.
//!
//! ## Example
//! ```no_run
//! use bevy_tiled_camera::*;
//! use bevy::prelude::*;
//!
//! fn setup(mut commands:Commands) {
//!   // Sets up a camera to display 80 x 25 tiles. The viewport will be scaled up
//!   // as much as possible to fit the window size and maintain the appearance of
//!   // 8 pixels per tile.
//!   let camera_bundle = TiledCameraBundle::new()
//!       .with_pixels_per_tile(8)
//!       .with_tile_count((80,25));
//!
//!   commands.spawn_bundle(camera_bundle);
//! }
//!
//! fn main() {
//!     App::build()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugin(TiledCameraPlugin)
//!     .add_startup_system(setup.system())
//!     .run();
//! }
//! ```
//!
//! Note this is only half the work needed to avoid artifacts with low resolution pixel art.
//! You also need to ensure the camera position and your sprite edges are aligned to the
//! pixel grid.
//!
//! *There is currently a bug in bevy version 0.5 where the camera projection
//! does not update unless you actively resize the window.*

use bevy::prelude::*;
use bevy::render::camera::VisibleEntities;
use bevy::render::camera::{self, Camera};

mod projection;

pub use projection::TiledProjection;

/// Component bundle with functions to specify how you want the camera set up.
///
/// ## Example
/// ```
/// use bevy_tiled_camera::TiledCameraBundle;
/// use bevy::prelude::Commands;
/// fn setup(mut commands:Commands) {
///   let camera_bundle = TiledCameraBundle::new()
///       .with_pixels_per_tile(8)
///       .with_tile_count((80,25))
///       .with_centered(false)
///       .with_camera_position((5.0,5.0));
///
///   commands.spawn_bundle(camera_bundle);
/// }
/// ```
#[derive(Bundle)]
pub struct TiledCameraBundle {
    pub camera: Camera,
    pub projection: TiledProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl TiledCameraBundle {
    pub fn new() -> Self {
        TiledCameraBundle::default()
    }

    /// Sets up the projection to display the given number of pixels per tile.
    pub fn with_pixels_per_tile(mut self, pixels_per_tile: u32) -> Self {
        self.projection.pixels_per_tile = pixels_per_tile;
        self
    }

    /// Sets the projection to display the given tile count.
    pub fn with_tile_count(mut self, tile_count: (u32, u32)) -> Self {
        self.projection.target_tile_count = UVec2::from(tile_count);
        self
    }

    /// Sets the camera position on spawn.
    pub fn with_camera_position(mut self, position: (f32, f32)) -> Self {
        let position = Vec2::from(position);
        let old_pos = self.transform.translation;
        self.transform.translation = position.extend(old_pos.z);
        self
    }

    /// Will the camera projection be centered or not? Defaults to true.
    pub fn with_centered(mut self, centered: bool) -> Self {
        self.projection.centered = centered;
        self
    }

    /// Camera will be scaled to be as close as possible to the given target resolution given
    /// pixels per tile.
    pub fn with_target_resolution(self, pixels_per_tile: u32, resolution: (u32, u32)) -> Self {
        let resolution = UVec2::from(resolution);
        self.with_pixels_per_tile(pixels_per_tile)
            .with_tile_count((resolution / pixels_per_tile).into())
    }
}

impl Default for TiledCameraBundle {
    fn default() -> Self {
        let name = Some(bevy::render::render_graph::base::camera::CAMERA_2D.to_string());
        let projection = TiledProjection::default();
        let transform = Transform::from_xyz(0.0, 0.0, projection.far - 0.1);
        TiledCameraBundle {
            camera: Camera {
                name,
                ..Default::default()
            },
            projection,
            transform,
            visible_entities: Default::default(),
            global_transform: Default::default(),
        }
    }
}

/// Plugin which initializes the camera update function.
pub struct TiledCameraPlugin;
impl Plugin for TiledCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            camera::camera_system::<TiledProjection>.system(),
        );
    }
}
