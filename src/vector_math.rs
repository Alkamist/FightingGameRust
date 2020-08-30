//use std::ops::{
//    Add, AddAssign,
//    Sub, SubAssign,
//    Mul, MulAssign,
//    Div, DivAssign,
//};

#[derive(Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn set_magnitude(&mut self, value: f64) {
        let magnitude = self.magnitude();
        if magnitude != 0.0 {
            let scale_factor = value / magnitude;
            self.x *= scale_factor;
            self.y *= scale_factor;
        }
    }

    pub fn with_magnitude(&self, value: f64) -> Self {
        let mut new_vector = self.clone();
        new_vector.set_magnitude(value);
        new_vector
    }

    pub fn angle(&self) -> f64 {
        if self.x != 0.0 || self.y != 0.0 {
            self.y.atan2(self.x)
        }
        else {
            0.0
        }
    }

    pub fn direction(&self) -> Self {
        self.clone().with_magnitude(1.0)
    }

    pub fn dot(&self, other_vector: &Self) -> f64 {
        self.x * other_vector.x + self.y * other_vector.y
    }

    pub fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    pub fn inverse(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    pub fn rotate(&mut self, rotation: f64) {
        let rotation_cos = rotation.cos();
        let rotation_sin = rotation.sin();
        let new_x = self.x * rotation_cos - self.y * rotation_sin;
        let new_y = self.x * rotation_sin + self.y * rotation_cos;
        self.x = new_x;
        self.y = new_y;
    }

    pub fn with_rotation(&self, rotation: f64) -> Vector {
        let mut new_vector = self.clone();
        new_vector.rotate(rotation);
        new_vector
    }
}

//impl Add for Vector {
//    type Output = Self;
//
//    fn add(self, other: Self) -> Self {
//        Self {
//            x: self.x + other.x,
//            y: self.y + other.y,
//        }
//    }
//}
//
//impl AddAssign for Vector {
//    fn add_assign(&mut self, other: Self) {
//        self.x += other.x;
//        self.y += other.y;
//    }
//}
//
//impl Sub for Vector {
//    type Output = Self;
//
//    fn sub(self, other: Self) -> Self {
//        Self {
//            x: self.x - other.x,
//            y: self.y - other.y,
//        }
//    }
//}
//
//impl SubAssign for Vector {
//    fn sub_assign(&mut self, other: Self) {
//        self.x -= other.x;
//        self.y -= other.y;
//    }
//}
//
//impl Mul for Vector {
//    type Output = Self;
//
//    fn mul(self, other: Self) -> Self {
//        Self {
//            x: self.x * other.x,
//            y: self.y * other.y,
//        }
//    }
//}
//
//impl MulAssign for Vector {
//    fn mul_assign(&mut self, other: Self) {
//        self.x *= other.x;
//        self.y *= other.y;
//    }
//}
//
//impl Div for Vector {
//    type Output = Self;
//
//    fn div(self, other: Self) -> Self {
//        Self {
//            x: self.x / other.x,
//            y: self.y / other.y,
//        }
//    }
//}
//
//impl DivAssign for Vector {
//    fn div_assign(&mut self, other: Self) {
//        self.x /= other.x;
//        self.y /= other.y;
//    }
//}
