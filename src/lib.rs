//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! [![Crates.io](https://img.shields.io/crates/v/bevy_tiled_camera)](https://crates.io/crates/bevy_tiled_camera)
//! [![docs](https://docs.rs/bevy_tiled_camera/badge.svg)](https://docs.rs/bevy_tiled_camera/)
//!
//! # `Bevy Tiled Camera`
//!
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
//!       .with_tile_count([80,25]);
//!
//!   commands.spawn_bundle(camera_bundle);
//! }
//!
//! fn main() {
//!     App::new()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugin(TiledCameraPlugin)
//!     .add_startup_system(setup)
//!     .run();
//! }
//! ```
//!
//! Note this is only half the work needed to avoid artifacts with low resolution pixel art.
//! You also need to ensure the camera position and your sprite edges are aligned to the
//! pixel grid.
//!
//! You can change the camera settings at any time by adjusting the [TiledProjection](src/projection.rs) component on the camera entity.
//!
//! ## World Space
//! Note that this projection assumes the size of one tile is equal to one world unit. This is different than Bevy's default 2D orthographic camera which assumes one *pixel* is equal to one world unit.
//!
//! ## Versions
//! | bevy | bevy_tiled_camera |
//! | --- | --- |
//! | 0.8 | 0.4.0 |
//! | 0.6 | 0.3.0 |
//! | 0.5 | 0.2.4 |
//! | 0.5 | 0.2.3 |
use bevy::{
    prelude::*, 
    render::{
        texture::{ImageSettings, ImageSampler}, 
        camera::{ScalingMode, Viewport}
    }, 
    window::{
        WindowResized, 
        WindowId
    }, core_pipeline::clear_color::ClearColorConfig
};
use sark_grids::{*, world_grid::{WorldGrid}, point::Point2d};

pub struct TiledCameraPlugin;

impl Plugin for TiledCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ImageSettings {
            default_sampler: ImageSampler::nearest_descriptor(),
        })
        .add_system(on_window_resized)
        .add_system(on_camera_changed)
        ;
    }
}

/// Component bundle with functions to specify how you want the camera set up.
///
/// ## Example
/// ```rust
/// use bevy_tiled_camera::TiledCameraBundle;
/// use bevy::prelude::Commands;
/// fn setup(mut commands:Commands) {
///   let camera_bundle = TiledCameraBundle::new()
///       .with_pixels_per_tile(8)
///       .with_tile_count([80,45]);
///
///   commands.spawn_bundle(camera_bundle);
/// }
/// ```
#[derive(Bundle)]
pub struct TiledCameraBundle {

    #[bundle]
    cam2d_bundle: Camera2dBundle,
    tiled_camera: TiledCamera,
}

impl TiledCameraBundle {
    pub fn new() -> Self {
        TiledCameraBundle::default()
    }

    pub fn unit_cam(tile_count: impl Size2d, pixels_per_tile: u32) -> Self {
        TiledCameraBundle { 
            tiled_camera: TiledCamera::unit_cam(tile_count, pixels_per_tile),
            ..default() 
        }
    }
    

    pub fn pixel_cam(tile_count: impl Size2d, pixels_per_tile: u32) -> Self {
        TiledCameraBundle { 
            tiled_camera: TiledCamera::pixel_cam(tile_count, pixels_per_tile),
            ..default() 
        }
    }

    pub fn with_clear_color(mut self, color: Color) -> Self {
        self.cam2d_bundle.camera_2d.clear_color = ClearColorConfig::Custom(color);
        self
    }

    pub fn with_pixels_per_tile(mut self, ppt: u32) -> Self {
        self.tiled_camera.pixels_per_tile = ppt;
        self
    }

    pub fn with_tile_count(mut self, tile_count: impl Size2d) -> Self {
        self.tiled_camera.tile_count = tile_count.as_uvec2();
        self
    }

    pub fn with_camera_position(mut self, tile_count: impl Size2d) -> Self {
        let pos = &mut self.cam2d_bundle.transform.translation;
        *pos = tile_count.as_vec2().extend(pos.z);
        self
    }
}

impl Default for TiledCameraBundle {
    fn default() -> Self {
        Self { 
            cam2d_bundle: Camera2dBundle {
                ..default()
            },
            tiled_camera: TiledCamera::default(),
         }
    }
}

