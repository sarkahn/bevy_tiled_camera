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
//! |0.7.1| 0.4.0 |
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
    }
};
use sark_grids::{*, world_grid::WorldGrid};

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
    pub cam2d_bundle: Camera2dBundle,
    pub tiled_camera: TiledCamera,
}

impl TiledCameraBundle {
    pub fn new() -> Self {
        TiledCameraBundle::default()
    }

    pub fn with_pixels_per_tile(mut self, ppt: impl Size2d) -> Self {
        self.tiled_camera.pixels_per_tile = ppt.as_uvec2();
        self
    }

    pub fn with_tile_count(mut self, tile_count: impl Size2d) -> Self {
        self.tiled_camera.tile_count = tile_count.as_uvec2();
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
    pub pixels_per_tile: UVec2,
    pub tile_count: UVec2,
}

impl TiledCamera {
    pub fn target_resolution(&self) -> UVec2 {
        self.tile_count * self.pixels_per_tile
    }

    pub fn tile_center_iter(&self, transform: &GlobalTransform) -> impl Iterator<Item=Vec2> {
        let xy = transform.translation.truncate();
        WorldGrid::new(self.tile_count, Pivot::Center).tile_center_iter().map(move |p|p + xy)
    }

    // pub fn world_to_tile(&self) -> Option<UVec2> {
    //     WorldGrid::new(self.tile_count, Pivot::Center).world_to_local(point)
    // }
}

impl Default for TiledCamera {
    fn default() -> Self {
        Self { 
            pixels_per_tile: UVec2::new(8,8), 
            tile_count: UVec2::new(80,45),
        }
    }
}

pub struct TiledCameraSettings {
    pub pixels_per_unit: UVec2,
}



// impl TiledCameraBundle {
//     pub fn new() -> Self {
//         TiledCameraBundle::default()
//     }

//     /// Sets up the projection to display the given number of pixels per tile.
//     pub fn with_pixels_per_tile(mut self, pixels_per_tile: u32) -> Self {
//         self.projection.pixels_per_tile = pixels_per_tile;
//         self
//     }

//     /// Sets the projection to display the given tile count.
//     pub fn with_tile_count(mut self, tile_count: [u32; 2]) -> Self {
//         self.projection.set_tile_count(tile_count);
//         self
//     }

//     /// Sets the camera position on spawn.
//     pub fn with_camera_position(mut self, position: [f32; 2]) -> Self {
//         let position = Vec2::from(position);
//         let old_pos = self.transform.translation;
//         self.transform.translation = position.extend(old_pos.z);
//         self
//     }

//     /// Will the camera projection be centered or not? Defaults to true.
//     pub fn with_centered(mut self, centered: bool) -> Self {
//         self.projection.set_centered(centered);
//         self
//     }

//     /// Camera will be scaled to be as close as possible to the given target resolution given
//     /// pixels per tile.
//     pub fn with_target_resolution(self, pixels_per_tile: u32, resolution: [u32; 2]) -> Self {
//         let resolution = UVec2::from(resolution);
//         self.with_pixels_per_tile(pixels_per_tile)
//             .with_tile_count((resolution / pixels_per_tile).into())
//     }
// }

// impl Default for TiledCameraBundle {
//     fn default() -> Self {
//         let projection = TiledProjection::default();
//         let transform = Transform::from_xyz(0.0, 0.0, projection.far - 0.1);
//         TiledCameraBundle {
//             camera: Camera::default(),
//             transform,
//             projection,
//             visible_entities: Default::default(),
//             frustum: Default::default(),
//             global_transform: Default::default(),
//             marker: Camera2d,
//         }
//     }
// }

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