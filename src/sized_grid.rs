use bevy::prelude::*;

/// A utility for retrieving positions from a unit sized grid.
#[derive(Debug, Clone)]
pub struct SizedGrid {
    tile_count: UVec2,
    center_offset: Vec2,
    centered: bool,
}

impl SizedGrid {
    /// Create a new grid where the origin [0,0] is the center of the grid.
    ///
    /// **IE:**
    ///
    /// |-1,-1| 0, 1| 1, 1|
    ///
    /// |-1, 0| 0, 0| 1, 0|
    ///
    /// |-1,-1| 0,-1| 1,-1|
    pub fn new(tile_count: [u32; 2]) -> Self {
        let tile_count = UVec2::from(tile_count);
        let b = (tile_count % 2).cmpeq(UVec2::ZERO);
        let center_offset = Vec2::select(b, Vec2::new(0.5, 0.5), Vec2::ZERO);

        SizedGrid {
            tile_count,
            center_offset,
            centered: true,
        }
    }

    /// Create a new grid where the origin [0,0] is the bottom left of the grid.
    ///
    /// **IE:**
    ///
    /// | 0, 2| 1, 2| 2, 2|
    ///
    /// | 0, 1| 1, 1| 2, 1|
    ///
    /// | 0, 0| 1, 0| 2, 0|
    pub fn new_uncentered(tile_count: [u32; 2]) -> Self {
        let tile_count = UVec2::from(tile_count);
        let center_offset = Vec2::new(0.5, 0.5);

        SizedGrid {
            tile_count,
            center_offset,
            centered: false,
        }
    }

    /// Converts a tile index to a it's world position.
    ///
    /// The "position" of a tile in world space is it's bottom left corner.
    /// Returns None if the position is out of bounds.
    pub fn tile_to_world(&self, transform: &GlobalTransform, tile_pos: [i32; 2]) -> Option<Vec3> {
        let tile_pos = IVec2::from(tile_pos);
        if !self.tile_in_bounds(tile_pos) {
            return None;
        }
        Some(self.tile_to_world_unchecked(transform, tile_pos))
    }

    /// Converts a world position to a tile index.
    ///
    /// The "position" of a tile is it's bottom left corner. Returns None if the position is out of bounds.
    pub fn world_to_tile(&self, transform: &GlobalTransform, world_pos: Vec3) -> Option<IVec2> {
        let tile_pos = self.world_to_tile_unchecked(transform, world_pos);
        if !self.tile_in_bounds(tile_pos) {
            return None;
        }
        Some(tile_pos)
    }

    /// Converts a tile position into that tile's center in world space.
    ///
    /// Returns None if the position is out of bounds.
    pub fn tile_to_tile_center_world(
        &self,
        transform: &GlobalTransform,
        tile_pos: [i32; 2],
    ) -> Option<Vec3> {
        let tile_pos = IVec2::from(tile_pos);
        if !self.tile_in_bounds(tile_pos) {
            return None;
        }

        Some(self.tile_center_world_unchecked(transform, tile_pos))
    }

    /// Converts a world position to a tile's center in world space.
    ///
    /// Returns None if the position is out of bounds.
    pub fn world_to_tile_center(
        &self,
        cam_transform: &GlobalTransform,
        world_pos: Vec3,
    ) -> Option<Vec3> {
        if let Some(tile) = self.world_to_tile(cam_transform, world_pos) {
            return Some(self.tile_center_world_unchecked(cam_transform, tile));
        }
        None
    }

    fn tile_to_world_unchecked(&self, transform: &GlobalTransform, tile_pos: IVec2) -> Vec3 {
        let local = self.tile_to_local(tile_pos);
        self.local_to_world(transform, local)
    }

    fn world_to_tile_unchecked(&self, transform: &GlobalTransform, world_pos: Vec3) -> IVec2 {
        let local = self.world_to_local(transform, world_pos);
        self.local_to_tile(local)
    }

    fn tile_center_world_unchecked(&self, transform: &GlobalTransform, tile_pos: IVec2) -> Vec3 {
        let world = self.tile_to_world_unchecked(transform, tile_pos).truncate();
        (world + Vec2::new(0.5, 0.5)).extend(0.0)
    }

    fn local_to_world(&self, transform: &GlobalTransform, local_pos: Vec2) -> Vec3 {
        (local_pos + transform.translation.truncate()).extend(0.0)
    }

    fn world_to_local(&self, transform: &GlobalTransform, world_pos: Vec3) -> Vec2 {
        world_pos.truncate() - transform.translation.truncate()
    }