#[derive(Component)]
pub struct TiledCamera {
    pub pixels_per_tile: u32,
    pub tile_count: UVec2,
    grid: WorldGrid,
}

impl TiledCamera {
    /// Creates a camera set to [`WorldSpace::Units`]. 
    pub fn unit_cam(tile_count: impl Size2d, pixels_per_tile: u32) -> Self {
        let tile_count = tile_count.as_uvec2();
        Self { 
            pixels_per_tile, 
            tile_count, 
            grid: WorldGrid::unit_grid(tile_count, pixels_per_tile)
        }
    }

    /// Creates a camera set to [`WorldSpace::Pixels`].
    pub fn pixel_cam(tile_count: impl Size2d, pixels_per_tile: u32) -> Self {
        let tile_count = tile_count.as_uvec2();
        Self { 
            pixels_per_tile, 
            tile_count, 
            grid: WorldGrid::pixel_grid(tile_count, pixels_per_tile)
        }
    }

    pub fn target_resolution(&self) -> UVec2 {
        self.tile_count * self.pixels_per_tile as u32
    }

    /// Returns an iterator that yields the center of every camera tile in world space.
    pub fn tile_center_iter(&self, transform: &GlobalTransform) -> impl Iterator<Item=Vec2> {
        let xy = transform.translation.truncate();
        self.grid.tile_center_iter().map(move |p| p + xy)
    }

    pub fn world_to_local(&self, transform: &GlobalTransform, world_pos: impl Point2d) -> Vec2 {
        world_pos.as_vec2() - transform.translation.truncate()
    }

    pub fn local_to_world(&self, transform: &GlobalTransform, local_pos: impl Point2d) -> Vec2 {
        local_pos.as_vec2() + transform.translation.truncate()
    }

    /// Convert a world position to it's camera tile index.
    pub fn world_to_index(&self, transform: &GlobalTransform, world_pos: impl Point2d) -> IVec2 {
        let local = self.world_to_local(transform, world_pos);
        self.grid.pos_to_index(local)
    }

    /// Convert a camera tile index to it's world position to it's world position.
    pub fn index_to_world(&self, transform: &GlobalTransform, pos: impl GridPoint) -> Vec2 {
        let p = self.grid.index_to_pos(pos);
        self.local_to_world(transform, p)
    }

    /// Return the world center of the tile at the given tile index.
    pub fn index_to_tile_center(&self, transform: &GlobalTransform, index: impl GridPoint) -> Vec2 {
        let p = self.grid.index_to_tile_center(index);
        self.local_to_world(transform, p)
    }


