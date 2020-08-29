pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn default() -> Vector {
        Vector {
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

    pub fn with_magnitude(&self, value: f64) -> Vector {
        let mut new_vector = Vector {
            x: self.x,
            y: self.y,
        };
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

    pub fn direction(&self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
        }.with_magnitude(1.0)
    }

    pub fn dot(&self, other_vector: &Vector) -> f64 {
        self.x * other_vector.x + self.y * other_vector.y
    }

    pub fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    pub fn inverse(&self) -> Vector {
        Vector{
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
        let mut new_vector = Vector {
            x: self.x,
            y: self.y,
        };
        new_vector.rotate(rotation);
        new_vector
    }
}
