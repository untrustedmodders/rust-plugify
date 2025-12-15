use std::fmt;
use std::ops::{Add, Sub, Mul, Neg, Index, IndexMut, AddAssign, SubAssign, MulAssign};
use crate::Vec4;

/// A 4x4 matrix with f32 components
///
/// This type is FFI-compatible with C/C++ due to `#[repr(C)]`.
/// The matrix is stored in row-major order.
///
/// Commonly used for:
/// - 3D transformations (translation, rotation, scale)
/// - Projection matrices (perspective, orthographic)
/// - View matrices (camera transformations)
/// - Homogeneous coordinate transformations
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4x4 {
    /// Matrix elements in row-major order: m[row][column]
    pub m: [[f32; 4]; 4],
}

/// Epsilon for floating point comparisons
const EPSILON: f32 = 1e-6;

impl Mat4x4 {
    /// Create a new matrix from a 2D array
    ///
    /// Elements are provided in row-major order: `[[row0], [row1], [row2], [row3]]`
    #[must_use]
    pub const fn new(elements: [[f32; 4]; 4]) -> Self {
        Self { m: elements }
    }

    /// Create an identity matrix
    ///
    /// The identity matrix has 1s on the diagonal and 0s elsewhere.
    /// Multiplying any matrix by the identity returns the original matrix.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Create a zero matrix (all elements are 0)
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            m: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }

    /// Create a scaling matrix
    ///
    /// This matrix scales along the x, y, and z axes by sx, sy, and sz respectively.
    #[must_use]
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

    /// Create a uniform scaling matrix (same scale on all axes)
    #[must_use]
    pub fn uniform_scaling(s: f32) -> Self {
        Self::scaling(s, s, s)
    }

    /// Create a translation matrix
    ///
    /// This matrix translates by (tx, ty, tz).
    #[must_use]
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

    /// Create a rotation matrix around the X axis
    ///
    /// Angle is in radians. Positive angle rotates counter-clockwise when looking down the axis.
    #[must_use]
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

    /// Create a rotation matrix around the Y axis
    ///
    /// Angle is in radians. Positive angle rotates counter-clockwise when looking down the axis.
    #[must_use]
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

    /// Create a rotation matrix around the Z axis
    ///
    /// Angle is in radians. Positive angle rotates counter-clockwise when looking down the axis.
    #[must_use]
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

    /// Create a rotation matrix around an arbitrary axis
    ///
    /// The axis should be normalized. Angle is in radians.
    #[must_use]
    pub fn rotation_axis(axis: (f32, f32, f32), angle: f32) -> Self {
        let (x, y, z) = axis;
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;

        Self {
            m: [
                [t * x * x + c, t * x * y - s * z, t * x * z + s * y, 0.0],
                [t * x * y + s * z, t * y * y + c, t * y * z - s * x, 0.0],
                [t * x * z - s * y, t * y * z + s * x, t * z * z + c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Create a perspective projection matrix
    ///
    /// # Arguments
    /// * `fov` - Field of view in radians
    /// * `aspect` - Aspect ratio (width / height)
    /// * `near` - Near clipping plane
    /// * `far` - Far clipping plane
    #[must_use]
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov / 2.0).tan();
        let nf = 1.0 / (near - far);

        Self {
            m: [
                [f / aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (far + near) * nf, 2.0 * far * near * nf],
                [0.0, 0.0, -1.0, 0.0],
            ],
        }
    }

    /// Create an orthographic projection matrix
    ///
    /// # Arguments
    /// * `left`, `right` - Left and right clipping planes
    /// * `bottom`, `top` - Bottom and top clipping planes
    /// * `near`, `far` - Near and far clipping planes
    #[must_use]
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let rl = 1.0 / (right - left);
        let tb = 1.0 / (top - bottom);
        let fn_ = 1.0 / (far - near);

        Self {
            m: [
                [2.0 * rl, 0.0, 0.0, -(right + left) * rl],
                [0.0, 2.0 * tb, 0.0, -(top + bottom) * tb],
                [0.0, 0.0, -2.0 * fn_, -(far + near) * fn_],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Create a look-at view matrix
    ///
    /// # Arguments
    /// * `eye` - Camera position
    /// * `target` - Point the camera is looking at
    /// * `up` - Up direction (should be normalized)
    #[must_use]
    pub fn look_at(eye: (f32, f32, f32), target: (f32, f32, f32), up: (f32, f32, f32)) -> Self {
        // Calculate forward vector (from eye to target)
        let fx = target.0 - eye.0;
        let fy = target.1 - eye.1;
        let fz = target.2 - eye.2;

        // Normalize forward
        let f_len = (fx * fx + fy * fy + fz * fz).sqrt();
        let fx = fx / f_len;
        let fy = fy / f_len;
        let fz = fz / f_len;

        // Calculate right vector (cross product of forward and up)
        let rx = fy * up.2 - fz * up.1;
        let ry = fz * up.0 - fx * up.2;
        let rz = fx * up.1 - fy * up.0;

        // Normalize right
        let r_len = (rx * rx + ry * ry + rz * rz).sqrt();
        let rx = rx / r_len;
        let ry = ry / r_len;
        let rz = rz / r_len;

        // Calculate actual up vector (cross product of right and forward)
        let ux = ry * fz - rz * fy;
        let uy = rz * fx - rx * fz;
        let uz = rx * fy - ry * fx;

        Self {
            m: [
                [rx, ux, -fx, 0.0],
                [ry, uy, -fy, 0.0],
                [rz, uz, -fz, 0.0],
                [
                    -(rx * eye.0 + ry * eye.1 + rz * eye.2),
                    -(ux * eye.0 + uy * eye.1 + uz * eye.2),
                    (fx * eye.0 + fy * eye.1 + fz * eye.2),
                    1.0,
                ],
            ],
        }
    }

    /// Add another matrix to this one
    ///
    /// **Deprecated:** Use the `+` operator instead: `m1 + m2`
    #[deprecated(since = "0.1.0", note = "Use the + operator instead")]
    #[must_use]
    pub fn add(&self, matrix: Mat4x4) -> Mat4x4 {
        *self + matrix
    }

    /// Subtract another matrix from this one
    ///
    /// **Deprecated:** Use the `-` operator instead: `m1 - m2`
    #[deprecated(since = "0.1.0", note = "Use the - operator instead")]
    #[must_use]
    pub fn subtract(&self, matrix: Mat4x4) -> Mat4x4 {
        *self - matrix
    }

    /// Multiply this matrix by another matrix
    ///
    /// **Deprecated:** Use the `*` operator instead: `m1 * m2`
    #[deprecated(since = "0.1.0", note = "Use the * operator instead")]
    #[must_use]
    pub fn multiply(&self, matrix: Mat4x4) -> Mat4x4 {
        *self * matrix
    }

    /// Multiply this matrix by a vector
    ///
    /// **Deprecated:** Use the `*` operator instead: `matrix * vector`
    #[deprecated(since = "0.1.0", note = "Use the * operator instead")]
    #[must_use]
    pub fn multiply_vector(&self, vector: Vec4) -> Vec4 {
        *self * vector
    }

    /// Transpose the matrix (swap rows and columns)
    #[must_use]
    pub fn transpose(&self) -> Mat4x4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[j][i] = self.m[i][j];
            }
        }
        Mat4x4 { m: result }
    }

    /// Calculate the determinant of the matrix
    #[must_use]
    pub fn determinant(&self) -> f32 {
        let m = &self.m;

        // Calculate 2x2 subdeterminants
        let a = m[2][2] * m[3][3] - m[2][3] * m[3][2];
        let b = m[2][1] * m[3][3] - m[2][3] * m[3][1];
        let c = m[2][1] * m[3][2] - m[2][2] * m[3][1];
        let d = m[2][0] * m[3][3] - m[2][3] * m[3][0];
        let e = m[2][0] * m[3][2] - m[2][2] * m[3][0];
        let f = m[2][0] * m[3][1] - m[2][1] * m[3][0];

        // Calculate 3x3 subdeterminants
        let det0 = m[1][1] * a - m[1][2] * b + m[1][3] * c;
        let det1 = m[1][0] * a - m[1][2] * d + m[1][3] * e;
        let det2 = m[1][0] * b - m[1][1] * d + m[1][3] * f;
        let det3 = m[1][0] * c - m[1][1] * e + m[1][2] * f;

        // Calculate 4x4 determinant
        m[0][0] * det0 - m[0][1] * det1 + m[0][2] * det2 - m[0][3] * det3
    }

    /// Calculate the inverse of the matrix
    ///
    /// Returns None if the matrix is not invertible (determinant is zero).
    #[must_use]
    pub fn inverse(&self) -> Option<Mat4x4> {
        let det = self.determinant();

        if det.abs() < EPSILON {
            return None;
        }

        let m = &self.m;
        let inv_det = 1.0 / det;

        // Calculate the matrix of cofactors, then transpose (adjugate)
        let mut result = [[0.0; 4]; 4];

        result[0][0] = (m[1][1] * (m[2][2] * m[3][3] - m[2][3] * m[3][2]) -
            m[1][2] * (m[2][1] * m[3][3] - m[2][3] * m[3][1]) +
            m[1][3] * (m[2][1] * m[3][2] - m[2][2] * m[3][1])) * inv_det;

        result[0][1] = -(m[0][1] * (m[2][2] * m[3][3] - m[2][3] * m[3][2]) -
            m[0][2] * (m[2][1] * m[3][3] - m[2][3] * m[3][1]) +
            m[0][3] * (m[2][1] * m[3][2] - m[2][2] * m[3][1])) * inv_det;

        result[0][2] = (m[0][1] * (m[1][2] * m[3][3] - m[1][3] * m[3][2]) -
            m[0][2] * (m[1][1] * m[3][3] - m[1][3] * m[3][1]) +
            m[0][3] * (m[1][1] * m[3][2] - m[1][2] * m[3][1])) * inv_det;

        result[0][3] = -(m[0][1] * (m[1][2] * m[2][3] - m[1][3] * m[2][2]) -
            m[0][2] * (m[1][1] * m[2][3] - m[1][3] * m[2][1]) +
            m[0][3] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])) * inv_det;

        result[1][0] = -(m[1][0] * (m[2][2] * m[3][3] - m[2][3] * m[3][2]) -
            m[1][2] * (m[2][0] * m[3][3] - m[2][3] * m[3][0]) +
            m[1][3] * (m[2][0] * m[3][2] - m[2][2] * m[3][0])) * inv_det;

        result[1][1] = (m[0][0] * (m[2][2] * m[3][3] - m[2][3] * m[3][2]) -
            m[0][2] * (m[2][0] * m[3][3] - m[2][3] * m[3][0]) +
            m[0][3] * (m[2][0] * m[3][2] - m[2][2] * m[3][0])) * inv_det;

        result[1][2] = -(m[0][0] * (m[1][2] * m[3][3] - m[1][3] * m[3][2]) -
            m[0][2] * (m[1][0] * m[3][3] - m[1][3] * m[3][0]) +
            m[0][3] * (m[1][0] * m[3][2] - m[1][2] * m[3][0])) * inv_det;

        result[1][3] = (m[0][0] * (m[1][2] * m[2][3] - m[1][3] * m[2][2]) -
            m[0][2] * (m[1][0] * m[2][3] - m[1][3] * m[2][0]) +
            m[0][3] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])) * inv_det;

        result[2][0] = (m[1][0] * (m[2][1] * m[3][3] - m[2][3] * m[3][1]) -
            m[1][1] * (m[2][0] * m[3][3] - m[2][3] * m[3][0]) +
            m[1][3] * (m[2][0] * m[3][1] - m[2][1] * m[3][0])) * inv_det;

        result[2][1] = -(m[0][0] * (m[2][1] * m[3][3] - m[2][3] * m[3][1]) -
            m[0][1] * (m[2][0] * m[3][3] - m[2][3] * m[3][0]) +
            m[0][3] * (m[2][0] * m[3][1] - m[2][1] * m[3][0])) * inv_det;

        result[2][2] = (m[0][0] * (m[1][1] * m[3][3] - m[1][3] * m[3][1]) -
            m[0][1] * (m[1][0] * m[3][3] - m[1][3] * m[3][0]) +
            m[0][3] * (m[1][0] * m[3][1] - m[1][1] * m[3][0])) * inv_det;

        result[2][3] = -(m[0][0] * (m[1][1] * m[2][3] - m[1][3] * m[2][1]) -
            m[0][1] * (m[1][0] * m[2][3] - m[1][3] * m[2][0]) +
            m[0][3] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])) * inv_det;

        result[3][0] = -(m[1][0] * (m[2][1] * m[3][2] - m[2][2] * m[3][1]) -
            m[1][1] * (m[2][0] * m[3][2] - m[2][2] * m[3][0]) +
            m[1][2] * (m[2][0] * m[3][1] - m[2][1] * m[3][0])) * inv_det;

        result[3][1] = (m[0][0] * (m[2][1] * m[3][2] - m[2][2] * m[3][1]) -
            m[0][1] * (m[2][0] * m[3][2] - m[2][2] * m[3][0]) +
            m[0][2] * (m[2][0] * m[3][1] - m[2][1] * m[3][0])) * inv_det;

        result[3][2] = -(m[0][0] * (m[1][1] * m[3][2] - m[1][2] * m[3][1]) -
            m[0][1] * (m[1][0] * m[3][2] - m[1][2] * m[3][0]) +
            m[0][2] * (m[1][0] * m[3][1] - m[1][1] * m[3][0])) * inv_det;

        result[3][3] = (m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) -
            m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) +
            m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])) * inv_det;

        Some(Mat4x4 { m: result })
    }

    /// Check if this matrix is approximately equal to another
    ///
    /// Uses an epsilon value for floating point comparison
    #[must_use]
    pub fn approx_eq(&self, other: &Mat4x4) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if (self.m[i][j] - other.m[i][j]).abs() >= EPSILON {
                    return false;
                }
            }
        }
        true
    }

    /// Check if this is approximately an identity matrix
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.approx_eq(&Self::identity())
    }

    /// Get a specific element at (row, column)
    ///
    /// Returns None if indices are out of bounds
    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> Option<f32> {
        if row < 4 && col < 4 {
            Some(self.m[row][col])
        } else {
            None
        }
    }

    /// Get a mutable reference to a specific element at (row, column)
    ///
    /// Returns None if indices are out of bounds
    #[must_use]
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut f32> {
        if row < 4 && col < 4 {
            Some(&mut self.m[row][col])
        } else {
            None
        }
    }
}

