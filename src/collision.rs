use crate::game_math::*;

#[derive(Copy, Clone)]
pub struct ECB {
    bottom: Point2D,
    left: Point2D,
    top: Point2D,
    right: Point2D,
}

impl ECB {
    pub fn new(bottom: Point2D, left: Point2D, top: Point2D, right: Point2D) -> ECB {
        ECB {bottom, left, top, right}
    }

    pub fn get_ground_line_collision_position(
        &self,
        position_previous: Point2D,
        position: Point2D,
        ground_line: LineSegment2D,
    ) -> Option<Point2D> {

        let tolerance = 0.01;

        let ecb_bottom_x_previous = self.bottom.x() + position_previous.x();
        let ecb_bottom_y_previous = self.bottom.y() + position_previous.y();
        let ecb_bottom_x = self.bottom.x() + position.x();
        let ecb_bottom_y = self.bottom.y() + position.y();

        let movement_vector = Vector2D::new(ecb_bottom_x - ecb_bottom_x_previous, ecb_bottom_y - ecb_bottom_y_previous);
        let movement_vector_direction = movement_vector.direction();

        // Extend the movement line backward by some tolerance to prevent,
        // moving through obstacles.
        let movement_line_start_point = Point2D::new(
            ecb_bottom_x_previous - movement_vector_direction.x() * tolerance,
            ecb_bottom_y_previous - movement_vector_direction.y() * tolerance,
        );

        let movement_line = LineSegment2D::new(
            movement_line_start_point,
            Point2D::new(ecb_bottom_x, ecb_bottom_y),
        );
        if movement_line.intersects_with(ground_line) {
            movement_line.get_intersection(ground_line)
        }
        else { None }
    }

    pub fn bottom(&self) -> Point2D { self.bottom }
    pub fn left(&self) -> Point2D { self.left }
    pub fn top(&self) -> Point2D { self.top }
    pub fn right(&self) -> Point2D { self.right }
}