    fn tile_to_local(&self, tile: IVec2) -> Vec2 {
        tile.as_vec2() + self.tile_offset()
    }

    fn local_to_tile(&self, local: Vec2) -> IVec2 {
        (local - self.tile_offset()).floor().as_ivec2()
    }

    fn tile_offset(&self) -> Vec2 {
        self.center_offset - Vec2::new(0.5, 0.5)
    }

    /// Retrieve the bottom left corner of the grid in world space.
    pub fn min_world_position(&self, transform: &GlobalTransform) -> Vec3 {
        let min_cell = match self.centered {
            true => -self.tile_count.as_ivec2() / 2,
            false => IVec2::ZERO,
        };
        self.tile_to_world_unchecked(transform, min_cell)
    }

    /// Whether or not the given tile is in the bounds of the grid.
    pub fn tile_in_bounds(&self, tile_pos: IVec2) -> bool {
        let (min, max) = match self.centered {
            true => {
                let min = -self.tile_count.as_ivec2() / 2;
                let max = min + self.tile_count.as_ivec2();
                (min, max)
            }
            false => (IVec2::ZERO, self.tile_count.as_ivec2()),
        };

        let above_min = tile_pos.cmpge(min);
        let below_max = tile_pos.cmplt(max);
        above_min.all() && below_max.all()
    }

    /// An iterator over the tile position in world space of every tile in the grid.
    ///
    /// The "position" of a tile in world space is it's bottom left corner.
    pub fn pos_iter(&self, transform: &GlobalTransform) -> TilePosIterator {
        TilePosIterator::from_grid(self, transform)
    }

    /// An iterator over the center position in world space of every tile in the grid.
    pub fn center_iter(&self, transform: &GlobalTransform) -> TileCenterIterator {
        TileCenterIterator::from_grid(self, transform)
    }
}

pub struct TileCenterIterator {
    iter: TilePosIterator,
}

impl TileCenterIterator {
    fn from_grid(grid: &SizedGrid, transform: &GlobalTransform) -> Self {
        TileCenterIterator {
            iter: grid.pos_iter(transform),
        }
    }
}

impl Iterator for TileCenterIterator {
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.iter.next() {
            return Some(p + Vec3::new(0.5, 0.5, 0.0));
        }
        None
    }
}

pub struct TilePosIterator {
    min: Vec2,
    width: u32,
    current: u32,
    length: u32,
}

impl TilePosIterator {
    fn from_grid(grid: &SizedGrid, transform: &GlobalTransform) -> Self {
        TilePosIterator {
            min: grid.min_world_position(transform).truncate(),
            width: grid.tile_count.x,
            current: 0,
            length: grid.tile_count.x * grid.tile_count.y,
        }
    }
}

impl Iterator for TilePosIterator {
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = (self.current..self.length).next() {
            self.current += 1;

            let xy = UVec2::new(i % self.width, i / self.width).as_vec2();

            return Some((self.min + xy).extend(0.0));
        }
        None
    }
}

#[cfg(test)]
mod test {
    use bevy::{
        math::{IVec2, Vec2, Vec3},
        prelude::GlobalTransform,
    };

    use super::SizedGrid;
    #[test]
    fn tile_to_world_odd() {
        let grid = SizedGrid::new([3, 3]);
        let t = GlobalTransform::default();
        let p = grid.tile_to_world(&t, [0, 0]).unwrap();
        assert_eq!(p.x, -0.5);
        assert_eq!(p.y, -0.5);
    }

    #[test]
    fn tile_to_world_even() {
        let grid = SizedGrid::new([2, 2]);
        let t = GlobalTransform::default();
        let p = grid.tile_to_world(&t, [0, 0]).unwrap();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn tile_center_odd() {
        let grid = SizedGrid::new([3, 3]);
        let t = GlobalTransform::default();
        let p = grid.tile_to_tile_center_world(&t, [0, 0]).unwrap();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);

        let p = grid.tile_to_tile_center_world(&t, [1, 0]).unwrap();
        assert_eq!(p.x, 1.0);
    }

    #[test]
    fn tile_center_even() {
        // Even tile pos should be + 0.5
        let grid = SizedGrid::new([2, 2]);
        let t = GlobalTransform::default();

        let p = grid.tile_to_tile_center_world(&t, [0, 0]).unwrap();
        assert_eq!(p.x, 0.5);
        assert_eq!(p.y, 0.5);

        let p = grid.tile_to_tile_center_world(&t, [-1, -1]).unwrap();
        assert_eq!(p.x, -0.5);
        assert_eq!(p.y, -0.5);
    }