// ============================================
// Trait Implementations
// ============================================

impl Default for Mat4x4 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Add for Mat4x4 {
    type Output = Mat4x4;

    fn add(self, other: Mat4x4) -> Mat4x4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.m[i][j] + other.m[i][j];
            }
        }
        Mat4x4 { m: result }
    }
}

impl Sub for Mat4x4 {
    type Output = Mat4x4;

    fn sub(self, other: Mat4x4) -> Mat4x4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.m[i][j] - other.m[i][j];
            }
        }
        Mat4x4 { m: result }
    }
}

impl Mul for Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, other: Mat4x4) -> Mat4x4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }
        Mat4x4 { m: result }
    }
}

impl Mul<Vec4> for Mat4x4 {
    type Output = Vec4;

    fn mul(self, vector: Vec4) -> Vec4 {
        Vec4 {
            x: self.m[0][0] * vector.x + self.m[0][1] * vector.y + self.m[0][2] * vector.z + self.m[0][3] * vector.w,
            y: self.m[1][0] * vector.x + self.m[1][1] * vector.y + self.m[1][2] * vector.z + self.m[1][3] * vector.w,
            z: self.m[2][0] * vector.x + self.m[2][1] * vector.y + self.m[2][2] * vector.z + self.m[2][3] * vector.w,
            w: self.m[3][0] * vector.x + self.m[3][1] * vector.y + self.m[3][2] * vector.z + self.m[3][3] * vector.w,
        }
    }
}

