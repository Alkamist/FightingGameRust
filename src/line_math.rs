use crate::point_math::{self, Point};
use crate::vector_math::Vector;

#[derive(Clone)]
pub struct LineSegment {
    pub point_a: Point,
    pub point_b: Point,
}

impl LineSegment {
    pub fn default() -> Self {
        Self {
            point_a: Point::default(),
            point_b: Point::default(),
        }
    }

    pub fn left_point(&self) -> &Point {
        if self.point_a.x <= self.point_b.x {
            &self.point_a
        }
        else {
            &self.point_b
        }
    }

    pub fn right_point(&self) -> &Point {
        if self.point_a.x > self.point_b.x {
            &self.point_a
        }
        else {
            &self.point_b
        }
    }

    pub fn slope(&self) -> f64 {
        let left_point = self.left_point();
        let right_point = self.right_point();
        (right_point.y - left_point.y) / (right_point.x - left_point.x)
    }

    pub fn is_parallel_with(&self, other_line: &Self) -> bool {
        self.slope() == other_line.slope()
    }

    pub fn y_intercept(&self) -> f64 {
        let left_point = self.left_point();
        left_point.y - self.slope() * left_point.x
    }

    pub fn length(&self) -> f64 {
        ((self.point_b.x - self.point_a.x).powi(2) + (self.point_b.y - self.point_a.y).powi(2)).sqrt()
    }

    pub fn direction(&self) -> Vector {
        let length = self.length();
        if length > 0.0 {
            Vector {
                x: (self.point_b.x - self.point_a.x) / length,
                y: (self.point_b.y - self.point_a.y) / length,
            }
        }
        else {
            Vector {
                x: 0.0,
                y: 0.0,
            }
        }
    }

    // This normal is the top normal if the line goes from left to right,
    // but the bottom normal if the line goes from right to left.
    pub fn normal(&self) -> Vector {
        let length = self.length();
        if length > 0.0 {
            Vector {
                x: (self.point_a.y - self.point_b.y) / length,
                y: (self.point_b.x - self.point_a.x) / length,
            }
        }
        else {
            Vector {
                x: 0.0,
                y: 0.0,
            }
        }
    }

    pub fn contains_colinear_point(&self, point: &Point) -> bool {
        point.x <= self.point_a.x.max(self.point_b.x)
        && point.x >= self.point_a.x.min(self.point_b.x)
        && point.y <= self.point_a.y.max(self.point_b.y)
        && point.y >= self.point_a.y.min(self.point_b.y)
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        let point_orientation = point.triplet_orientation(self.left_point(), self.right_point());
        if point_orientation == point_math::Orientation::Colinear {
            self.contains_colinear_point(&point)
        }
        else {
            false
        }
    }

    // https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
    pub fn intersects_with(&self, other_line: &Self) -> bool {
        let line_a = self;
        let line_b = other_line;

        let orientation0 = line_a.point_a.triplet_orientation(&line_a.point_b, &line_b.point_a);
        let orientation1 = line_a.point_a.triplet_orientation(&line_a.point_b, &line_b.point_b);
        let orientation2 = line_b.point_a.triplet_orientation(&line_b.point_b, &line_a.point_a);
        let orientation3 = line_b.point_a.triplet_orientation(&line_b.point_b, &line_a.point_b);

        if orientation0 != orientation1 && orientation2 != orientation3 {
            true
        }
        else if orientation0 == point_math::Orientation::Colinear
            && line_a.contains_colinear_point(&line_b.point_a) {
            true
        }
        else if orientation1 == point_math::Orientation::Colinear
            && line_a.contains_colinear_point(&line_b.point_b) {
            true
        }
        else if orientation2 == point_math::Orientation::Colinear
            && line_b.contains_colinear_point(&line_a.point_a) {
            true
        }
        else if orientation3 == point_math::Orientation::Colinear
            && line_b.contains_colinear_point(&line_a.point_b) {
            true
        }
        else {
            false
        }
    }

    pub fn intersection_with(&self, other_line: &Self) -> Option<Point> {
        if let Some(intersection) = self.intersection_with_possible_nan(other_line) {
            if !intersection.x.is_nan() && !intersection.y.is_nan() {
                return Some(intersection)
            }
        }
        None
    }

    fn intersection_with_possible_nan(&self, other_line: &Self) -> Option<point_math::Point> {
        let line_a = self;
        let line_b = other_line;

        let line_a_left_point = line_a.left_point();
        let line_a_y_intercept = line_a.y_intercept();
        let line_a_slope = line_a.slope();
        let line_b_left_point = line_b.left_point();
        let line_b_y_intercept = line_b.y_intercept();
        let line_b_slope = line_b.slope();

        // Both lines are just points.
        if line_a_slope.is_nan() && line_b_slope.is_nan() {
            // The points are touching.
            if line_a_left_point.x == line_b_left_point.x
            && line_a_left_point.y == line_b_left_point.y {
                return Some(
                    point_math::Point {
                        x: line_a_left_point.x,
                        y: line_a_left_point.y,
                    }
                )
            }

            // The points are not touching.
            return None
        }

        // Line A is just a point and is on line B.
        if line_a_slope.is_nan() && line_b.contains_point(line_a_left_point) {
            return Some(
                point_math::Point {
                    x: line_a_left_point.x,
                    y: line_a_left_point.y,
                }
            )
        }

        // Line B is just a point and is on line A.
        if line_b_slope.is_nan() && line_a.contains_point(line_b_left_point)  {
            return Some(
                point_math::Point {
                    x: line_b_left_point.x,
                    y: line_b_left_point.y,
                }
            )
        }

        if !line_a.is_parallel_with(line_b) {
            // Line A is vertical. This guarantees that line B is not vertical since
            // the two lines are not parallel.
            if line_a_slope.is_infinite() {
                let intersection_x = line_a_left_point.x;
                let intersection_y = intersection_x * line_b_slope + line_b_y_intercept;
                return Some(
                    point_math::Point {
                        x: intersection_x,
                        y: intersection_y,
                    }
                )
            }

            // The other line is vertical. This guarantees that the self line is not
            // vertical since the two lines are not parallel.
            if line_b_slope.is_infinite() {
                let intersection_x = line_b_left_point.x;
                let intersection_y = intersection_x * line_a_slope + line_a_y_intercept;
                return Some(
                    point_math::Point {
                        x: intersection_x,
                        y: intersection_y,
                    }
                )
            }

            // Both lines are not vertical.
            let intersection_x = (line_b_y_intercept - line_a_y_intercept) / (line_a_slope - line_b_slope);
            let intersection_y = intersection_x * line_a_slope + line_a_y_intercept;
            return Some(
                point_math::Point {
                    x: intersection_x,
                    y: intersection_y,
                }
            )
        }

        // Two parallel lines will never intersect unless they are the same line,
        // in which case they will intersect everywhere.
        None
    }
}

#[derive(Clone)]
pub struct PolyLine {
    pub segments: Vec<LineSegment>,
}

impl PolyLine {
    pub fn from_points(points: &Vec<Point>) -> Self {
        let mut poly_line = Self {
            segments: Vec::new(),
        };

        let num_points = points.len();
        if num_points > 1 {
            for i in 1..num_points {
                let previous_i = i - 1;
                let segment = LineSegment {
                    point_a: Point {
                        x: points[previous_i].x,
                        y: points[previous_i].y,
                    },
                    point_b: Point {
                        x: points[i].x,
                        y: points[i].y,
                    },
                };
                poly_line.segments.push(segment);
            }
        }

        poly_line
    }
}