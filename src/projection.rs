use bevy::{
    prelude::*,
    render::{camera::{CameraProjection, DepthCalculation}, view::window},
};

/// A projection which will adjust itself based on your target pixels per tile and tile count.
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
    target_tile_count: UVec2,
    pub centered: bool,
    zoom: u32,
    center_offset: Vec2,
}

impl TiledProjection {
    
    fn new(target_tile_count: (u32,u32)) -> Self {
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
            target_tile_count,
            pixels_per_tile: 8,
            center_offset: Default::default(),
        };
        proj.set_tile_count(target_tile_count.into());
        proj
    }
    /// Refers to how much the view is scaled up based on your pixels per tile and tile count settings.
    pub fn zoom(&self) -> u32 {
        self.zoom
    }


    pub fn set_tile_count(&mut self, tile_count: (u32,u32)) {
        let tile_count = UVec2::from(tile_count);
        self.target_tile_count = tile_count;
        self.center_offset = center_offset(true, tile_count);
    }

    /// Returns a tile position in world space
    pub fn tile_to_world(&self, cam_transform: &GlobalTransform, tile_pos: (i32,i32)) -> Option<Vec3> {
        let tile_pos = IVec2::from(tile_pos);
        if !self.tile_in_bounds(tile_pos) {
            return None;
        }

        let tile_offset = self.center_offset - Vec2::new(0.5,0.5);
        Some((cam_transform.translation.truncate() + tile_pos.as_vec2() + tile_offset).extend(0.0))
    }

    pub fn world_to_tile(&self, cam_transform: &GlobalTransform, world_pos: Vec3) -> IVec2 {
        let tile_offset = self.center_offset - Vec2::new(0.5,0.5);
        let local = world_pos.truncate() - cam_transform.translation.truncate();
        (local - tile_offset).floor().as_ivec2()
    }

    pub fn tile_center_world(&self, cam_transform: &GlobalTransform, tile_pos: (i32,i32)) -> Option<Vec3> {
        let tile_pos = IVec2::from(tile_pos);
        if !self.tile_in_bounds(tile_pos) {
            return None;
        }

        let tile_pos = tile_pos.as_vec2();
        Some((cam_transform.translation.truncate() + tile_pos + self.center_offset).extend(0.0))
    }


    /// Returns the center of a camera tile in world space, or none if it's out of bounds.
    pub fn world_to_tile_center(&self, 
        cam_transform: &GlobalTransform,
        world_pos: Vec3,
    ) -> Option<Vec3> {
        let tile = self.world_to_tile(cam_transform, world_pos);
        if !self.tile_in_bounds(tile) {
            return None;
        };
        self.tile_center_world(cam_transform, tile.into())
    }

    /// Converts a screen position to a world position
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
    
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    
        Some(world_pos)
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

    fn world_to_local(&self, cam_transform: &GlobalTransform, world_pos: Vec3) -> Vec2 {
        world_pos.truncate() - cam_transform.translation.truncate()
    }

    fn local_to_tile(&self, local: Vec2) -> IVec2 {
        (local.floor() - self.center_offset).as_ivec2()
    }

    fn tile_in_bounds(&self, tile_pos: IVec2) -> bool {
        let (min, max) = match self.centered {
            true => {
                let min = self.target_tile_count.as_ivec2() / 2;
                let max = min + self.target_tile_count.as_ivec2();
                (min,max)
            },
            false => {
                (IVec2::ZERO, self.target_tile_count.as_ivec2())
            },
        };
        
        let above_min = tile_pos.cmpge(min);
        let below_max = tile_pos.cmplt(max);
        above_min.all() && below_max.all()
    }

    fn local_in_bounds(&self, local: Vec2) -> bool {
        false
        // let (min, max) = match self.centered {
        //     true => (Vec2::ZERO, self.tile_count.as_vec2()),
        //     false => (),
        // }
    }

    fn min_max_local(&self) -> (Vec2,Vec2) {
        match self.centered {
            true => {
                let min = self.target_tile_count.as_vec2() / 2.0;
                (min, min + self.target_tile_count.as_vec2())
            }
            false => (Vec2::ZERO, self.target_tile_count.as_vec2()),
        }
    }
}

impl Default for TiledProjection {
    fn default() -> Self {
        TiledProjection::new((5,5))
    }
}

fn center_offset(centered: bool, size: UVec2) -> Vec2 {
    if !centered {
        return Vec2::new(0.5,0.5);
    }
    let b = (size % 2).cmpeq(UVec2::ZERO);
    Vec2::select(b, Vec2::new(0.5,0.5), Vec2::ZERO)
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

        let tile_count = self.target_tile_count;

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

#[cfg(test)]
mod test {
    use bevy::{render::camera::CameraProjection, math::IVec2, prelude::GlobalTransform};

    use crate::TiledProjection;

    #[test]
    fn round() {
        let round_to_multiple = |value: f32, step: f32| step * (value / step).round();
        let pixel_size = 1.0 / 8.0;
        let rounded = round_to_multiple(-2.5, pixel_size);
        println!("{}", rounded);
    }  

    #[test]
    fn test_projection() {
        let mut proj = TiledProjection {
            target_tile_count: (6,6).into(),
            pixels_per_tile: 10,
            ..Default::default()
        };
        // 8 * 20 = 160

        let p: &mut dyn CameraProjection = &mut proj;

        p.update(100.0, 100.0);

        //assert_eq!(proj.zoom(), 3);
    }

    #[test]
    fn size_test() {
        let size = 4.0f32;
        let half = size / 2.0;
        let remainder = half - half.floor();
        let p = 1.0f32;

        let t = p.floor() + half;

        println!("T {}, offset {}", t, remainder);
    }

    #[test]
    fn tile_pos_odd() {
        let proj = TiledProjection::new((3,3));
        let t = GlobalTransform::default();
        let p = proj.tile_to_world(&t, (0,0)).unwrap();
        assert_eq!(p.x, -0.5);
        assert_eq!(p.y, -0.5);
    }

    #[test]
    fn tile_pos_even() {
        let proj = TiledProjection::new((2,2));
        let t = GlobalTransform::default();
        let p = proj.tile_to_world(&t, (0,0)).unwrap();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn tile_center_odd() {
        let proj = TiledProjection::new((3,3));
        let t = GlobalTransform::default();
        let p = proj.tile_center_world(&t, (0,0)).unwrap();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn tile_center_even() {
        // Even tile pos should be + 0.5 
        let proj = TiledProjection::new((2,2));
        let t = GlobalTransform::default();
        let p = proj.tile_center_world(&t, (-1,-1)).unwrap();
        assert_eq!(p.x, -0.5);
        assert_eq!(p.y, -0.5);
    }

    #[test]
    fn tile_pos_diff() {
        let proj = TiledProjection::new((3,2));
        let t = GlobalTransform::default();
        let p = proj.tile_to_world(&t, (0,0)).unwrap();
        assert_eq!(p.x, -0.5);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn tile_center_diff() {
        let proj = TiledProjection::new((3,2));
        let t = GlobalTransform::default();
        let p = proj.tile_center_world(&t, (0,0)).unwrap();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.5);
    }

    #[test]
    fn world_to_tile() {
        let t = GlobalTransform::from_xyz(1.1, 2.5, 0.0);
        let proj = TiledProjection::new((3,3));
    }
}