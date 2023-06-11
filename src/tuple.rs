pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn is_vector(&self) -> bool {
        let result = if self.w == 0.0 {true} else {false};
        result
    }

    pub fn is_direction(&self) -> bool {
        let result = if self.w == 1.0 {true} else {false};
        result
    }
}