impl Mul<f32> for Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, scalar: f32) -> Mat4x4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.m[i][j] * scalar;
            }
        }
        Mat4x4 { m: result }
    }
}

impl Mul<Mat4x4> for f32 {
    type Output = Mat4x4;

    fn mul(self, matrix: Mat4x4) -> Mat4x4 {
        matrix * self
    }
}

impl Neg for Mat4x4 {
    type Output = Mat4x4;

    fn neg(self) -> Mat4x4 {
        self * -1.0
    }
}

impl AddAssign for Mat4x4 {
    fn add_assign(&mut self, other: Mat4x4) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] += other.m[i][j];
            }
        }
    }
}

impl SubAssign for Mat4x4 {
    fn sub_assign(&mut self, other: Mat4x4) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] -= other.m[i][j];
            }
        }
    }
}

impl MulAssign for Mat4x4 {
    fn mul_assign(&mut self, other: Mat4x4) {
        *self = *self * other;
    }
}

impl MulAssign<f32> for Mat4x4 {
    fn mul_assign(&mut self, scalar: f32) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] *= scalar;
            }
        }
    }
}

impl Index<usize> for Mat4x4 {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &Self::Output {
        if index >= 4 {
            panic!("Index out of bounds for Matrix4x4: row {}, but matrix has 4 rows", index);
        }
        &self.m[index]
    }
}

