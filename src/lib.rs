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

mod grid;
mod rect;
mod sized_grid;

pub mod projection;

pub struct TiledCameraPlugin;

use grid::PositionGrid;
use rect::Rect;

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
    pub tiled_camera: TiledCamera,
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
            tiled_camera: TiledCamera::default(),
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
        self.camera_bundle.tiled_camera.set_tile_count(tile_count.into());
        self
    }
}

impl Default for TiledCameraBuilder {
    fn default() -> Self {
        TiledCameraBuilder::new()
    }
}

#[derive(Default, Component)]
pub struct TiledCamera {
    center_offset: Vec2,
}

impl TiledCamera {
    pub fn new(tile_count: (u32,u32)) -> Self {
        let mut cam = TiledCamera::default();
        cam.set_tile_count(tile_count);
        cam
    }

    pub fn set_tile_count(&mut self, tile_count: (u32,u32)) {
        let tile_count = UVec2::from(tile_count);
        let offset = ( tile_count % 2).cmpeq(UVec2::ZERO);
        let offset = Vec2::select(offset, Vec2::new(0.5,0.5), Vec2::ZERO);
        self.center_offset = offset;
    }

    /// Convert a world position to a tile position.
    pub fn world_to_tile(&self, cam_transform: &GlobalTransform, world_pos: (f32,f32,f32)) -> IVec2 {
        let local = (Vec3::from(world_pos) - cam_transform.translation).truncate() + self.center_offset;
        local.floor().as_ivec2()
    }

    pub fn tile_to_world(&self, cam_transform: &GlobalTransform, tile_pos: (i32,i32)) -> Vec3 {
        let tile = IVec2::from(tile_pos).as_vec2() - self.center_offset;
        cam_transform.translation + tile.extend(0.0)
    }

    pub fn tile_center_world(&self, cam_transform: &GlobalTransform, tile_pos: (i32,i32)) -> Vec3 {
        cam_transform.translation + self.tile_to_world(cam_transform, tile_pos) + self.center_offset.extend(0.0)
    }

    /// Converts a screen position to a world position
    pub fn screen_to_world(
        camera: &Camera,
        windows: &Windows,
        camera_transform: &GlobalTransform,
        screen_pos: Vec2,
    ) -> Option<Vec2> {
        let window = windows.get(camera.window)?;
        let window_size = Vec2::new(window.width(), window.height());
    
        // Convert screen position [0..resolution] to ndc [-1..1]
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    
        Some(world_pos.truncate())
    }

    pub fn world_to_screen(
        &self,
        camera: &Camera,
        windows: &Windows,
        camera_transform: &GlobalTransform,
        world_position: Vec3,
    ) -> Option<Vec2> {
        let window = windows.get(camera.window)?;
        let window_size = Vec2::new(window.width(), window.height());
        // Build a transform to convert from world to NDC using camera data
        let world_to_ndc: Mat4 =
            camera.projection_matrix * camera_transform.compute_matrix().inverse();
        let ndc_space_coords: Vec3 = world_to_ndc.project_point3(world_position);
        // NDC z-values outside of 0 < z < 1 are outside the camera frustum and are thus not in screen space
        if ndc_space_coords.z < 0.0 || ndc_space_coords.z > 1.0 {
            return None;
        }
        // Once in NDC space, we can discard the z element and rescale x/y to fit the screen
        let screen_space_coords = (ndc_space_coords.truncate() + Vec2::ONE) / 2.0 * window_size;
        if !screen_space_coords.is_nan() {
            Some(screen_space_coords)
        } else {
            None
        }
    }
}
