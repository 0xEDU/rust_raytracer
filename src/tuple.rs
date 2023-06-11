use std::ops;

/* Tuple/Point/Vector declaration and implementation ======================= */
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

/* ========================================================================= */

/* Operator overloads for Tuple ============================================ */
impl std::cmp::PartialEq<Tuple> for Tuple {
    fn eq(&self, rhs: &Self) -> bool {
        self.x.to_bits() == rhs.x.to_bits()
        && self.y.to_bits() == rhs.y.to_bits()
        && self.z.to_bits() == rhs.z.to_bits()
        && self.w.to_bits() == rhs.w.to_bits()
    }
}

impl std::cmp::Eq for Tuple {}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple { x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
        }
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple { x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
                w: self.w - rhs.w,
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: if self.w == 0.0 {self.w} else {-self.w},
        }
    }
}
/* ========================================================================= */

/* Factory functions ======================================================= */
pub fn point(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z, w: 1.0 }
}

pub fn vector(x: f64, y: f64, z: f64) -> Vector {
    Vector { x, y, z, w: 0.0 }
}
/* ========================================================================= */
