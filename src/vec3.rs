use std::fmt;
use std::ops::{Add, Sub, Mul, Index, IndexMut};

// Vector3
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn add(&self, vector: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }

    pub fn subtract(&self, vector: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
        }
    }

    pub fn scale(&self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            return Vector3::zero();
        }
        self.scale(1.0 / magnitude)
    }

    pub fn dot(&self, vector: Vector3) -> f32 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z
    }

    pub fn cross(&self, vector: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * vector.z - self.z * vector.y,
            y: self.z * vector.x - self.x * vector.z,
            z: self.x * vector.y - self.y * vector.x,
        }
    }

    pub fn distance_to(&self, vector: Vector3) -> f32 {
        let dx = self.x - vector.x;
        let dy = self.y - vector.y;
        let dz = self.z - vector.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn unit() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0 }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector3({}, {}, {})", self.x, self.y, self.z)
    }
}