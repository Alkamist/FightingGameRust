#[derive(Copy, Clone)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Point2D { Point2D {x, y} }

    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn set_x(&mut self, value: f64) { self.x = value; }
    pub fn y(&self) -> f64 { self.y }
    pub fn set_y(&mut self, value: f64) { self.y = value; }

    pub fn orientation(&self, q: Point2D, r: Point2D) -> PointOrientation {
        let p = self;
        let value = (q.y() - p.y()) * (r.x() - q.x()) - (q.x() - p.x()) * (r.y() - q.y());
        if value == 0.0 { PointOrientation::Colinear }
        else if value > 0.0 { PointOrientation::Clockwise }
        else { PointOrientation::CounterClockwise }
    }
}

#[derive(PartialEq)]
pub enum PointOrientation {
    Colinear,
    Clockwise,
    CounterClockwise,
}

#[derive(Copy, Clone)]
pub struct LineSegment2D {
    order_is_correct: bool,
    points: [Point2D; 2],
    slope: f64,
    length: f64,
    y_intercept: f64,
}

impl LineSegment2D {
    pub fn new(point0: Point2D, point1: Point2D) -> LineSegment2D {
        let mut line = LineSegment2D {
            order_is_correct: true,
            points: [point0, point1],
            slope: 0.0,
            length: 0.0,
            y_intercept: 0.0,
        };
        line.set(point0, point1);
        line
    }

    pub fn set(&mut self, point0: Point2D, point1: Point2D) {
        self.points[0] = point0;
        self.points[1] = point1;
        self.order_is_correct = self.calculate_order_is_correct();
        self.slope = self.calculate_slope();
        self.length = self.calculate_length();
        self.y_intercept = self.calculate_y_intercept();
    }

    pub fn left_point(&self) -> Point2D { self.points[if self.order_is_correct { 0 } else { 1 }] }
    pub fn right_point(&self) -> Point2D { self.points[if self.order_is_correct { 1 } else { 0 }] }

    pub fn slope(&self) -> f64 { self.slope }
    pub fn y_intercept(&self) -> f64 { self.y_intercept }
    pub fn length(&self) -> f64 { self.length }

    pub fn top_normal(&self) -> Vector2D {
        let left_point = self.left_point();
        let right_point = self.right_point();
        let length = self.length();
        if length > 0.0 {
            Vector2D::new((left_point.y - right_point.y) / length, (right_point.x - left_point.x) / length)
        }
        else {
            Vector2D::new(0.0, 0.0)
        }
    }

    pub fn bottom_normal(&self) -> Vector2D {
        let left_point = self.left_point();
        let right_point = self.right_point();
        let length = self.length();
        if length > 0.0 {
            Vector2D::new((right_point.y - left_point.y) / length, (left_point.x - right_point.x) / length)
        }
        else {
            Vector2D::new(0.0, 0.0)
        }
    }

    pub fn contains_colinear_point(&self, point: Point2D) -> bool {
        let p = self.points[0];
        let r = self.points[1];
        point.x <= p.x.max(r.x) && point.x >= p.x.min(r.x)
        && point.y <= p.y.max(r.y) && point.y >= p.y.min(r.y)
    }

    pub fn contains_point(&self, point: Point2D) -> bool {
        if point.orientation(self.left_point(), self.right_point()) == PointOrientation::Colinear {
            self.contains_colinear_point(point)
        }
        else { false }
    }

    pub fn is_parallel_with(&self, line: LineSegment2D) -> bool {
        self.slope() == line.slope()
    }

    // https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
    pub fn intersects_with(&self, line: LineSegment2D) -> bool {
        let p0 = self.points[0];
        let q0 = self.points[1];
        let p1 = line.points[0];
        let q1 = line.points[1];

        let orientation0 = p0.orientation(q0, p1);
        let orientation1 = p0.orientation(q0, q1);
        let orientation2 = p1.orientation(q1, p0);
        let orientation3 = p1.orientation(q1, q0);

        if orientation0 != orientation1 && orientation2 != orientation3 { true }
        else if orientation0 == PointOrientation::Colinear && self.contains_colinear_point(p1) { true }
        else if orientation1 == PointOrientation::Colinear && self.contains_colinear_point(q1) { true }
        else if orientation2 == PointOrientation::Colinear && line.contains_colinear_point(p0) { true }
        else if orientation3 == PointOrientation::Colinear && line.contains_colinear_point(q0) { true }
        else { false }
    }