impl IndexMut<usize> for Mat4x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= 4 {
            panic!("Index out of bounds for Matrix4x4: row {}, but matrix has 4 rows", index);
        }
        &mut self.m[index]
    }
}

impl fmt::Display for Mat4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix4x4[\n  [{:7.3}, {:7.3}, {:7.3}, {:7.3}]\n  [{:7.3}, {:7.3}, {:7.3}, {:7.3}]\n  [{:7.3}, {:7.3}, {:7.3}, {:7.3}]\n  [{:7.3}, {:7.3}, {:7.3}, {:7.3}]\n]",
            self.m[0][0], self.m[0][1], self.m[0][2], self.m[0][3],
            self.m[1][0], self.m[1][1], self.m[1][2], self.m[1][3],
            self.m[2][0], self.m[2][1], self.m[2][2], self.m[2][3],
            self.m[3][0], self.m[3][1], self.m[3][2], self.m[3][3],
        )
    }
}

impl From<[[f32; 4]; 4]> for Mat4x4 {
    fn from(m: [[f32; 4]; 4]) -> Self {
        Self::new(m)
    }
}

impl From<Mat4x4> for [[f32; 4]; 4] {
    fn from(matrix: Mat4x4) -> Self {
        matrix.m
    }
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let id = Mat4x4::identity();
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let result = id * v;
        assert!((result.x - v.x).abs() < EPSILON);
        assert!((result.y - v.y).abs() < EPSILON);
        assert!((result.z - v.z).abs() < EPSILON);
        assert!((result.w - v.w).abs() < EPSILON);
    }

    #[test]
    fn test_transpose() {
        let m = Mat4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let t = m.transpose();
        assert_eq!(t.m[0][1], 5.0);
        assert_eq!(t.m[1][0], 2.0);
    }

    #[test]
    fn test_determinant() {
        let id = Mat4x4::identity();
        assert!((id.determinant() - 1.0).abs() < EPSILON);

        let scale = Mat4x4::scaling(2.0, 3.0, 4.0);
        assert!((scale.determinant() - 24.0).abs() < EPSILON);
    }

    #[test]
    fn test_inverse() {
        let m = Mat4x4::scaling(2.0, 3.0, 4.0);
        let inv = m.inverse().unwrap();
        let result = m * inv;
        assert!(result.is_identity());
    }

    #[test]
    fn test_operators() {
        let m1 = Mat4x4::identity();
        let m2 = Mat4x4::identity();

        let sum = m1 + m2;
        assert!((sum.m[0][0] - 2.0).abs() < EPSILON);

        let product = m1 * m2;
        assert!(product.is_identity());
    }
}