    /// Converts a world position to a screen position (0..resolution)
    pub fn world_to_screen(
        camera: &Camera,
        windows: &Windows,
        camera_transform: &GlobalTransform,
        world_position: impl Point2d,
    ) -> Option<Vec2> {
        let window = windows.primary();
        let window_size = Vec2::new(window.width(), window.height());

        // Build a transform to convert from world to NDC using camera data
        let world_to_ndc: Mat4 =
            camera.projection_matrix() * camera_transform.compute_matrix().inverse();
        let ndc_space_coords: Vec3 = world_to_ndc.project_point3(world_position.as_vec2().extend(0.0));

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

impl Default for TiledCamera {
    fn default() -> Self {
        let pixels_per_tile = 8;
        let tile_count = UVec2::new(80,45);
        Self { 
            pixels_per_tile, 
            tile_count,
            grid: WorldGrid::unit_grid(tile_count, pixels_per_tile as u32)
        }
    }
}

pub struct TiledCameraSettings {
    pub pixels_per_unit: UVec2,
}


fn on_window_resized(
    windows: Res<Windows>,
    mut resize_events: EventReader<WindowResized>,
    mut q_cam: Query<(&mut OrthographicProjection, &mut Camera, &TiledCamera)>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.iter() {
        if resize_event.id == WindowId::primary() {
            let window = windows.primary();

            let wres = UVec2::new(window.physical_width(), window.physical_height());
            let (mut proj, mut cam, tiled_cam) = q_cam.single_mut();

            update_viewport(&tiled_cam, wres, &mut proj, &mut cam);
        }
    }
}

fn on_camera_changed(
    windows: Res<Windows>,
    mut q_cam: Query<(&mut OrthographicProjection, &mut Camera, &TiledCamera), Changed<TiledCamera>>,
) {
    for (mut proj, mut cam, tiled_cam) in q_cam.iter_mut() {
        let window = windows.primary();
        let wres = UVec2::new(window.physical_width(), window.physical_height());
        update_viewport(&tiled_cam, wres, &mut proj, &mut cam);
    }
}

fn update_viewport(
    tiled_cam: &TiledCamera,
    wres: UVec2,
    proj: &mut OrthographicProjection,
    cam: &mut Camera,
) {
    let tres = tiled_cam.target_resolution();
    let zoom = (wres / tres).min_element();


    proj.scaling_mode = ScalingMode::FixedVertical(tiled_cam.tile_count.y as f32);
    let vp_size = tres * zoom;
    let pos = (wres / 2) - (vp_size / 2);
    cam.viewport = Some(Viewport {
        physical_position: pos,
        physical_size: vp_size,
        ..default()
    });
}


// MIT License
// Copyright (c) 2021 Aevyrie
pub fn screen_to_world(
    screen_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    let view = camera_transform.compute_matrix();
    let screen_size = match camera.logical_target_size() {
        Some(s) => s,
        None => {
            error!(
                "Unable to get screen size for RenderTarget {:?}",
                camera.target
            );
            return None;
        }
    };
    let projection = camera.projection_matrix();

    // 2D Normalized device coordinate cursor position from (-1, -1) to (1, 1)
    let cursor_ndc = (screen_pos / screen_size) * 2.0 - Vec2::from([1.0, 1.0]);
    let ndc_to_world: Mat4 = view * projection.inverse();
    let world_to_ndc = projection * view;

    // Calculate the camera's near plane using the projection matrix
    let projection = projection.to_cols_array_2d();
    let camera_near = (2.0 * projection[3][2]) / (2.0 * projection[2][2] - 2.0);

    // Compute the cursor position at the near plane. The bevy camera looks at -Z.
    let ndc_near = world_to_ndc.transform_point3(-Vec3::Z * camera_near).z;
    let cursor_pos_near = ndc_to_world.transform_point3(cursor_ndc.extend(ndc_near));

    Some(cursor_pos_near.truncate())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_cam(pos: impl Point2d, tile_count: impl Size2d) -> (GlobalTransform, TiledCamera) {
        (GlobalTransform::from_translation(pos.as_vec2().extend(0.0)),
        TiledCamera::unit_cam(tile_count, 8))
    }

    fn make_pixel_cam(pos: impl Point2d, tile_count: impl Size2d) -> (GlobalTransform, TiledCamera) {
        (GlobalTransform::from_translation(pos.as_vec2().extend(0.0)),
        TiledCamera::pixel_cam(tile_count, 8))
    }


    #[test]
    fn world_to_index() {
        let (t,cam) = unit_cam([5.0, 5.0], [3,3]);
        let p = cam.world_to_index(&t, [4.5, 4.5]);
        assert_eq!([0,0], p.to_array());

        let (t,cam) = unit_cam([5.0, 5.0], [4,4]);
        let p = cam.world_to_index(&t, [4.5, 4.5]);
        assert_eq!([-1,-1], p.to_array());

        let (t,cam) = make_pixel_cam([16.0, 16.0], [3,3]);
        let p = cam.world_to_index(&t, [12.0, 12.0]);
        assert_eq!([0,0], p.to_array());

        let (t,cam) = make_pixel_cam([16.0, 16.0], [4,4]);
        let p = cam.world_to_index(&t, [12.0, 12.0]);
        assert_eq!([-1,-1], p.to_array());
    }

    #[test]
    fn index_to_world() {
        let (t,cam) = make_pixel_cam([5,5], [4,4]);
        let p = cam.index_to_world(&t, [3,3]);
        assert_eq!([29.0,29.0], p.to_array());

        let (t,cam) = unit_cam([5,5], [4,4]);
        let p = cam.index_to_world(&t, [3,3]);
        assert_eq!([8.0,8.0], p.to_array());

        let (t,cam) = make_pixel_cam([16,16], [3,3]);
        let p = cam.index_to_world(&t, [3,3]);
        assert_eq!([36.0,36.0], p.to_array());
    }
}