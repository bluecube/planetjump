use std::ops::*;
use std::num::Float;

#[derive(PartialEq,Copy,Debug,Default)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

#[derive(PartialEq,Copy,Debug)]
pub struct BoundingBox {
    pub a: Vector2D,
    pub b: Vector2D
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
       Vector2D { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
       Vector2D { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Neg for Vector2D {
    type Output = Vector2D;

    fn neg(self) -> Vector2D {
       Vector2D { x: -self.x, y: -self.y }
    }
}

impl Mul<f32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, factor: f32) -> Vector2D {
       Vector2D { x: self.x * factor, y: self.y * factor }
    }
}

impl Div<f32> for Vector2D {
    type Output = Vector2D;

    fn div(self, factor: f32) -> Vector2D {
       Vector2D { x: self.x / factor, y: self.y / factor }
    }
}

pub fn dot(a: Vector2D, b: Vector2D) -> f32 {
    a.x * b.x + a.y * b.y
}

impl Vector2D {
    pub fn zero() -> Vector2D {
        Vector2D { x: 0.0, y: 0.0 }
    }
    pub fn len_squared(self) -> f32 {
        dot(self, self)
    }

    pub fn len(self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn normalized2(self) -> (Vector2D, f32) {
        let len = self.len();
        (self / len, len)
    }

    pub fn normalized(self) -> Vector2D {
        self / self.len()
    }

    pub fn min(a: Vector2D, b: Vector2D) -> Vector2D {
        Vector2D { x: if a.x < b.x { a.x } else { b.x },
                   y: if a.y < b.y { a.y } else { b.y } }
    }

    pub fn max(a: Vector2D, b: Vector2D) -> Vector2D {
        Vector2D { x: if a.x > b.x { a.x } else { b.x },
                   y: if a.y > b.y { a.y } else { b.y } }
    }
}

impl BoundingBox {
    pub fn get_center(&self) -> Vector2D {
        (self.a + self.b) * 0.5
    }
}
