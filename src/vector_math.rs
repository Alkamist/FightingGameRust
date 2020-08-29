#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

pub fn magnitude(vector: Vector) -> f64 {
    (vector.x.powi(2) + vector.y.powi(2)).sqrt()
}

pub fn with_magnitude(vector: Vector, value: f64) -> Vector {
    let current_magnitude = magnitude(vector);
    if current_magnitude != 0.0 {
        let scale_factor = value / current_magnitude;
        Vector {
            x: vector.x * scale_factor,
            y: vector.y * scale_factor,
        }
    }
    else {
        vector
    }
}

pub fn angle(vector: Vector) -> f64 {
    if vector.x != 0.0 || vector.y != 0.0 {
        vector.y.atan2(vector.x)
    }
    else {
        0.0
    }
}

pub fn direction(vector: Vector) -> Vector {
    let current_magnitude = magnitude(vector);
    if current_magnitude > 0.0 {
        Vector {
            x: vector.x / current_magnitude,
            y: vector.y / current_magnitude,
        }
    }
    else {
        Vector {
            x: 0.0,
            y: 0.0,
        }
    }
}

pub fn dot(vector_a: Vector, vector_b: Vector) -> f64 {
    vector_a.x * vector_b.x + vector_a.y * vector_b.y
}

pub fn inverse(vector: Vector) -> Vector {
    Vector{ x: -vector.x, y: -vector.y }
}
