use crate::math::Vector2;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    #[inline]
    pub fn new2d(x: f64, y: f64) -> Self {
        Self::new(x, y, 0.0)
    }

    #[inline]
    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn from_vector(v: &Vector2) -> Self {
        Self::new(v.x, v.y, 0.0)
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.z
    }

    #[inline]
    pub fn distance_to(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
    
    #[inline]
    pub fn to_vector2(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
    
    #[inline]
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    
    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    #[inline]
    pub fn cross(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }
    
    #[inline]
    pub fn subtract(&self, other: &Self) -> Vector2 {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
    
    #[inline]
    pub fn add_vector(&self, v: &Vector2) -> Point {
        Point::new(self.x + v.x, self.y + v.y, self.z)
    }
    
    #[inline]
    pub fn scale(&self, s: f64) -> Point {
        Point::new(self.x * s, self.y * s, self.z * s)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({:.6}, {:.6}, {:.6})", self.x, self.y, self.z)
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        if scalar.abs() < 1e-10 {
            self
        } else {
            Point::new(self.x / scalar, self.y / scalar, self.z / scalar)
        }
    }
}
