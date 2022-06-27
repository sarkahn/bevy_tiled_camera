//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! [![Crates.io](https://img.shields.io/crates/v/bevy_tiled_camera)](https://crates.io/crates/bevy_tiled_camera)
//! [![docs](https://docs.rs/bevy_tiled_camera/badge.svg)](https://docs.rs/bevy_tiled_camera/)
//!
//! # `Bevy Tiled Camera`
//!
//! A camera for properly displaying low resolution pixel perfect 2D games in bevy.
//!
//! This camera will scale up the viewport as much as possible while maintaing your target
//! resolution and avoiding pixel artifacts.
//!
//! **Note**: Due to how resources are initialized, `TiledCameraPlugin` *must* be added
//! before `DefaultPlugins`. This ensures the default image filtering is set properly.
//!
//! ## Example
//! ```dont_run
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
//!     .add_plugin(TiledCameraPlugin)
//!     .add_plugins(DefaultPlugins)
//!     .add_startup_system(setup)
//!     .run();
//! }
//! ```
//!
//! ## Rendering Pixel Art
//! There's a unfortunately few important things to consider when displaying low resolution pixel art. Depending on how
//! your camera is set up and what your image filtering is, you might end up with extreme rendering artifacts. Your beautiful
//! pixels may deform in shape when moving, or you might see blinking lines between your tiles depending on the position
//! of your camera.
//!
//! This camera attempts to alleviate some of that, but you still have some decisions to make when deciding how to render
//! your game. Namely, you must decide how your world space will be defined, as it informs how all your sprites must
//! be sized and positioned in the game:
//!
//! ### `WorldSpace::Units`
//! With this method you decide on a set number of "pixels per unit". This defines how large a single
//! world unit is in pixels, which informs how the rest of the art in your game is displayed. This complicates
//! how your camera needs to be set up, as the viewport/camera projection must be adjusted precisely to properly display
//! your art.
//!
//! The upside is you can define all transforms and movement in your game in terms of world units,
//! the same as you would in 3d.
//!
//! This is the default for TiledCamera.
//!
//! ### `WorldSpace::Pixels`
//! This makes it easy to properly display sprites of different sizes, but complicates in game positioning
//! and movement. Since your space is defined by pixels, all motion and positioning are also defined by pixels,
//! which can be counterintuitive and often requires scaling your transforms.
//!
//! This is the default for bevy's built in orthographic camera.
//!
//! ## Customization
//!
//! BevyTiledCamera supports either world space, but defaults to `WorldSpace::Units`. You can change this during
//! construction:
//!
//! ```rust
//! use bevy_tiled_camera::*;
//! let cam = TiledCameraBundle::pixel_cam([5,5], 8);
//! // Or
//! let cam = TiledCameraBundle::new()
//!     .with_world_space(WorldSpace::Pixels);
//! ```

