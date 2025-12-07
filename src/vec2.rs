use std::fmt;
use std::ops::{Add, Sub, Mul, Index, IndexMut};

// Vector2
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn add(&self, vector: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }

    pub fn subtract(&self, vector: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - vector.x,
            y: self.y - vector.y,
        }
    }

    pub fn scale(&self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vector2 {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            return Vector2::zero();
        }
        self.scale(1.0 / magnitude)
    }

    pub fn dot(&self, vector: Vector2) -> f32 {
        self.x * vector.x + self.y * vector.y
    }

    pub fn distance_to(&self, vector: Vector2) -> f32 {
        let dx = self.x - vector.x;
        let dy = self.y - vector.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn unit() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Index<usize> for Vector2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds for Vector2"),
        }
    }
}

impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds for Vector2"),
        }
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector2({}, {})", self.x, self.y)
    }
}