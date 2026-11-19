use std::ops;

// 3-component 64-bit floating point vector
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn from(value: f64) -> Vec3 { Vec3{x: value, y: value, z: value} }

    pub fn sqr_magnitude(&self) -> f64 {
        self.x * self.x +
            self.y * self.y +
            self.z * self.z
    }

    pub fn magnitude(&self) -> f64 { self.sqr_magnitude().sqrt() }

    pub fn normalized(&self) -> Vec3 { *self / self.magnitude() }

    pub fn normalize(&mut self) -> &mut Self { *self = self.normalized(); self }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output { Vec3{x: -self.x, y: -self.y, z: -self.z} }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs
    }
}

pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
    lhs.x * rhs.x +
        lhs.y * rhs.y +
        lhs.z * rhs.z
}

pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3{
        x: lhs.y * rhs.z - lhs.z * rhs.y,
        y: lhs.z * rhs.x - lhs.x * rhs.z,
        z: lhs.x * rhs.y - lhs.y * rhs.x,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqr_magnitude_test() {
        let vec = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec.sqr_magnitude(), 14.0);
    }

    #[test]
    fn magnitude_test() {
        let vec = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec.magnitude(), 14.0f64.sqrt());
    }

    #[test]
    fn normalized_test() {
        let vec = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let sqrt_fourteen = 14.0f64.sqrt();
        assert_eq!(vec.normalized(), Vec3{x: 1.0 / sqrt_fourteen, y: 2.0 / sqrt_fourteen, z: 3.0 / sqrt_fourteen});
    }

    #[test]
    fn normalize_test() {
        let mut vec = Vec3{x: 1.0, y: 2.0, z: 3.0};
        vec.normalize();
        let sqrt_fourteen = 14.0f64.sqrt();
        assert_eq!(vec, Vec3{x: 1.0 / sqrt_fourteen, y: 2.0 / sqrt_fourteen, z: 3.0 / sqrt_fourteen});
    }

    #[test]
    fn negate_test() {
        let vec = Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(-vec, Vec3{x: -1.0, y: -2.0, z: -3.0});
    }

    #[test]
    fn add_test() {
        let lhs = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let rhs = Vec3{ x: 4.0, y: 5.0, z: 6.0, };
        assert_eq!(lhs + rhs, Vec3{x: 5.0, y: 7.0, z: 9.0});
    }

    #[test]
    fn add_assign_test() {
        let mut lhs = Vec3{x: 1.0, y: 2.0, z: 3.0 };
        let rhs = Vec3{ x: 4.0, y: 5.0, z: 6.0, };
        lhs += rhs;
        assert_eq!(lhs, Vec3{x: 5.0, y: 7.0, z: 9.0});
    }

    #[test]
    fn sub_test() {
        let lhs = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let rhs = Vec3{ x: 4.0, y: 5.0, z: 6.0, };
        assert_eq!(lhs - rhs, Vec3{x: -3.0, y: -3.0, z: -3.0});
    }

    #[test]
    fn sub_assign_test() {
        let mut lhs = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let rhs = Vec3{ x: 4.0, y: 5.0, z: 6.0, };
        lhs -= rhs;
        assert_eq!(lhs, Vec3{x: -3.0, y: -3.0, z: -3.0});
    }

    #[test]
    fn mul_test() {
        let lhs = Vec3{x: 1.0, y: 2.0, z: 3.0 };
        let rhs = 2.0;
        assert_eq!(lhs * rhs, Vec3{x: 2.0, y: 4.0, z: 6.0});
    }

    #[test]
    fn mul_assign_test() {
        let mut lhs = Vec3{x: 1.0, y: 2.0, z: 3.0 };
        let rhs = 2.0;
        lhs *= rhs;
        assert_eq!(lhs, Vec3{x: 2.0, y: 4.0, z: 6.0});
    }

    #[test]
    fn div_test() {
        let lhs = Vec3{x: 1.0, y: 2.0, z: 3.0 };
        let rhs = 2.0;
        assert_eq!(lhs / rhs, Vec3{x: 0.5, y: 1.0, z: 1.5});
    }

    #[test]
    fn div_assign_test() {
        let mut lhs = Vec3{x: 1.0, y: 2.0, z: 3.0 };
        let rhs = 2.0;
        lhs /= rhs;
        assert_eq!(lhs, Vec3{x: 0.5, y: 1.0, z: 1.5});
    }

    #[test]
    fn dot_test() {
        let lhs = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let rhs = Vec3{ x: 4.0, y: 5.0, z: 6.0, };
        assert_eq!(dot(&lhs, &rhs), 32.0f64);
    }

    #[test]
    fn cross_test() {
        let lhs = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let rhs = Vec3{ x: 4.0, y: 5.0, z: 6.0, };
        assert_eq!(cross(&lhs, &rhs), Vec3{x: -3.0, y: 6.0, z: -3.0});
    }
}