    #[test]
    fn tile_pos_diff() {
        let grid = SizedGrid::new([3, 2]);
        let t = GlobalTransform::default();
        let p = grid.tile_to_world(&t, [0, 0]).unwrap();
        assert_eq!(p.x, -0.5);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn tile_center_diff() {
        let grid = SizedGrid::new([3, 2]);
        let t = GlobalTransform::default();
        let p = grid.tile_to_tile_center_world(&t, [0, 0]).unwrap();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.5);
    }

    #[test]
    fn tile_to_local_even() {
        let grid = SizedGrid::new([2, 2]);
        let p = grid.tile_to_local(IVec2::new(0, 0));
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn tile_to_local_odd() {
        let grid = SizedGrid::new([3, 3]);
        let p = grid.tile_to_local(IVec2::new(0, 0));
        assert_eq!(p.x, -0.5);
        assert_eq!(p.y, -0.5);
    }

    #[test]
    fn uncentered_odd() {
        let t = GlobalTransform::default();
        let grid = SizedGrid::new_uncentered([3, 3]);
        let p = grid.tile_to_tile_center_world(&t, [0, 0]).unwrap();
        assert_eq!(p.x, 0.5);
        assert_eq!(p.y, 0.5);
    }

    #[test]
    fn local_to_tile_odd() {
        let grid = SizedGrid::new([3, 3]);

        let p = grid.local_to_tile(Vec2::new(0.85, 0.85));

        assert_eq!(p.x, 1);
        assert_eq!(p.y, 1);
    }

    #[test]
    fn local_to_tile_even() {
        let grid = SizedGrid::new([4, 4]);

        let p = grid.local_to_tile(Vec2::new(0.85, 0.85));

        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
    }

    #[test]
    fn moved_tile_to_world() {
        let grid = SizedGrid::new([3, 3]);
        let t = GlobalTransform::from_xyz(3.0, 0.0, 0.0);
        let p = grid.tile_to_world(&t, [0, 0]).unwrap();
        assert_eq!(p.x, 2.5);
        assert_eq!(p.y, -0.5);
    }

    #[test]
    fn moved_tile_to_center() {
        let grid = SizedGrid::new([3, 3]);
        let t = GlobalTransform::from_xyz(3.0, 0.0, 0.0);
        let p = grid.tile_to_tile_center_world(&t, [0, 0]).unwrap();
        assert_eq!(p.x, 3.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn min() {
        let t = GlobalTransform::default();
        let grid = SizedGrid::new([3, 3]);
        let min = grid.min_world_position(&t);

        assert_eq!(min.x, -1.5);
        assert_eq!(min.y, -1.5);
    }

    #[test]
    fn center_iter() {
        let t = GlobalTransform::default();
        let grid = SizedGrid::new([3, 3]);

        let iter = grid.center_iter(&t);

        let points: Vec<_> = iter.collect();

        assert!(points.contains(&Vec3::new(-1.0, -1.0, 0.0)));
        assert!(points.contains(&Vec3::new(0.0, -1.0, 0.0)));
        assert!(points.contains(&Vec3::new(1.0, -1.0, 0.0)));
        assert!(points.contains(&Vec3::new(-1.0, 0.0, 0.0)));
        assert!(points.contains(&Vec3::new(0.0, 0.0, 0.0)));
        assert!(points.contains(&Vec3::new(1.0, 0.0, 0.0)));
        assert!(points.contains(&Vec3::new(-1.0, 1.0, 0.0)));
        assert!(points.contains(&Vec3::new(0.0, 1.0, 0.0)));
        assert!(points.contains(&Vec3::new(1.0, 1.0, 0.0)));
    }

    #[test]
    fn pos_iter() {
        let t = GlobalTransform::default();
        let grid = SizedGrid::new([3, 3]);

        let iter = grid.pos_iter(&t);

        let points: Vec<_> = iter.collect();

        assert!(points.contains(&Vec3::new(-1.5, -1.5, 0.0)));
        assert!(points.contains(&Vec3::new(-0.5, -1.5, 0.0)));
        assert!(points.contains(&Vec3::new(0.5, -1.5, 0.0)));
        assert!(points.contains(&Vec3::new(-1.5, -0.5, 0.0)));
        assert!(points.contains(&Vec3::new(-0.5, -0.5, 0.0)));
        assert!(points.contains(&Vec3::new(0.5, -0.5, 0.0)));
        assert!(points.contains(&Vec3::new(-1.5, 0.5, 0.0)));
        assert!(points.contains(&Vec3::new(-0.5, 0.5, 0.0)));
        assert!(points.contains(&Vec3::new(0.5, 0.5, 0.0)));
    }
}
