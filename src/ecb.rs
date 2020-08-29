use crate::point_math::Point;

pub struct ECB {
    pub bottom: Point,
    pub left: Point,
    pub top: Point,
    pub right: Point,
}

//impl ECB {
//    pub fn get_ground_line_collision_position(
//        &self,
//        position_previous: Point2D,
//        position: Point2D,
//        ground_line: LineSegment2D,
//    ) -> Option<Point2D> {
//
//        let tolerance = 0.01;
//
//        let ecb_bottom_x_previous = self.bottom.x() + position_previous.x();
//        let ecb_bottom_y_previous = self.bottom.y() + position_previous.y();
//        let ecb_bottom_x = self.bottom.x() + position.x();
//        let ecb_bottom_y = self.bottom.y() + position.y();
//
//        let movement_vector = Vector2D::new(ecb_bottom_x - ecb_bottom_x_previous, ecb_bottom_y - ecb_bottom_y_previous);
//        let movement_vector_direction = movement_vector.direction();
//
//        // Extend the movement line backward by some tolerance to prevent,
//        // moving through obstacles.
//        let movement_line_start_point = Point2D::new(
//            ecb_bottom_x_previous - movement_vector_direction.x() * tolerance,
//            ecb_bottom_y_previous - movement_vector_direction.y() * tolerance,
//        );
//
//        let movement_line = LineSegment2D::new(
//            movement_line_start_point,
//            Point2D::new(ecb_bottom_x, ecb_bottom_y),
//        );
//        if movement_line.intersects_with(ground_line) {
//            movement_line.get_intersection(ground_line)
//        }
//        else { None }
//    }
//}

//pub struct CollisionLine {
//    line: LineSegment2D,
//}
//
//impl CollisionLine {
//    pub fn new(line: LineSegment2D) -> CollisionLine {
//        CollisionLine {
//            line,
//        }
//    }
//
//    pub fn line(&self) -> LineSegment2D { self.line }
//    pub fn is_ground(&self) -> bool {
//        self.line.slope().abs() < 1.0
//    }
//}
//
//pub struct CollisionPolyLine {
//    points: Vec<Point2D>,
//    lines: Vec<CollisionLine>,
//}
//
//impl CollisionPolyLine {
//    pub fn new(points: Vec<Point2D>) -> CollisionPolyLine {
//        let mut new_poly_line = CollisionPolyLine {
//            points,
//            lines: Vec::new(),
//        };
//        new_poly_line.create_lines_from_points();
//        new_poly_line
//    }
//
//    pub fn points(&self) -> &Vec<Point2D> { &self.points }
//    pub fn collision_lines(&self) -> &Vec<CollisionLine> { &self.lines }
//
//    fn create_lines_from_points(&mut self) {
//        self.lines.clear();
//        let num_points = self.points.len();
//        if num_points > 1 {
//            for i in 1..num_points {
//                let i_previous = i - 1;
//                let collision_line = CollisionLine::new(
//                    LineSegment2D::new(
//                        Point2D::new(self.points[i_previous].x(), self.points[i_previous].y()),
//                        Point2D::new(self.points[i].x(), self.points[i].y()),
//                    ),
//                );
//                self.lines.push(collision_line);
//            }
//        }
//    }
//}
