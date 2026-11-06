use std::ops;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Vec2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y:f64) -> Vec2 {
        Self {
            x, 
            y
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        // divide each part by the magnitude
        *self / self.magnitude()
    }
}

pub fn distance(a: Vec2, b: Vec2) -> f64{
    let delta = a - b;
    delta.magnitude()
}

impl ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul for Vec2 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl ops::Div for Vec2 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, divisor: f64) -> Self::Output {
        Self {
            x: self.x * divisor,
            y: self.y * divisor
        }
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, divisor: f64) -> Self::Output {
        Self {
            x: self.x / divisor,
            y: self.y / divisor
        }
    }

}