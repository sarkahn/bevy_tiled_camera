use bevy::prelude::*;



struct Rect {
    min: Vec2,
    max: Vec2,
}

impl Rect {
    pub fn center(&self) -> Vec2 {
        self.min + self.max / 2.0
    }

    pub fn from_min_max(min: (f32,f32), max: (f32,f32)) -> Self {
        Rect {
            min: min.into(), 
            max: max.into(),
        }
    }
    
    pub fn from_center_size(center: (f32, f32), size: (f32,f32)) -> Self {
        let size = Vec2::from(size);
        let center = Vec2::from(center);
        let half_size = size / 2.0;
        let min = center - half_size;
        let max = min + size;
        Rect {
            min,
            max
        }
    }
}

fn main() {
    let width = 30.0 as f32;
    let target = 5.0;

    let zoom = (width / target).floor();
}

#[test]
fn center_test() {
    let r = Rect::from_min_max((0.0,0.0), (10.0,10.0));
    assert_eq!(r.center(), (5.0,5.0).into() );
}