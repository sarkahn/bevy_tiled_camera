use bevy::prelude::*;

/// A grid with indices [0,0] start at the bottom left,
/// But with a local position (0.0,0.0) starting in the center.
#[derive(Default, Component, Clone)]
pub struct SizedGrid {
    size: UVec2,
    min: Vec2,
    max: Vec2,
    center_offset: Vec2,
}

impl SizedGrid {
    pub fn new(size: (u32,u32)) -> Self {
        let size = UVec2::from(size);
        let half = size / 2;
        let offset = (size % 2).cmpeq(UVec2::ZERO);
        let offset = Vec2::select(offset, Vec2::new(0.5,0.5), Vec2::ZERO);
        SizedGrid {
            size,
            min: -half.as_vec2(),
            max: half.as_vec2(),
            center_offset: offset,
        }
    }

    pub fn set_size(&mut self, size: (u32,u32)) {
        let size = UVec2::from(size);
        self.size;
        let offset = (size % 2).cmpeq(UVec2::ZERO);
        self.center_offset = Vec2::select(offset, Vec2::new(0.5,0.5), Vec2::ZERO);
    }

    /// Transform a local position to a cell position.
    pub fn to_cell(&self, pos: (f32,f32)) -> IVec2 {
        Vec2::from(pos).floor().as_ivec2()
    }
    

    /// Snaps a position to it's corresponding cell position in local space.
    pub fn pos_snap(&self, local_pos: (f32, f32)) -> Vec2 {
        self.to_cell(local_pos).as_vec2()
    }

    /// Transform a cell position to the center point of a grid cell in local space.
    pub fn cell_to_cell_center(&self, cell_pos: (i32,i32)) -> Vec2 {
        self.to_local(cell_pos.into()) + self.center_offset
    }

    #[inline]
    pub fn to_local(&self, cell_pos: (i32,i32)) -> Vec2 {
        IVec2::from(cell_pos).as_vec2()
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::{Vec2};
    use assert_approx_eq::assert_approx_eq;

    use super::SizedGrid;

    #[test]
    fn cell_center() {
        let grid = SizedGrid::new((3,3));
        let (x,y) = grid.cell_to_cell_center((0,0)).into();

        assert_approx_eq!(0.0, x);
        assert_approx_eq!(0.0, y);
        
        let grid = SizedGrid::new((3,3));
        let (x,y) = grid.cell_to_cell_center((0,0)).into();

        assert_approx_eq!(0.0, x);
        assert_approx_eq!(0.0, y);
    }

    #[test]
    fn to_cell() {
        let grid = SizedGrid::new((3,3));
        let (x,y) = grid.to_cell((10.5, 11.12)).into();
        assert_eq!(x, 10);
        assert_eq!(y, 11);

        let (x,y) = grid.to_cell((-5.1, -12.3)).into();
        assert_eq!(x, -6);
        assert_eq!(y, -13);
    }

    #[test]
    fn snap() {
        let grid = SizedGrid::default();
        let world = Vec2::new(15.5, -10.12);

        let snapped = grid.pos_snap(world.into());

        assert_approx_eq!(snapped.x, 15.0);
        assert_approx_eq!(snapped.y, -11.0);
    }
}
