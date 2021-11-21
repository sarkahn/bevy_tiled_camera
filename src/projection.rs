use bevy::{
    prelude::*,
    render::camera::{CameraProjection, DepthCalculation},
};

/// A projection which will adjust itself based on your target pixels per tile and tile count.
/// The camera view will be scaled up to fill the window as much as possible while displaying
/// your target tile count and not deforming pixels.
///
/// Note that this projection assumes the size of one tile is equal to one world unit. This is
/// different than Bevy's default 2D orthographic camera which assumes one *pixel* is equal to one
/// world unit.
pub struct TiledProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,

    pub pixels_per_tile: u32,
    pub target_tile_count: UVec2,
    pub centered: bool,
    zoom: u32,
}

impl TiledProjection {
    /// Refers to how much the view is scaled up based on your pixels per tile and tile count settings.
    pub fn zoom(&self) -> u32 {
        self.zoom
    }
    pub fn width(&self) -> u32 {
        (self.right - self.left) as u32
    }

    pub fn height(&self) -> u32 {
        (self.top - self.bottom) as u32
    }
}

impl Default for TiledProjection {
    fn default() -> Self {
        TiledProjection {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1000.0,
            zoom: 1,
            centered: true,
            target_tile_count: UVec2::new(18, 10),
            pixels_per_tile: 8,
        }
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

        let mut tile_count = Vec2::new(
            self.target_tile_count.x as f32,
            self.target_tile_count.y as f32,
        );

        let tile_count = match self.centered {
            // Ensure our tile count is always a multiple of two for correct rendering with a centered camera
            true => {
                tile_count = (tile_count / 2.0).ceil() * 2.0;
                UVec2::new(tile_count.x as u32, tile_count.y as u32)
            }
            false => self.target_tile_count,
        };

        let target_size = tile_count * self.pixels_per_tile;
        let window_size = UVec2::new(width as u32, height as u32);
        let zoom = (window_size / target_size).max(UVec2::ONE);

        self.zoom = zoom.min_element();

        let height = height / (self.zoom * self.pixels_per_tile) as f32;
        let width = height * aspect;

        if self.centered {
            let round_to_multiple = |value: f32, step: f32| step * (value / step).round();

            // Ensure our "edges" are sitting on the pixel grid, so sprites that also sit on the grid will render properly
            let pixel_size = 1.0 / self.pixels_per_tile as f32;
            let left = width / 2.0;
            let left = -round_to_multiple(left, pixel_size);
            let bottom = height / 2.0;
            let bottom = -round_to_multiple(bottom, pixel_size);

            self.left = left;
            self.right = left + width;
            self.bottom = bottom;
            self.top = bottom + height;
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
}
