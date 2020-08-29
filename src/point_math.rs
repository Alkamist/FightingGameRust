#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(PartialEq)]
pub enum PointOrientation {
    Colinear,
    Clockwise,
    CounterClockwise,
}

pub fn triplet_point_orientation(p: Point, q: Point, r: Point) -> PointOrientation {
    let value = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if value == 0.0 {
        PointOrientation::Colinear
    }
    else if value > 0.0 {
        PointOrientation::Clockwise
    }
    else {
        PointOrientation::CounterClockwise
    }
}
