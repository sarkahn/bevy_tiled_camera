use bevy::render::camera::VisibleEntities;
use bevy::prelude::*;
use bevy::render::{
    camera::{self, Camera},
};

pub mod projection;

pub use projection::TiledProjection;

/// A simple camera for properly displaying low resolution pixel perfect 2D games.
///
/// ## Example
/// ```
/// use bevy_tiled_camera::TiledCameraBundle;
/// use bevy::prelude::Commands;
/// fn setup(mut commands:Commands) {
///   // Sets up a camera to display 80 x 25 tiles. The viewport will be scaled up
///   // as much as possible to fit the window size and maintain the appearance of
///   // 8 pixels per tile.
///   let camera_bundle = TiledCameraBundle::new()
///       .with_pixels_per_tile(8)
///       .with_tile_count((80,25).into()
///   );
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

    pub fn with_pixels_per_tile(mut self, pixels_per_tile: u32) -> Self {
        self.projection.pixels_per_tile = pixels_per_tile;
        self
    }

    pub fn with_tile_count(mut self, tile_count: UVec2) -> Self {
        self.projection.target_tile_count = tile_count;
        self
    }

    pub fn with_camera_position(mut self, position: Vec2) -> Self {
        let old_pos = self.transform.translation;
        self.transform.translation = position.extend(old_pos.z);
        self
    }

    /// Will the camera projection be centered or not? Defaults to true
    pub fn with_centered(mut self, centered: bool) -> Self {
        self.projection.centered = centered;
        self
    }
    
    /// Camera will be scaled to be as close as possible to the given target resolution given
    /// pixels per tile.
    pub fn with_target_resolution(self, pixels_per_tile: u32, resolution: UVec2) -> Self {
        self.with_pixels_per_tile(pixels_per_tile)
            .with_tile_count(resolution / pixels_per_tile)
    }
}

impl Default for TiledCameraBundle {
    fn default() -> Self {
        let name = Some(bevy::render::render_graph::base::camera::CAMERA_2D.to_string());
        TiledCameraBundle {
            camera: Camera {
                name,
                ..Default::default()
            },
            projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

pub struct TiledCameraPlugin;
impl Plugin for TiledCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            camera::camera_system::<TiledProjection>.system(),
        );
    }
}