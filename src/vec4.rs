use std::fmt;
use std::ops::{Add, Sub, Mul, Index};

// Vector4
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn add(&self, vector: Vector4) -> Vector4 {
        Vector4 {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
            w: self.w + vector.w,
        }
    }

    pub fn subtract(&self, vector: Vector4) -> Vector4 {
        Vector4 {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
            w: self.w - vector.w,
        }
    }

    pub fn scale(&self, scalar: f32) -> Vector4 {
        Vector4 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Vector4 {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            return Vector4::zero();
        }
        self.scale(1.0 / magnitude)
    }

    pub fn dot(&self, vector: Vector4) -> f32 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z + self.w * vector.w
    }

    pub fn distance_to(&self, vector: Vector4) -> f32 {
        let dx = self.x - vector.x;
        let dy = self.y - vector.y;
        let dz = self.z - vector.z;
        let dw = self.w - vector.w;
        (dx * dx + dy * dy + dz * dz + dw * dw).sqrt()
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub fn unit() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }
    }
}

impl Add for Vector4 {
    type Output = Vector4;

    fn add(self, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Vector4 {
    type Output = Vector4;

    fn sub(self, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, scalar: f32) -> Vector4 {
        Vector4 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Index<usize> for Vector4 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds for Vector4"),
        }
    }
}

impl fmt::Display for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}