//! ## Versions
//! | bevy | bevy_tiled_camera |
//! | --- | --- |
//! | 0.8 | 0.4.0 |
//! | 0.6 | 0.3.0 |
//! | 0.5 | 0.2.4 |
//! | 0.5 | 0.2.3 |
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::{ScalingMode, Viewport},
        texture::{ImageSampler, ImageSettings},
    },
    window::{WindowId, WindowResized},
};
pub use sark_grids::world_grid::WorldSpace;
use sark_grids::{point::Point2d, world_grid::WorldGrid, *};

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
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            cam2d_bundle: Camera2dBundle { ..default() },
            tiled_camera: TiledCamera::default(),
        }
    }

    /// Construct a [`TiledCamera`] set to [`WorldSpace::Units`].
    pub fn unit_cam(tile_count: impl Size2d, pixels_per_tile: impl Size2d) -> Self {
        Self::new()
            .with_world_space(WorldSpace::Units)
            .with_tile_count(tile_count)
            .with_pixels_per_tile(pixels_per_tile)
    }

    /// Construct a [`TiledCamera`] set to [`WorldSpace::Pixels`].
    pub fn pixel_cam(tile_count: impl Size2d, pixels_per_tile: impl Size2d) -> Self {
        Self::new()
            .with_world_space(WorldSpace::Pixels)
            .with_tile_count(tile_count)
            .with_pixels_per_tile(pixels_per_tile)
    }

    /// Set the camera's [`WorldSpace`].
    pub fn with_world_space(mut self, world_space: WorldSpace) -> Self {
        self.tiled_camera.set_world_space(world_space);
        self
    }

    /// Set the camera's clear color.
    pub fn with_clear_color(mut self, color: Color) -> Self {
        self.cam2d_bundle.camera_2d.clear_color = ClearColorConfig::Custom(color);
        self
    }

    /// Set the camera's pixels per tile.
    ///
    /// This along with tile count and [`WorldSpace`] define how the camera
    /// sizes the viewport.
    pub fn with_pixels_per_tile(mut self, ppt: impl Size2d) -> Self {
        self.tiled_camera.pixels_per_tile = ppt.as_uvec2();
        self.tiled_camera.grid.pixels_per_tile = ppt.as_uvec2();
        self
    }

    /// Set the camera's tile count.
    ///
    /// This along with pixels per tile and [`WorldSpace`] define how the camera
    /// sizes the viewport.
    pub fn with_tile_count(mut self, tile_count: impl Size2d) -> Self {
        self.tiled_camera.tile_count = tile_count.as_uvec2();
        self.tiled_camera.grid.tile_count = tile_count.as_uvec2();
        self
    }

    /// Set the initial world position for the camera.
    pub fn with_camera_position(mut self, world_pos: impl Point2d) -> Self {
        let pos = &mut self.cam2d_bundle.transform.translation;
        *pos = world_pos.as_vec2().extend(pos.z);
        self
    }
}

/// A camera with a virtual grid for displaying low resolution pixel art.
///
/// Contains various functions for translating points between world space and the camera's
///"virtual grid tiles.
#[derive(Component)]
pub struct TiledCamera {
    /// Pixels per tile determines the size of your tiles/art, depending on
    /// the camera's [`WorldSpace`].
    pub pixels_per_tile: UVec2,
    /// The number of virtual grid tiles in the camera's viewport.
    pub tile_count: UVec2,
    /// World grid used for transforming positions.
    grid: WorldGrid,
    /// Camera zoom from the last viewport update.
    zoom: u32,
    /// Viewport size from the last viewport update.
    vp_size: UVec2,
    /// Viewport position from the last viewport update.
    vp_pos: UVec2,
}

impl TiledCamera {
    /// Creates a camera set to [`WorldSpace::Units`].
    pub fn unit_cam(tile_count: impl Size2d, pixels_per_tile: impl Size2d) -> Self {
        let tile_count = tile_count.as_uvec2();
        let pixels_per_tile = pixels_per_tile.as_uvec2();
        Self {
            pixels_per_tile,
            tile_count,
            grid: WorldGrid::unit_grid(tile_count, pixels_per_tile),
            ..default()
        }
    }

    /// Creates a camera set to [`WorldSpace::Pixels`].
    pub fn pixel_cam(tile_count: impl Size2d, pixels_per_tile: impl Size2d) -> Self {
        let tile_count = tile_count.as_uvec2();
        let pixels_per_tile = pixels_per_tile.as_uvec2();
        Self {
            pixels_per_tile,
            tile_count,
            grid: WorldGrid::pixel_grid(tile_count, pixels_per_tile),
            ..default()
        }
    }

    /// Retrieve the target resolution (in pixels) of the camera.
    pub fn target_resolution(&self) -> UVec2 {
        self.pixels_per_tile * self.tile_count
    }

    /// Returns an iterator that yields the center of the camera's virtual grid tiles in world space.
    pub fn tile_center_iter(&self, transform: &GlobalTransform) -> impl Iterator<Item = Vec2> {
        let xy = transform.translation.truncate();
        self.grid.tile_center_iter().map(move |p| p + xy)
    }