    pub fn get_intersection(&self, line: LineSegment2D) -> Option<Point2D> {
        if let Some(intersection) = self.get_intersection_with_possible_nan(line) {
            if intersection.x().is_nan() || intersection.y().is_nan() {
                None
            }
            else {
                Some(intersection)
            }
        }
        else { None }
    }

    fn calculate_y_intercept(&self) -> f64 {
        let left_point = self.left_point();
        left_point.y() - self.slope() * left_point.x()
    }

    fn calculate_slope(&self) -> f64 {
        let left_point = self.left_point();
        let right_point = self.right_point();
        (right_point.y - left_point.y) / (right_point.x - left_point.x)
    }

    fn calculate_length(&self) -> f64 {
        let left_point = self.left_point();
        let right_point = self.right_point();
        ((right_point.x - left_point.x).powi(2) + (right_point.y - left_point.y).powi(2)).sqrt()
    }

    fn calculate_order_is_correct(&self) -> bool {
        if self.points[0].x() <=  self.points[1].x() { true }
        else { false }
    }

    fn get_intersection_with_possible_nan(&self, line: LineSegment2D) -> Option<Point2D> {
        let left_point = self.left_point();
        let self_y_intercept = self.y_intercept();
        let self_slope = self.slope();
        let other_y_intercept = line.y_intercept();
        let other_slope = line.slope();
        let other_left_point = line.left_point();

        // Both lines are just points.
        if self_slope.is_nan() && other_slope.is_nan() {
            // The points are touching.
            if left_point.x() == other_left_point.x()
            && left_point.y() == other_left_point.y() {
                return Some(Point2D::new(left_point.x(), left_point.y()))
            }

            // The points are not touching.
            return None
        }

        // The self line is just a point and is on the other line.
        if self_slope.is_nan() && line.contains_point(left_point) {
            return Some(Point2D::new(left_point.x(), left_point.y()))
        }
        // The other line is just a point and is on the self line.
        else if other_slope.is_nan() && self.contains_point(other_left_point) {
            return Some(Point2D::new(other_left_point.x(), other_left_point.y()))
        }

        if !self.is_parallel_with(line) {
            // The self line is vertical. This guarantees that the other line is not
            // vertical since the two lines are not parallel.
            if self_slope.is_infinite() {
                let intersection_x = self.left_point().x();
                let intersection_y = intersection_x * other_slope + other_y_intercept;
                return Some(Point2D::new(intersection_x, intersection_y))
            }

            // The other line is vertical. This guarantees that the self line is not
            // vertical since the two lines are not parallel.
            if other_slope.is_infinite() {
                let intersection_x = line.left_point().x();
                let intersection_y = intersection_x * self_slope + self_y_intercept;
                return Some(Point2D::new(intersection_x, intersection_y))
            }

            // Both lines are not vertical.
            let intersection_x = (other_y_intercept - self_y_intercept) / (self_slope - other_slope);
            let intersection_y = intersection_x * self_slope + self_y_intercept;
            return Some(Point2D::new(intersection_x, intersection_y))
        }

        // Two parallel lines will never intersect unless they are the same line,
        // in which case they will intersect everywhere.
        None
    }
}

#[derive(Copy, Clone)]
pub struct Vector2D {
    x: f64,
    y: f64,
    magnitude: f64,
    angle: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Vector2D {
        let mut vector = Vector2D {
            x: 0.0,
            y: 0.0,
            magnitude: 0.0,
            angle: 0.0,
        };
        vector.set(x, y);
        vector
    }

    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.magnitude = self.calculate_magnitude();
        self.angle = self.calculate_angle();
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn set_x(&mut self, value: f64) { self.set(value, self.y); }
    pub fn y(&self) -> f64 { self.y }
    pub fn set_y(&mut self, value: f64) { self.set(self.x, value); }

    pub fn magnitude(&self) -> f64 { self.magnitude }

    pub fn set_magnitude(&mut self, value: f64) {
        let current_magnitude = self.magnitude();
        if current_magnitude != 0.0 {
            let scale_factor = value / current_magnitude;
            self.set_x(self.x() * scale_factor);
            self.set_y(self.y() * scale_factor);
        }
    }

