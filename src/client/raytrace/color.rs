use std::ops::*;
use std::f32;

#[derive(PartialEq,Clone,Copy,Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
       Color { r: self.r + other.r, g: self.g + other.g, b: self.b + other.b }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
       Color { r: self.r - other.r, g: self.g - other.g, b: self.b - other.b }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn sub(self, factor: f32) -> Color {
       Color { r: self.r * factor, g: self.g * factor, b: self.b * factor }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn sub(self, factor: f32) -> Color {
       Color { r: self.r / factor, g: self.g / factor, b: self.b / factor }
    }
}