    /// Returns an iterator that yields the position of the camera's virtual grid tiles in world space.
    ///
    /// A tile's "position" refers to the bottom left corner of the tile.
    pub fn tile_pos_iter(&self, transform: &GlobalTransform) -> impl Iterator<Item = Vec2> {
        let xy = transform.translation.truncate();
        self.grid.tile_pos_iter().map(move |p| p + xy)
    }

    /// Transform from world space to camera-local space.
    pub fn world_to_local(&self, transform: &GlobalTransform, world_pos: impl Point2d) -> Vec2 {
        world_pos.as_vec2() - transform.translation.truncate()
    }

    /// Transform from camera-local space to world space.
    pub fn local_to_world(&self, transform: &GlobalTransform, local_pos: impl Point2d) -> Vec2 {
        local_pos.as_vec2() + transform.translation.truncate()
    }

    /// Convert a world position to it's virtual tile index.
    ///
    /// Tile indices are relative to the camera center.
    pub fn world_to_index(&self, transform: &GlobalTransform, world_pos: impl Point2d) -> IVec2 {
        let local = self.world_to_local(transform, world_pos);
        self.grid.pos_to_index(local)
    }

    /// Convert a world position to it's virtual tile position.
    ///
    /// A tile's "position" refers to the bottom left point of the tile.
    pub fn world_to_tile(&self, transform: &GlobalTransform, world_pos: impl Point2d) -> Vec2 {
        let local = self.world_to_local(transform, world_pos);
        self.grid.pos_to_tile_pos(local)
    }

    /// Convert a tile index to it's virtual tile position in world space.
    ///
    /// Tiles indices are relative to the camera center.
    ///
    /// A tile's "position" refers to the bottom left point of the tile.
    pub fn index_to_tile_pos(&self, transform: &GlobalTransform, pos: impl GridPoint) -> Vec2 {
        let p = self.grid.index_to_pos(pos);
        self.local_to_world(transform, p)
    }

    /// Return the world center of the virtual tile at the given tile index.
    ///
    /// Tile indices are relative to the camera center.
    pub fn index_to_tile_center(&self, transform: &GlobalTransform, index: impl GridPoint) -> Vec2 {
        let p = self.grid.index_to_tile_center(index);
        self.local_to_world(transform, p)
    }

    /// Change the camera's [`WorldSpace`].
    pub fn set_world_space(&mut self, world_space: WorldSpace) {
        self.grid.world_space = world_space;
    }

    /// Get the camera's [`WorldSpace`].
    pub fn world_space(&self) -> WorldSpace {
        self.grid.world_space
    }

    /// Get unit size or [`None`], depending on the camera's [`WorldSpace`].
    ///
    /// This can be used for sizing spawned sprites. If the camera's [`WorldSpace`]
    /// is [`WorldSpace::Units`] then a unit sized sprite should be one world unit.
    /// Otherwise it should use the default sprite size, which is the pixel dimensions
    /// of the sprite's texture.
    pub fn unit_size(&self) -> Option<Vec2> {
        match self.grid.world_space {
            WorldSpace::Units => Some(Vec2::ONE),
            WorldSpace::Pixels => None,
        }
    }

    /// How much the camera view is scaled up, based on target resolution and window size.
    pub fn zoom(&self) -> u32 {
        self.zoom
    }

    // MIT License
    // Copyright (c) 2021 Aevyrie
    // https://github.com/aevyrie/bevy_mod_raycast
    /// Convert a screen position (IE: The mouse cursor position) to it's corresponding world position.
    pub fn screen_to_world(
        &self,
        screen_pos: Vec2,
        camera: &Camera,
        camera_transform: &GlobalTransform,
    ) -> Option<Vec2> {
        let screen_size = self.vp_size.as_vec2();
        let screen_pos = (screen_pos - self.vp_pos.as_vec2()).round();

        let view = camera_transform.compute_matrix();
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
        let cursor_pos_near = cursor_pos_near.truncate() * self.grid.world_space_scale();
        Some(cursor_pos_near)
    }

