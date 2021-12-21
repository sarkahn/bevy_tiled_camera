use bevy::{
    prelude::*,
    render::camera::{CameraProjection, DepthCalculation},
};

use crate::sized_grid::{TileCenterIterator, TilePosIterator};

use super::sized_grid::SizedGrid;

/// A projection which will adjust itself based on your target pixels per tile and tile count.
///
/// The camera view will be scaled up to fill the window as much as possible while displaying
/// your target tile count and not deforming pixels.
///
/// Note that this projection assumes the size of one *tile* is equal to one world unit. This is
/// different than Bevy's default 2D orthographic camera which assumes one *pixel* is equal to one
/// world unit.
#[derive(Component, Debug)]
pub struct TiledProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,

    pub pixels_per_tile: u32,
    tile_count: UVec2,
    centered: bool,
    zoom: u32,
    grid: SizedGrid,
}

impl TiledProjection {
    fn new(target_tile_count: (u32, u32)) -> Self {
        let target_tile_count = UVec2::from(target_tile_count);
        let mut proj = TiledProjection {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1000.0,
            zoom: 1,
            centered: true,
            tile_count: target_tile_count,
            pixels_per_tile: 8,
            grid: SizedGrid::new(target_tile_count.into()),
        };
        proj.set_tile_count(target_tile_count.into());
        proj
    }

    pub fn uncentered(target_tile_count: (u32, u32)) -> Self {
        let target_tile_count = UVec2::from(target_tile_count);
        let mut proj = TiledProjection {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1000.0,
            zoom: 1,
            centered: false,
            tile_count: target_tile_count,
            pixels_per_tile: 8,
            grid: SizedGrid::new_uncentered(target_tile_count.into()),
        };
        proj.set_tile_count(target_tile_count.into());
        proj
    }

    /// Refers to how much the view is scaled up based on your pixels per tile and tile count settings.
    pub fn zoom(&self) -> u32 {
        self.zoom
    }

    pub fn centered(&self) -> bool {
        self.centered
    }

    pub fn tile_count(&self) -> UVec2 {
        self.tile_count
    }

    pub fn pixels_per_tile(&self) -> u32 {
        self.pixels_per_tile
    }

    pub fn set_tile_count(&mut self, tile_count: (u32, u32)) {
        self.grid = match self.centered {
            true => SizedGrid::new(tile_count),
            false => SizedGrid::new_uncentered(tile_count),
        };
        let tile_count = UVec2::from(tile_count);
        self.tile_count = tile_count;
    }

    pub fn set_centered(&mut self, centered: bool) {
        self.centered = centered;
        self.grid = match self.centered {
            true => SizedGrid::new(self.tile_count.into()),
            false => SizedGrid::new_uncentered(self.tile_count.into()),
        };
    }

    /// Converts a tile index to it's tile position in world space, or None if it's out of bounds.
    ///
    /// The "position" of a tile in world space is it's bottom left corner.
    pub fn tile_to_world(
        &self,
        cam_transform: &GlobalTransform,
        tile_pos: (i32, i32),
    ) -> Option<Vec3> {
        self.grid.tile_to_world(cam_transform, tile_pos)
    }

    /// Converts a world position to it's camera tile index, or None if it's out of bounds.
    pub fn world_to_tile(&self, cam_transform: &GlobalTransform, world_pos: Vec3) -> Option<IVec2> {
        self.grid.world_to_tile(cam_transform, world_pos)
    }

    /// Converts a tile index to it's tile center in world space.
    ///
    /// Returns none if the position is out of bounds.
    pub fn tile_center_world(
        &self,
        cam_transform: &GlobalTransform,
        tile_pos: (i32, i32),
    ) -> Option<Vec3> {
        self.grid.tile_to_tile_center_world(cam_transform, tile_pos)
    }

    /// Returns the center of a camera tile in world space, or None if it's out of bounds.
    pub fn world_to_tile_center(
        &self,
        cam_transform: &GlobalTransform,
        world_pos: Vec3,
    ) -> Option<Vec3> {
        self.grid.world_to_tile_center(cam_transform, world_pos)
    }

    /// An iterator over the center position in world space of every "tile" of the camera.
    pub fn tile_center_iter(&self, transform: &GlobalTransform) -> TileCenterIterator {
        self.grid.center_iter(transform)
    }

    /// An iterator over the position in world space of every tile of the camera.
    ///
    /// The "position" of a tile in world space is it's bottom left corner.
    pub fn tile_pos_iter(&self, transform: &GlobalTransform) -> TilePosIterator {
        self.grid.pos_iter(transform)
    }

    /// Converts a screen position [0..resolution] to a world position
    pub fn screen_to_world(
        &self,
        camera: &Camera,
        windows: &Windows,
        camera_transform: &GlobalTransform,
        screen_pos: Vec2,
    ) -> Option<Vec3> {
        let window = windows.get(camera.window)?;
        let window_size = Vec2::new(window.width(), window.height());

        // Convert screen position [0..resolution] to ndc [-1..1]
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        let min = -Vec2::ONE;
        let max = Vec2::ONE;
        let below_min = !ndc.cmpge(min);
        let above_max = !ndc.cmplt(max);
        if below_min.any() || above_max.any() {
            return None;
        }

        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos = world_pos.truncate().extend(0.0);

        Some(world_pos)
    }

    /// Converts a world position to a screen position (0..resolution)
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

impl Default for TiledProjection {
    fn default() -> Self {
        TiledProjection::new((5, 5))
    }
}

impl CameraProjection for TiledProjection {
    fn get_projection_matrix(&self) -> bevy::math::Mat4 {
        Mat4::orthographic_rh(
            self.left,
            self.right,
            self.bottom,
            self.top,
            // NOTE: near and far are swapped to invert the depth range from [0,1] to [1,0]
            // This is for interoperability with pipelines using infinite reverse perspective projections.
            self.far,
            self.near,
        )
    }

    fn update(&mut self, width: f32, height: f32) {
        let aspect = width / height;

        let tile_count = self.tile_count;

        let target_size = tile_count * self.pixels_per_tile;
        let window_size = UVec2::new(width as u32, height as u32);
        let zoom = (window_size / target_size).max(UVec2::ONE);

        self.zoom = zoom.min_element();

        let height = height / (self.zoom * self.pixels_per_tile) as f32;
        let width = height * aspect;

        if self.centered {
            let round_to_multiple = |value: f32, step: f32| step * (value / step).round();

            // Ensure our "edges" are sitting on the pixel grid, so sprites that also sit on the grid will render properly
            let pixel_size = 1.0 / (self.pixels_per_tile as f32 * self.zoom() as f32);
            let half_width = width / 2.0;
            let half_width = round_to_multiple(half_width, pixel_size);
            let half_height = height / 2.0;
            let half_height = round_to_multiple(half_height, pixel_size);

            self.left = -half_width;
            self.right = self.left + width;
            self.bottom = -half_height;
            self.top = self.bottom + height;
        } else {
            self.left = 0.0;
            self.bottom = 0.0;
            self.right = width;
            self.top = height;
        }
    }

    fn depth_calculation(&self) -> DepthCalculation {
        DepthCalculation::ZDifference
    }

    fn far(&self) -> f32 {
        self.far
    }
}
