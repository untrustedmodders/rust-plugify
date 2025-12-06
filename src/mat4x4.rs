use std::fmt;
use std::ops::{Add, Sub, Mul, Index};
use crate::vec4::Vector4;

// Matrix4x4
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn new(elements: [[f32; 4]; 4]) -> Self {
        Self { m: elements }
    }

    pub fn add(&self, matrix: Matrix4x4) -> Matrix4x4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.m[i][j] + matrix.m[i][j];
            }
        }
        Matrix4x4 { m: result }
    }

    pub fn subtract(&self, matrix: Matrix4x4) -> Matrix4x4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.m[i][j] - matrix.m[i][j];
            }
        }
        Matrix4x4 { m: result }
    }

    pub fn multiply(&self, matrix: Matrix4x4) -> Matrix4x4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.m[i][k] * matrix.m[k][j];
                }
            }
        }
        Matrix4x4 { m: result }
    }

    pub fn multiply_vector(&self, vector: Vector4) -> Vector4 {
        Vector4 {
            x: self.m[0][0] * vector.x + self.m[0][1] * vector.y + self.m[0][2] * vector.z + self.m[0][3] * vector.w,
            y: self.m[1][0] * vector.x + self.m[1][1] * vector.y + self.m[1][2] * vector.z + self.m[1][3] * vector.w,
            z: self.m[2][0] * vector.x + self.m[2][1] * vector.y + self.m[2][2] * vector.z + self.m[2][3] * vector.w,
            w: self.m[3][0] * vector.x + self.m[3][1] * vector.y + self.m[3][2] * vector.z + self.m[3][3] * vector.w,
        }
    }

    pub fn transpose(&self) -> Matrix4x4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[j][i] = self.m[i][j];
            }
        }
        Matrix4x4 { m: result }
    }

    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn zero() -> Self {
        Self {
            m: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }

    pub fn scaling(sx: f32, sy: f32, sz: f32) -> Self {
        Self {
            m: [
                [sx, 0.0, 0.0, 0.0],
                [0.0, sy, 0.0, 0.0],
                [0.0, 0.0, sz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translation(tx: f32, ty: f32, tz: f32) -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, tx],
                [0.0, 1.0, 0.0, ty],
                [0.0, 0.0, 1.0, tz],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, c, -s, 0.0],
                [0.0, s, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            m: [
                [c, 0.0, s, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-s, 0.0, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_z(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            m: [
                [c, -s, 0.0, 0.0],
                [s, c, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl Add for Matrix4x4 {
    type Output = Matrix4x4;

    fn add(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.m[i][j] + other.m[i][j];
            }
        }
        Matrix4x4 { m: result }
    }
}

impl Sub for Matrix4x4 {
    type Output = Matrix4x4;

    fn sub(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.m[i][j] - other.m[i][j];
            }
        }
        Matrix4x4 { m: result }
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }
        Matrix4x4 { m: result }
    }
}

impl Mul<Vector4> for Matrix4x4 {
    type Output = Vector4;

    fn mul(self, vector: Vector4) -> Vector4 {
        Vector4 {
            x: self.m[0][0] * vector.x + self.m[0][1] * vector.y + self.m[0][2] * vector.z + self.m[0][3] * vector.w,
            y: self.m[1][0] * vector.x + self.m[1][1] * vector.y + self.m[1][2] * vector.z + self.m[1][3] * vector.w,
            z: self.m[2][0] * vector.x + self.m[2][1] * vector.y + self.m[2][2] * vector.z + self.m[2][3] * vector.w,
            w: self.m[3][0] * vector.x + self.m[3][1] * vector.y + self.m[3][2] * vector.z + self.m[3][3] * vector.w,
        }
    }
}

impl Index<usize> for Matrix4x4 {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &[f32; 4] {
        &self.m[index]
    }
}

impl fmt::Display for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix4x4[{}, {}, {}, {}]\n[{}, {}, {}, {}]\n[{}, {}, {}, {}]\n[{}, {}, {}, {}]",
            self.m[0][0], self.m[0][1], self.m[0][2], self.m[0][3],
            self.m[1][0], self.m[1][1], self.m[1][2], self.m[1][3],
            self.m[2][0], self.m[2][1], self.m[2][2], self.m[2][3],
            self.m[3][0], self.m[3][1], self.m[3][2], self.m[3][3],
        )
    }
}