use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct PositionGrid {
    cell_half_size: Vec2,
    cell_size: Vec2,
}

impl PositionGrid {
    pub fn new(cell_size: (f32,f32)) -> Self {
        PositionGrid {
            cell_size: Vec2::from(cell_size),
            cell_half_size: Vec2::new(0.5,0.5),
        }
    }

    pub fn set_cell_size(&mut self, size: (f32,f32)) {
        self.cell_size = size.into();
        self.cell_half_size = self.cell_size / 2.0;
    }

    pub fn cell_size(&self) -> Vec2 {
        self.cell_size
    }

    /// Transform a local position to a cell position.
    pub fn to_cell(&self, pos: (f32,f32)) -> IVec2 {
        (Vec2::from(pos) / self.cell_size).floor().as_ivec2()
    }
    

    /// Snaps a position to it's corresponding cell position in local space.
    pub fn pos_snap(&self, local_pos: (f32, f32)) -> Vec2 {
        self.to_cell(local_pos).as_vec2() * self.cell_size
    }

    /// Transform a cell position to the center point of a grid cell in local space.
    pub fn cell_to_cell_center(&self, cell_pos: (i32,i32)) -> Vec2 {
        self.to_local(cell_pos.into()) + self.cell_half_size
    }

    #[inline]
    pub fn to_local(&self, cell_pos: (i32,i32)) -> Vec2 {
        IVec2::from(cell_pos).as_vec2() * self.cell_size
    }
}

impl Default for PositionGrid {
    fn default() -> Self {
        PositionGrid::new((1.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::{Vec2};
    use assert_approx_eq::assert_approx_eq;

    use super::PositionGrid;

    #[test]
    fn cell_center() {
        let grid = PositionGrid::default();
        let (x,y) = grid.cell_to_cell_center((10,15)).into();

        assert_approx_eq!(10.5, x);
        assert_approx_eq!(15.5, y);
    }

    #[test]
    fn to_cell() {
        let grid = PositionGrid::default();
        let (x,y) = grid.to_cell((10.5, 11.12)).into();
        assert_eq!(x, 10);
        assert_eq!(y, 11);

        let (x,y) = grid.to_cell((-5.1, -12.3)).into();
        assert_eq!(x, -6);
        assert_eq!(y, -13);
    }

    #[test]
    fn snap() {
        let grid = PositionGrid::default();
        let world = Vec2::new(15.5, -10.12);

        let snapped = grid.pos_snap(world.into());

        assert_approx_eq!(snapped.x, 15.0);
        assert_approx_eq!(snapped.y, -11.0);
    }

    #[test]
    fn cell_size() {
        let grid = PositionGrid::new((2.0, 2.0));
        let p = grid.to_local((0,0));
        assert_approx_eq!(p.x, 5.5);
        assert_approx_eq!(p.y, 6.3);

        let p = grid.to_local((1,2));

        assert_approx_eq!(p.x, 7.5);
        assert_approx_eq!(p.y, 10.3);

        let p = grid.to_local((-1,-1));

        assert_approx_eq!(p.x, 3.5);
    }
}
