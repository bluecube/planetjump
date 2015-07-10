use std::ops::*;
use std::f32;

#[derive(PartialEq,Clone,Copy,Debug,Default)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
       Vector3D { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
       Vector3D { x: self.x - other.x, y: self.y - other.y , z: self.z - other.z }
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Vector3D {
       Vector3D { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Mul<f32> for Vector3D {
    type Output = Vector3D;

    fn mul(self, factor: f32) -> Vector3D {
       Vector3D { x: self.x * factor, y: self.y * factor, z: self.z * factor }
    }
}

impl Div<f32> for Vector3D {
    type Output = Vector3D;

    fn div(self, factor: f32) -> Vector3D {
       Vector3D { x: self.x / factor, y: self.y / factor, z: self.z / factor}
    }
}

pub fn dot(a: Vector3D, b: Vector3D) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3D {
        Vector3D { x: x, y: y }
    }

    pub fn len_squared(self) -> f32 {
        dot(self, self)
    }

    pub fn len(self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn normalized2(self) -> (Vector3D, f32) {
        let len = self.len();
        (self / len, len)
    }

    pub fn normalized(self) -> Vector3D {
        self / self.len()
    }
}

#[derive(PartialEq,Clone,Debug)]
struct Ray {
    pub origin: Vector3D,
    pub direction: Vector3D
}