    /// Converts a world position to a screen position (0..resolution)
    pub fn world_to_screen(
        &self,
        world_pos: impl Point2d,
        camera: &Camera,
        camera_transform: &GlobalTransform,
    ) -> Option<Vec2> {
        let window_size = self.vp_size.as_vec2();

        // Build a transform to convert from world to NDC using camera data
        let world_to_ndc: Mat4 =
            camera.projection_matrix() * camera_transform.compute_matrix().inverse();
        let ndc_space_coords: Vec3 = world_to_ndc.project_point3(world_pos.as_vec2().extend(0.0));

        // NDC z-values outside of 0 < z < 1 are outside the camera frustum and are thus not in screen space
        if ndc_space_coords.z < 0.0 || ndc_space_coords.z > 1.0 {
            return None;
        }

        // Once in NDC space, we can discard the z element and rescale x/y to fit the screen
        let screen_space_coords = (ndc_space_coords.truncate() + Vec2::ONE) / 2.0 * window_size;
        if !screen_space_coords.is_nan() {
            Some((screen_space_coords + self.vp_pos.as_vec2()).round())
        } else {
            None
        }
    }

    /// Retrieve the camera's [`WorldGrid`].
    pub fn world_grid(&self) -> &WorldGrid {
        &self.grid
    }
}

impl Default for TiledCamera {
    fn default() -> Self {
        let pixels_per_tile = UVec2::new(8,8);
        let tile_count = UVec2::new(80, 45);
        Self {
            pixels_per_tile,
            tile_count,
            grid: WorldGrid::unit_grid(tile_count, pixels_per_tile),
            zoom: 1,
            vp_size: UVec2::ONE,
            vp_pos: UVec2::ZERO,
        }
    }
}

fn on_window_resized(
    windows: Res<Windows>,
    mut resize_events: EventReader<WindowResized>,
    mut q_cam: Query<(&mut OrthographicProjection, &mut Camera, &mut TiledCamera)>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.iter() {
        if resize_event.id == WindowId::primary() {
            let window = windows.primary();

            let wres = UVec2::new(window.physical_width(), window.physical_height());
            if let Ok((mut proj, mut cam, mut tiled_cam)) = q_cam.get_single_mut() {
                update_viewport(&mut tiled_cam, wres, &mut proj, &mut cam);
            }

        }
    }
}

fn on_camera_changed(
    windows: Res<Windows>,
    mut q_cam: Query<
        (&mut OrthographicProjection, &mut Camera, &mut TiledCamera),
        Changed<TiledCamera>,
    >,
) {
    for (mut proj, mut cam, mut tiled_cam) in q_cam.iter_mut() {
        let window = windows.primary();
        let wres = UVec2::new(window.physical_width(), window.physical_height());
        update_viewport(&mut tiled_cam, wres, &mut proj, &mut cam);
    }
}

