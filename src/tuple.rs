pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub type Point = Tuple;
pub type Vector = Tuple;

impl Tuple {
    pub fn is_vector(&self) -> bool {
        let result = if self.w == 0.0 {true} else {false};
        result
    }

    pub fn is_point(&self) -> bool {
        let result = if self.w == 1.0 {true} else {false};
        result
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z, w: 1.0 }
}

pub fn vector(x: f64, y: f64, z: f64) -> Vector {
    Vector { x, y, z, w: 0.0 }
}
