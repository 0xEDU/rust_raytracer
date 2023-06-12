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

/* Operator overloads for Color ============================================ */
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
/* ========================================================================= */
