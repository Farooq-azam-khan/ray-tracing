use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

type Color = Vec3;
type Point = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn r(&self) -> f64 {
        self.e[0]
    }

    pub fn g(&self) -> f64 {
        self.e[1]
    }

    pub fn b(&self) -> f64 {
        self.e[2]
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()],
        }
    }
}

// addition
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        };
    }
}

// multiplication
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [rhs.x() * self, rhs.y() * self, rhs.z() * self],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            e: [self.x() * t, self.y() * t, self.z() * t],
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}
// impl ops::Mul<Vec3> for Vec3 {
//     type Output = Vec3;
//     fn mul(self, rhs: Vec3) -> Vec3 {
//         Vec3 {
//             e: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * self.z()],
//         }
//     }
// }

// Division
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

// impl ops::Div<Vec3> for f64 {
//     type Output = Vec3;
//     fn div(self, v: Vec3) -> Vec3 {
//         v * (1.0 / self) // brackets matter (got sent into infinite loop)
//     }
// }
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        let reciprocal: f64 = 1.0 / rhs;
        Vec3::new(
            self.x() * reciprocal,
            self.y() * reciprocal,
            self.z() * reciprocal,
        )
    }
}
// impl ops::Sub<Vec3> for Vec3 {
//     type Output = Vec3;
//     fn sub(self, rhs: Vec3) -> Vec3 {
//         Vec3 {
//             e: [
//                 self.e[0] - rhs.e[0],
//                 self.e[1] - rhs.e[1],
//                 self.e[2] - rhs.e[2],
//             ],
//         }
//     }
// }

#[test]
fn create_vec3_struct() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x(), 1.0);
    assert_eq!(v.y(), 2.0);
    assert_eq!(v.z(), 3.0);
}

#[test]
fn create_color_struct() {
    let c = Color::new(255.0, 250.0, 10.0);
    assert_eq!(c.r(), 255.0);
    assert_eq!(c.g(), 250.0);
    assert_eq!(c.b(), 10.0);
}

#[test]
fn create_point_struct() {
    let p = Point::new(0.5, 1.0, 0.25);
    assert_eq!(p.x(), 0.5);
    assert_eq!(p.y(), 1.0);
    assert_eq!(p.z(), 0.25);
}

#[test]
fn negate_vector() {
    let v = Vec3::new(1.0, -1.0, 1.0);
    let negv = -v;
    assert_eq!(negv.x(), -1.0);
    assert_eq!(negv.y(), 1.0);
    assert_eq!(negv.z(), -1.0);
}

#[test]
fn add_two_vectors() {
    let v1 = Vec3::new(1.0, 1.0, 1.0);
    let v2 = Vec3::new(2.0, 2.0, 2.0);
    let v3 = v1 + v2;
    assert_eq!(v3.x(), 3.0);
    assert_eq!(v3.y(), 3.0);
    assert_eq!(v3.z(), 3.0);
}

#[test]
fn test_inplace_addition() {
    let mut v1 = Vec3::new(1.0, 1.0, 1.0);
    let v2 = Vec3::new(2.0, 2.0, 2.0);
    v1 += v2;
    assert_eq!(v1.x(), 3.0);
    assert_eq!(v1.y(), 3.0);
    assert_eq!(v1.z(), 3.0);
}

#[test]
fn multiply_contant() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let vt = 2.0 * v;
    assert_eq!(vt.x(), 2.0);
    assert_eq!(vt.y(), 4.0);
    assert_eq!(vt.z(), 6.0);

    let v = Vec3::new(1.0, 2.0, 3.0);
    let vt = v * 2.0;
    assert_eq!(vt.x(), 2.0);
    assert_eq!(vt.y(), 4.0);
    assert_eq!(vt.z(), 6.0);
}

#[test]
fn test_inplace_contant_mult() {
    let mut v = Vec3::new(1.0, 2.0, 3.0);
    v *= 2.0;
    assert_eq!(v.x(), 2.0);
    assert_eq!(v.y(), 4.0);
    assert_eq!(v.z(), 6.0);
}

#[test]
fn test_inplace_division_by_constant() {
    let mut v = Vec3::new(1.0, 2.0, 3.0);
    v /= 1.0;
    assert_eq!(v.x(), 1.0);
    assert_eq!(v.y(), 2.0);
    assert_eq!(v.z(), 3.0);
}

#[test]
fn test_division_by_constant() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let vd = v / 1.0;
    assert_eq!(vd.x(), 1.0);
    assert_eq!(vd.y(), 2.0);
    assert_eq!(vd.z(), 3.0);
}

#[test]
fn test_vector_length() {
    let v = Vec3::new(3.0, 4.0, 5.0);
    let length = v.length();
    let expected = 7.07;
    let treashold = 0.01;
    assert!((length - expected).abs() <= treashold);
}

#[test]
fn test_length_squared() {
    let v = Vec3::new(3.0, 4.0, 5.0);
    let len_squared = v.length_squared();
    assert_eq!(len_squared, 50.0);
}
