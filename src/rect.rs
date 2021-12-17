use std::fmt::Display;

use bevy::{math::{UVec2, Vec2, IVec2}, prelude::Component};

/// A rectangle on a grid.
/// 
/// Points contained in the rect can be iterated over. 
#[derive(Default, Component)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    /// Construct a rect from it's position and size.
    pub fn from_position_size(pos: (f32,f32), size: (f32,f32)) -> Self {
        let pos = Vec2::from(pos);
        let size = Vec2::from(size);
        Rect {
            min: pos,
            max: pos + size
        }
    }

    pub fn from_grid_position_size(grid_pos: (i32,i32), grid_size: (u32,u32)) -> Self {
        let pos = IVec2::from(grid_pos);
        let size = UVec2::from(grid_size);
        Rect {
            min: pos.as_vec2(),
            max: (pos + size.as_ivec2()).as_vec2()
        }
    } 

    /// Construct a rect from it's min and max extents.
    pub fn from_extents(min: (f32,f32), max: (f32,f32)) -> Self {
        Rect {
            min: Vec2::from(min),
            max: Vec2::from(max),
        }
    }

    /// Construct a rect from it's min and max extents.
    pub fn from_grid_extents(min: (i32,i32), max: (i32,i32)) -> Self {
        Rect {
            min: IVec2::from(min).as_vec2(),
            max: IVec2::from(max).as_vec2(),
        }
    }

    pub fn from_center_size(center: (f32,f32), size: (f32,f32)) -> Self {
        let size = Vec2::from(size);
        let min = size / 2.0;
        let max = min + size;
        Rect {
            min,
            max
        }
    }

    pub fn from_grid_center_size(center: (i32,i32), size: (u32,u32)) -> Self { 
        let size = UVec2::from(size).as_ivec2();
        let min = size / 2;
        let max = min + size;
        Rect {
            min: min.as_vec2(),
            max: max.as_vec2(),
        }
    }


    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn grid_size(&self) -> UVec2 {
        self.size().floor().as_uvec2()
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn grid_width(&self) -> u32 {
        self.width().floor() as u32
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn grid_height(&self) -> u32 {
        self.height().floor() as u32
    }

    pub fn set_size(&mut self, new_size: (f32,f32)) {
        let new_size = Vec2::from(new_size);
        self.max = self.min + new_size;
    }

    pub fn position(&self) -> Vec2 {
        self.min
    }

    pub fn grid_position(&self) -> (u32,u32) {
        self.position().floor().as_uvec2().into()
    }

    pub fn grid_min(&self) -> IVec2 {
        self.min.floor().as_ivec2()
    }

    pub fn grid_max(&self) -> IVec2 {
        self.max.floor().as_ivec2()
    }

    pub fn set_position(&mut self, new_pos: (f32,f32)) {
        let new_pos = Vec2::from(new_pos);
        let size = Vec2::from(self.size());
        self.min = new_pos;
        self.max = new_pos + size;
    }

    pub fn set_grid_position(&mut self, new_pos: (i32,i32)) {
        self.set_position(IVec2::from(new_pos).as_vec2().into())
    }

    pub fn center(&self) -> (f32,f32) {
        let size = Vec2::from(self.size());
        (self.min + size / 2.0).into()
    }

    pub fn grid_center(&self) -> (i32,i32) {
        let size = UVec2::from(self.grid_size()).as_ivec2();
        (self.grid_min() + size / 2).into()
    }

    /// Move the rect's center without affecting it's position or size.
    pub fn set_center(&mut self, pos: (f32,f32)) {
        let pos = Vec2::from(pos);
        let size = Vec2::from(self.size());
        let pos = pos - size / 2.0;
        self.set_position(pos.into());
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        let min = self.min;
        let max = self.max;
        !(max.cmplt(other.min).any() || min.cmpgt(other.max).any())
    }

    pub fn grid_overlaps(&self, other: &Rect) -> bool {
        let min = self.grid_min();
        let max = self.grid_max();
        !(max.cmplt(other.grid_min()).any() || min.cmpgt(other.grid_max()).any())
    }

    /// An iterator over all grid positions contained in the rect.
    pub fn iter(&self) -> RectGridIterator {
        RectGridIterator::from_rect(self)
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = self.width();
        let h = self.height();
        let grid_min = self.grid_min();
        let grid_max = self.grid_max();
        let grid_size = self.grid_size();
        write!(f, "Rect[pos({},{}) size({},{}), Grid[pos({},{}) size({},{})]]", 
        self.min.x, self.min.y, w, h, grid_min.x, grid_min.y, grid_size.x, grid_size.y)
    }
}

pub struct RectGridIterator {
    min: IVec2,
    width: u32,
    current: u32,
    length: u32,
}

impl RectGridIterator {
    pub fn from_rect(rect: &Rect) -> Self {
        let min = rect.grid_min();
        let size = rect.grid_size();
        RectGridIterator {
            min: rect.grid_min(),
            width: size.x as u32,
            current: 0,
            length: (size.x * size.y) as u32,
        }
    }
}

impl Iterator for RectGridIterator {
    type Item = (i32,i32);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.current..self.length {
            self.current += 1;

            let xy = IVec2::new(
                (i % self.width) as i32,
                (i / self.width) as i32
            );

            return Some((self.min + xy).into())
        }
        None
    }
}

