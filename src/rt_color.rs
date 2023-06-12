use std::ops;

/* RTColor declaration and implementation ================================== */
pub struct RTColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub fn color(r: f64, g: f64, b: f64) -> RTColor {
    RTColor { r, g, b }
}
/* ========================================================================= */

/* Operator overloads for RTColor ========================================== */
impl std::cmp::PartialEq<RTColor> for RTColor {
    fn eq(&self, rhs: &Self) -> bool {
        self.r.to_bits() == rhs.r.to_bits()
        && self.g.to_bits() == rhs.g.to_bits()
        && self.b.to_bits() == rhs.b.to_bits()
    }
}

impl std::cmp::Eq for RTColor {}

impl ops::Add<RTColor> for RTColor {
    type Output = RTColor;

    fn add(self, rhs: RTColor) -> Self::Output {
        RTColor {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::Sub<RTColor> for RTColor {
    type Output = RTColor;

    fn sub(self, rhs: RTColor) -> Self::Output {
        RTColor {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl ops::Mul<RTColor> for RTColor {
    type Output = RTColor;

    fn mul(self, rhs: RTColor) -> Self::Output {
        RTColor {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
/* ========================================================================= */

/* Operations with RTColor ================================================= */
pub fn hadamard_product(c1: RTColor, c2: RTColor) -> RTColor {
    color(
        c1.r * c2.r,
        c1.g * c2.g,
        c1.b * c2.b,
    )
}
/* ========================================================================= */