fn update_viewport(
    tiled_cam: &mut TiledCamera,
    wres: UVec2,
    proj: &mut OrthographicProjection,
    cam: &mut Camera,
) {
    let tres = tiled_cam.target_resolution().as_vec2();
    let wres = wres.as_vec2();
    let zoom = (wres / tres).min_element().max(1.0);

    // The 'size' of the orthographic projection.
    // 
    // For a `FixedVertical` projection this refers to the size of the
    // projection in vertical units.
    let ortho_size = match tiled_cam.world_space() {
        WorldSpace::Units => tiled_cam.tile_count.y as f32,
        WorldSpace::Pixels => tiled_cam.pixels_per_tile.y as f32 * zoom,
    };

    proj.scaling_mode = ScalingMode::FixedVertical(ortho_size);
    //let scale = tiled_cam.tile_count.y as f32 * tiled_cam.grid.world_space_scale().y;
    // if scale.x > scale.y {
    //     proj.scaling_mode = ScalingMode::FixedHorizontal(scale.x);
    // } else {
    //     proj.scaling_mode = ScalingMode::FixedVertical(scale.y);
    // }
    let vp_size = tres * zoom;
    let vp_pos = if wres.cmple(tres).any() {
        Vec2::ZERO
    } else {
        (wres / 2.0) - (vp_size / 2.0)
    }.floor();

    // cam.viewport = Some(Viewport {
    //     physical_position: vp_pos.as_uvec2(),
    //     physical_size: vp_size.as_uvec2(),
    //     ..default()
    // });

    
    cam.viewport = Some(Viewport {
       // physical_position: vp_pos.as_uvec2(),
        physical_size: vp_size.as_uvec2(),
        ..default()
    });

    println!("Tile count {}, Window res {}, Target resolution {}, zoom {}, PPU {}. VP pos {}. VP size {}", 
        tiled_cam.tile_count,
        wres,
        tres, 
        zoom,
        tiled_cam.pixels_per_tile,
        vp_pos,
        vp_size
    );


    // Camera values may have been changed manually - update grid values.
    tiled_cam.grid.tile_count = tiled_cam.tile_count;
    tiled_cam.grid.pixels_per_tile = tiled_cam.pixels_per_tile;
    tiled_cam.zoom = zoom as u32;
    tiled_cam.vp_pos = vp_pos.as_uvec2();
    tiled_cam.vp_size = vp_size.as_uvec2();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_cam(pos: impl Point2d, tile_count: impl Size2d) -> (GlobalTransform, TiledCamera) {
        (
            GlobalTransform::from_translation(pos.as_vec2().extend(0.0)),
            TiledCamera::unit_cam(tile_count, [8,8]),
        )
    }

    fn make_pixel_cam(
        pos: impl Point2d,
        tile_count: impl Size2d,
    ) -> (GlobalTransform, TiledCamera) {
        (
            GlobalTransform::from_translation(pos.as_vec2().extend(0.0)),
            TiledCamera::pixel_cam(tile_count, [8,8]),
        )
    }

    #[test]
    fn world_to_index() {
        let (t, cam) = unit_cam([5.0, 5.0], [3, 3]);
        let p = cam.world_to_index(&t, [4.5, 4.5]);
        assert_eq!([0, 0], p.to_array());

        let (t, cam) = unit_cam([5.0, 5.0], [4, 4]);
        let p = cam.world_to_index(&t, [4.5, 4.5]);
        assert_eq!([-1, -1], p.to_array());

        let (t, cam) = make_pixel_cam([16.0, 16.0], [3, 3]);
        let p = cam.world_to_index(&t, [12.0, 12.0]);
        assert_eq!([0, 0], p.to_array());

        let (t, cam) = make_pixel_cam([16.0, 16.0], [4, 4]);
        let p = cam.world_to_index(&t, [12.0, 12.0]);
        assert_eq!([-1, -1], p.to_array());
    }

    #[test]
    fn index_to_world() {
        let (t, cam) = make_pixel_cam([5, 5], [4, 4]);
        let p = cam.index_to_tile_pos(&t, [3, 3]);
        assert_eq!([29.0, 29.0], p.to_array());

        let (t, cam) = unit_cam([5, 5], [4, 4]);
        let p = cam.index_to_tile_pos(&t, [3, 3]);
        assert_eq!([8.0, 8.0], p.to_array());

        let (t, cam) = make_pixel_cam([16, 16], [3, 3]);
        let p = cam.index_to_tile_pos(&t, [3, 3]);
        assert_eq!([36.0, 36.0], p.to_array());
    }

    #[test]
    fn new() {
        let cam = TiledCameraBundle::pixel_cam([5, 5], [8,8]).tiled_camera;
        assert_eq!(cam.world_space(), WorldSpace::Pixels);

        let cam = TiledCameraBundle::unit_cam([5, 5], [8,8]).tiled_camera;
        assert_eq!(cam.world_space(), WorldSpace::Units);
    }
}
