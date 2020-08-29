use crate::point_math;
use crate::vector_math;

#[derive(Copy, Clone)]
pub struct LineSegment {
    pub point_a: point_math::Point,
    pub point_b: point_math::Point,
}

pub fn line_segment_left_point(line_segment: LineSegment) -> point_math::Point {
    if line_segment.point_a.x <= line_segment.point_b.x {
        line_segment.point_a
    }
    else {
        line_segment.point_b
    }
}

pub fn line_segment_right_point(line_segment: LineSegment) -> point_math::Point {
    if line_segment.point_a.x > line_segment.point_b.x {
        line_segment.point_a
    }
    else {
        line_segment.point_b
    }
}

pub fn line_segments_are_parallel(line_a: LineSegment, line_b: LineSegment) -> bool {
    line_segment_slope(line_a) == line_segment_slope(line_b)
}

pub fn line_segment_slope(line_segment: LineSegment) -> f64 {
    let left_point = line_segment_left_point(line_segment);
    let right_point = line_segment_right_point(line_segment);
    (right_point.y - left_point.y) / (right_point.x - left_point.x)
}

pub fn line_segment_y_intercept(line_segment: LineSegment) -> f64 {
    let left_point = line_segment_left_point(line_segment);
    left_point.y - line_segment_slope(line_segment) * left_point.x
}

pub fn line_segment_length(line_segment: LineSegment) -> f64 {
    ((line_segment.point_b.x - line_segment.point_a.x).powi(2) + (line_segment.point_b.y - line_segment.point_a.y).powi(2)).sqrt()
}

pub fn line_segment_direction(line_segment: LineSegment) -> vector_math::Vector {
    let length = line_segment_length(line_segment);
    if length > 0.0 {
        vector_math::Vector {
            x: (line_segment.point_b.x - line_segment.point_a.x) / length,
            y: (line_segment.point_b.y - line_segment.point_a.y) / length,
        }
    }
    else {
        vector_math::Vector {
            x: 0.0,
            y: 0.0,
        }
    }
}

// This normal is the top normal if the line goes from left to right,
// but the bottom normal if the line goes from right to left.
pub fn line_segment_normal(line_segment: LineSegment) -> vector_math::Vector {
    let length = line_segment_length(line_segment);
    if length > 0.0 {
        vector_math::Vector {
            x: (line_segment.point_a.y - line_segment.point_b.y) / length,
            y: (line_segment.point_b.x - line_segment.point_a.x) / length,
        }
    }
    else {
        vector_math::Vector {
            x: 0.0,
            y: 0.0,
        }
    }
}

pub fn line_segment_contains_colinear_point(line: LineSegment, point: point_math::Point) -> bool {
    point.x <= line.point_a.x.max(line.point_b.x)
    && point.x >= line.point_a.x.min(line.point_b.x)
    && point.y <= line.point_a.y.max(line.point_b.y)
    && point.y >= line.point_a.y.min(line.point_b.y)
}

pub fn line_segment_contains_point(line: LineSegment, point: point_math::Point) -> bool {
    let point_orientation = point_math::triplet_point_orientation(point, line_segment_left_point(line), line_segment_right_point(line))
    if point_orientation == point_math::PointOrientation::Colinear {
        line_segment_contains_colinear_point(line, point)
    }
    else {
        false
    }
}

// https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
pub fn line_segments_intersect(line_a: LineSegment, line_b: LineSegment) -> bool {
    let orientation0 = point_math::triplet_point_orientation(line_a.point_a, line_a.point_b, line_b.point_a);
    let orientation1 = point_math::triplet_point_orientation(line_a.point_a, line_a.point_b, line_b.point_b);
    let orientation2 = point_math::triplet_point_orientation(line_b.point_a, line_b.point_b, line_a.point_a);
    let orientation3 = point_math::triplet_point_orientation(line_b.point_a, line_b.point_b, line_a.point_b);

    if orientation0 != orientation1 && orientation2 != orientation3 {
        true
    }
    else if orientation0 == point_math::PointOrientation::Colinear
         && line_segment_contains_colinear_point(line_a, line_b.point_a) {
        true
    }
    else if orientation1 == point_math::PointOrientation::Colinear
         && line_segment_contains_colinear_point(line_a, line_b.point_b) {
        true
    }
    else if orientation2 == point_math::PointOrientation::Colinear
         && line_segment_contains_colinear_point(line_b, line_a.point_a) {
        true
    }
    else if orientation3 == point_math::PointOrientation::Colinear
         && line_segment_contains_colinear_point(line_b, line_a.point_b) {
        true
    }
    else {
        false
    }
}

pub fn line_segment_intersection(line_a: LineSegment, line_b: LineSegment) -> Option<point_math::Point> {
    if let Some(intersection) = line_segment_intersection_with_possible_nan(line_a, line_b) {
        if !intersection.x.is_nan() && !intersection.y.is_nan() {
            return Some(intersection)
        }
    }
    None
}

fn line_segment_intersection_with_possible_nan(line_a: LineSegment, line_b: LineSegment) -> Option<point_math::Point> {
    let line_a_left_point = line_segment_left_point(line_a);
    let line_a_y_intercept = line_segment_y_intercept(line_a);
    let line_a_slope = line_segment_slope(line_a);
    let line_b_left_point = line_segment_left_point(line_b);
    let line_b_y_intercept = line_segment_y_intercept(line_b);
    let line_b_slope = line_segment_slope(line_b);

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
    if line_a_slope.is_nan() && line_segment_contains_point(line_b, line_a_left_point) {
        return Some(
            point_math::Point {
                x: line_a_left_point.x,
                y: line_a_left_point.y,
            }
        )
    }

    // Line B is just a point and is on line A.
    if line_b_slope.is_nan() && line_segment_contains_point(line_a, line_b_left_point)  {
        return Some(
            point_math::Point {
                x: line_b_left_point.x,
                y: line_b_left_point.y,
            }
        )
    }

    if !line_segments_are_parallel(line_a, line_b) {
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