    pub fn angle(&self) -> f64 { self.angle }

    pub fn direction(&self) -> Vector2D {
        let magnitude = self.magnitude();
        if magnitude > 0.0 { Vector2D::new(self.x() / magnitude, self.y() / magnitude) }
        else { Vector2D::new(0.0, 0.0) }
    }

    pub fn dot(&self, vector: Vector2D) -> f64 {
        self.x() * vector.x() + self.y() * vector.y()
    }

    fn calculate_magnitude(&self) -> f64 { (self.x().powi(2) + self.y().powi(2)).sqrt() }
    fn calculate_angle(&self) -> f64 {
        if self.x() != 0.0 || self.y() != 0.0 { self.y().atan2(self.x()) }
        else { 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vectors() {
        let vector = Vector2D::new(5.0, 0.0);
        assert_eq!(vector.magnitude(), 5.0);
        assert_eq!(vector.direction().x(), 1.0);

        let vector = Vector2D::new(0.0, 5.0);
        assert_eq!(vector.magnitude(), 5.0);
        assert_eq!(vector.direction().y(), 1.0);

        let vector = Vector2D::new(-5.0, 0.0);
        assert_eq!(vector.magnitude(), 5.0);
        assert_eq!(vector.direction().x(), -1.0);

        let vector = Vector2D::new(0.0, -5.0);
        assert_eq!(vector.magnitude(), 5.0);
        assert_eq!(vector.direction().y(), -1.0);
    }

    #[test]
    fn test_line_y_intercept() {
        let line = LineSegment2D::new(Point2D::new(4.0, 4.0), Point2D::new(10.0, 10.0));
        assert_eq!(line.y_intercept(), 0.0);

        let line = LineSegment2D::new(Point2D::new(4.0, 10.0), Point2D::new(10.0, 16.0));
        assert_eq!(line.y_intercept(), 6.0);

        let line = LineSegment2D::new(Point2D::new(-4.0, -4.0), Point2D::new(-10.0, -10.0));
        assert_eq!(line.y_intercept(), 0.0);
    }

    #[test]
    fn test_line_intersection() {
        let line_a = LineSegment2D::new(Point2D::new(0.0, 0.0), Point2D::new(1.0, 1.0));
        let line_b = LineSegment2D::new(Point2D::new(1.0, 0.0), Point2D::new(0.0, 1.0));
        assert!(line_a.intersects_with(line_b));

        let line_a = LineSegment2D::new(Point2D::new(0.0, 0.0), Point2D::new(1.0, 1.0));
        let line_b = LineSegment2D::new(Point2D::new(2.0, 1.0), Point2D::new(1.0, 2.0));
        assert!(!line_a.intersects_with(line_b));

        let line_a = LineSegment2D::new(Point2D::new(0.0, 0.0), Point2D::new(1.0, 1.0));
        let line_b = LineSegment2D::new(Point2D::new(0.0, -1.0), Point2D::new(-1.0, 0.0));
        assert!(!line_a.intersects_with(line_b));

        let line_a = LineSegment2D::new(Point2D::new(0.0, 0.0), Point2D::new(1.0, 1.0));
        let line_b = LineSegment2D::new(Point2D::new(0.0, 0.0), Point2D::new(1.0, 1.0));
        assert!(line_a.intersects_with(line_b));

        let line_a = LineSegment2D::new(Point2D::new(0.0, 0.0), Point2D::new(10.0, 10.0));
        let line_b = LineSegment2D::new(Point2D::new(0.0, 10.0), Point2D::new(10.0, 0.0));
        let intersection = line_a.get_intersection(line_b);
        assert!(intersection.is_some());
        assert_eq!(intersection.unwrap().x(), 5.0);
        assert_eq!(intersection.unwrap().y(), 5.0);

        let line_a = LineSegment2D::new(Point2D::new(0.0, 0.0), Point2D::new(10.0, 10.0));
        let line_b = LineSegment2D::new(Point2D::new(0.0, 0.0), Point2D::new(10.0, 10.0));
        let intersection = line_a.get_intersection(line_b);
        assert!(intersection.is_none());
    }
}
