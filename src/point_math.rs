use crate::general_math;

#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn triplet_orientation(&self, q: &Self, r: &Self) -> Orientation {
        let value = (q.y - self.y) * (r.x - q.x) - (q.x - self.x) * (r.y - q.y);
        if value == 0.0 {
            Orientation::Colinear
        }
        else if value > 0.0 {
            Orientation::Clockwise
        }
        else {
            Orientation::CounterClockwise
        }
    }

    pub fn lerp(&self, other_point: &Self, interpolation: f64) -> Self {
        Self {
            x: general_math::lerp(self.x, other_point.x, interpolation),
            y: general_math::lerp(self.y, other_point.y, interpolation),
        }
    }
}

#[derive(PartialEq)]
pub enum Orientation {
    Colinear,
    Clockwise,
    CounterClockwise,
}