pub struct RectIterator {
    min: Vec2,
    width: u32,
    current: u32,
    length: u32,
}
impl RectIterator {
    pub fn from_rect(rect: &Rect) -> Self {
        let min = rect.min;
        let size = rect.grid_size();
        RectIterator {
            min,
            width: size.x as u32,
            current: 0,
            length: (size.x * size.y) as u32,
        }
    }
}

impl Iterator for RectIterator {
    type Item = (f32,f32);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.current..self.length {
            self.current += 1;

            let xy = IVec2::new(
                (i % self.width) as i32,
                (i / self.width) as i32
            );

            return Some((self.min + xy.as_vec2()).into())
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::Rect;

    #[test]
    fn init() {
        let rect = Rect::from_grid_position_size((5,5), (5,5));
        assert_eq!((5,5), rect.grid_position().into());
        assert_eq!((5,5), rect.grid_size().into());
    }

    #[test]
    fn iterator() {
        let size = 10i32;
        let rect = Rect::from_grid_position_size((0,0), (size as u32, size as u32));

        let points: Vec<(i32,i32)> = rect.iter().collect();

        for x in 0..size {
            for y in 0..size {
                assert!(points.contains(&(x,y)));
            }
        }

        assert_eq!(size * size, points.len() as i32);
    }

    #[test]
    fn overlap() {
        let r1 = Rect::from_grid_extents( (0,0), (10,10) );
        let r2 = Rect::from_grid_extents( (5,5), (10,10) );
        let r3 = Rect::from_grid_extents( (100,100), (10,10) );

        assert!(  r1.grid_overlaps(&r2) );
        assert!( !r1.grid_overlaps(&r3) );
        assert!(  r1.grid_overlaps(&r1) );

        let r1 = Rect::from_grid_extents( (0,0), (5,5) );
        let r2 = Rect::from_grid_extents( (6,6), (10,10) );

        assert!( !r1.overlaps(&r2) );

        let r1 = Rect::from_grid_position_size( (24,12), (6,8) );
        let r2 = Rect::from_grid_position_size( (6,31), (9,7) );

        assert!( !r1.grid_overlaps(&r2) );
    }

    // #[test]
    // fn set_center() {
    //     let mut r = Rect::from_grid_position_size((0,0), (10,10));

    //     r.set_grid_center((30,30));

    //     assert_eq!((30,30), r.center());
    //     assert_eq!((25,25), r.min.into());
    //     assert_eq!((35,35), r.max.into());
    //     assert_eq!((10,10), r.size());
    // }
}