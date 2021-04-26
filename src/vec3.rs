use std::ops; 
// Represents: colors, locations, directions, offsets, etc. 
// Could have an alias for all these so adding color to location is impossible


#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64;3], 
}

impl Vec3 {
    pub fn new(x:f64, y:f64, z:f64) -> Vec3 {
        Vec3 { e:  [x,y,z] }
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
    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v * (1.0 / v.length())
    }

    pub fn cross(u:Vec3, v:Vec3) -> Vec3 {
        Vec3 {
            e: [ u.e[1] * v.e[2] - u.e[2] * v.e[1], 
                 u.e[2] * v.e[0] - u.e[0] * v.e[2], 
                 u.e[0] * v.e[1] - u.e[1] * v.e[0]
              ]
        }
    }
}


impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3; 

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]]
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]]
        };
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3; 
    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0]*t, self.e[1]*t, self.e[2]*t]
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3; 
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0]*rhs.e[0], self.e[1]*rhs.e[1], self.e[2]*self.e[2]]
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3; 
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [rhs.e[0]*self, rhs.e[1]*self, rhs.e[2]*self]
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3; 
    fn div(self, t: f64) -> Vec3 {
        self * (1.0/t)// brackets matter (got sent into infinite loop)
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3; 
    fn div(self, v: Vec3) -> Vec3 {
        v * (1.0/self) // brackets matter (got sent into infinite loop)
    }
}


impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = Self {
            e: [self.e[0]*t, self.e[1]*t, self.e[2]*t]
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3; 
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]]
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*; 
    #[test]
    fn dot_basic() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::dot(v1, v1), 14.0);
    }
    #[test]
    fn sphere() {

    let oc: Vec3 = Vec3::new(0.0,0.0,0.0) - Vec3::new(0.0,0.0,-1.0); 
    assert_eq!(oc.e[2], 1.0);
    let dir = Vec3::new(0.0, 0.0, -1.0); 
    let radius = 0.5; 
    let a: f64 = Vec3::dot(dir, dir);
    assert_eq!(a, 1.0); 
    let b: f64 = 2.0 * Vec3::dot(oc, dir);
    let c: f64 = Vec3::dot(oc, oc) - radius*radius; 
    let discriminant: f64 = b*b - 4.0 * a * c; 
    assert_eq!(discriminant > 0.0, true); 

    }
}