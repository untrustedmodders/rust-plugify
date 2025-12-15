use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut, AddAssign, SubAssign, MulAssign, DivAssign};

/// A 3D vector with f32 components
///
/// This type is FFI-compatible with C/C++ due to `#[repr(C)]` and has
/// the same memory layout as a struct with three f32 fields.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Epsilon for floating point comparisons
const EPSILON: f32 = 1e-6;

impl Vec3 {
    /// Create a new Vector3 with the given x, y, and z components
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Create a zero vector (0, 0, 0)
    #[must_use]
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Create a unit vector (1, 1, 1)
    #[must_use]
    pub const fn unit() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0 }
    }

    /// Create a vector pointing right (1, 0, 0)
    #[must_use]
    pub const fn right() -> Self {
        Self { x: 1.0, y: 0.0, z: 0.0 }
    }

    /// Create a vector pointing left (-1, 0, 0)
    #[must_use]
    pub const fn left() -> Self {
        Self { x: -1.0, y: 0.0, z: 0.0 }
    }

    /// Create a vector pointing up (0, 1, 0)
    #[must_use]
    pub const fn up() -> Self {
        Self { x: 0.0, y: 1.0, z: 0.0 }
    }

    /// Create a vector pointing down (0, -1, 0)
    #[must_use]
    pub const fn down() -> Self {
        Self { x: 0.0, y: -1.0, z: 0.0 }
    }

    /// Create a vector pointing forward (0, 0, 1)
    #[must_use]
    pub const fn forward() -> Self {
        Self { x: 0.0, y: 0.0, z: 1.0 }
    }

    /// Create a vector pointing backward (0, 0, -1)
    #[must_use]
    pub const fn back() -> Self {
        Self { x: 0.0, y: 0.0, z: -1.0 }
    }

    /// Add another vector to this one
    ///
    /// **Deprecated:** Use the `+` operator instead: `v1 + v2`
    #[deprecated(since = "0.1.0", note = "Use the + operator instead")]
    #[must_use]
    pub fn add(&self, vector: Vec3) -> Vec3 {
        *self + vector
    }

    /// Subtract another vector from this one
    ///
    /// **Deprecated:** Use the `-` operator instead: `v1 - v2`
    #[deprecated(since = "0.1.0", note = "Use the - operator instead")]
    #[must_use]
    pub fn subtract(&self, vector: Vec3) -> Vec3 {
        *self - vector
    }

    /// Multiply this vector by a scalar
    ///
    /// **Deprecated:** Use the `*` operator instead: `v * scalar`
    #[deprecated(since = "0.1.0", note = "Use the * operator instead")]
    #[must_use]
    pub fn scale(&self, scalar: f32) -> Vec3 {
        *self * scalar
    }

    /// Calculate the magnitude (length) of the vector
    #[must_use]
    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    /// Calculate the squared magnitude of the vector
    ///
    /// This is faster than `magnitude()` since it avoids the square root.
    /// Use this when you only need to compare lengths.
    #[must_use]
    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Normalize the vector to unit length
    ///
    /// Returns a zero vector if the magnitude is too small (near zero).
    #[must_use]
    pub fn normalize(&self) -> Vec3 {
        let mag_sq = self.magnitude_squared();

        // Use squared magnitude to avoid unnecessary sqrt when checking for zero
        if mag_sq < EPSILON * EPSILON {
            return Vec3::zero();
        }

        let mag = mag_sq.sqrt();
        *self / mag
    }

    /// Normalize the vector to unit length
    ///
    /// Returns None if the magnitude is too small (near zero).
    #[must_use]
    pub fn try_normalize(&self) -> Option<Vec3> {
        let mag_sq = self.magnitude_squared();

        if mag_sq < EPSILON * EPSILON {
            return None;
        }

        let mag = mag_sq.sqrt();
        Some(*self / mag)
    }

    /// Calculate the dot product with another vector
    #[must_use]
    pub fn dot(&self, vector: Vec3) -> f32 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z
    }

    /// Calculate the cross product with another vector
    ///
    /// The cross product produces a vector perpendicular to both input vectors.
    /// The magnitude equals the area of the parallelogram formed by the two vectors.
    #[must_use]
    pub fn cross(&self, vector: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * vector.z - self.z * vector.y,
            y: self.z * vector.x - self.x * vector.z,
            z: self.x * vector.y - self.y * vector.x,
        }
    }

    /// Calculate the distance to another vector
    #[must_use]
    pub fn distance_to(&self, vector: Vec3) -> f32 {
        (*self - vector).magnitude()
    }

    /// Calculate the squared distance to another vector
    ///
    /// This is faster than `distance_to()` since it avoids the square root.
    #[must_use]
    pub fn distance_squared_to(&self, vector: Vec3) -> f32 {
        (*self - vector).magnitude_squared()
    }

    /// Linear interpolation between this vector and another
    ///
    /// `t` should be between 0.0 and 1.0:
    /// - t = 0.0 returns self
    /// - t = 1.0 returns other
    /// - t = 0.5 returns the midpoint
    #[must_use]
    pub fn lerp(&self, other: Vec3, t: f32) -> Vec3 {
        Vec3 {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }

    /// Spherical linear interpolation between this vector and another
    ///
    /// This maintains constant speed on the unit sphere. Both vectors
    /// should be normalized for correct results.
    #[must_use]
    pub fn slerp(&self, other: Vec3, t: f32) -> Vec3 {
        let dot = self.dot(other).clamp(-1.0, 1.0);
        let theta = dot.acos();

        if theta.abs() < EPSILON {
            // Vectors are nearly parallel, use linear interpolation
            return self.lerp(other, t);
        }

        let sin_theta = theta.sin();
        let a = ((1.0 - t) * theta).sin() / sin_theta;
        let b = (t * theta).sin() / sin_theta;

        *self * a + other * b
    }

    /// Clamp the vector's magnitude to a maximum value
    #[must_use]
    pub fn clamp_magnitude(&self, max_magnitude: f32) -> Vec3 {
        let mag_sq = self.magnitude_squared();
        let max_sq = max_magnitude * max_magnitude;

        if mag_sq <= max_sq {
            *self
        } else {
            let mag = mag_sq.sqrt();
            *self * (max_magnitude / mag)
        }
    }

    /// Reflect the vector across a normal
    ///
    /// The normal should be normalized for correct results.
    #[must_use]
    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - normal * (2.0 * self.dot(normal))
    }

    /// Project this vector onto another vector
    #[must_use]
    pub fn project_onto(&self, other: Vec3) -> Vec3 {
        let dot = self.dot(other);
        let mag_sq = other.magnitude_squared();

        if mag_sq < EPSILON * EPSILON {
            return Vec3::zero();
        }

        other * (dot / mag_sq)
    }

    /// Get the angle between this vector and another, in radians
    ///
    /// Always returns a positive angle in the range [0, π]
    #[must_use]
    pub fn angle_to(&self, other: Vec3) -> f32 {
        let dot = self.dot(other);
        let mags = self.magnitude() * other.magnitude();

        if mags < EPSILON {
            return 0.0;
        }

        (dot / mags).clamp(-1.0, 1.0).acos()
    }

    /// Rotate the vector around an axis by an angle (in radians)
    ///
    /// The axis should be normalized for correct results.
    /// Uses Rodrigues' rotation formula.
    #[must_use]
    pub fn rotate_around_axis(&self, axis: Vec3, angle: f32) -> Vec3 {
        let cos = angle.cos();
        let sin = angle.sin();

        *self * cos + axis.cross(*self) * sin + axis * (axis.dot(*self) * (1.0 - cos))
    }

    /// Check if this vector is approximately equal to another
    ///
    /// Uses an epsilon value for floating point comparison
    #[must_use]
    pub fn approx_eq(&self, other: Vec3) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }

    /// Check if this vector is approximately zero
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.magnitude_squared() < EPSILON * EPSILON
    }

    /// Check if this vector is approximately normalized (unit length)
    #[must_use]
    pub fn is_normalized(&self) -> bool {
        (self.magnitude_squared() - 1.0).abs() < EPSILON
    }

    /// Get a specific component by index (0 = x, 1 = y, 2 = z)
    ///
    /// Returns None if index is out of bounds
    #[must_use]
    pub fn get(&self, index: usize) -> Option<f32> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            _ => None,
        }
    }

    /// Get a mutable reference to a specific component by index (0 = x, 1 = y, 2 = z)
    ///
    /// Returns None if index is out of bounds
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut f32> {
        match index {
            0 => Some(&mut self.x),
            1 => Some(&mut self.y),
            2 => Some(&mut self.z),
            _ => None,
        }
    }

    /// Return the component-wise minimum of two vectors
    #[must_use]
    pub fn min(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    /// Return the component-wise maximum of two vectors
    #[must_use]
    pub fn max(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    /// Clamp each component to a range
    #[must_use]
    pub fn clamp(&self, min: Vec3, max: Vec3) -> Vec3 {
        Vec3 {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
            z: self.z.clamp(min.z, max.z),
        }
    }

    /// Return the component-wise absolute value
    #[must_use]
    pub fn abs(&self) -> Vec3 {
        Vec3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    /// Calculate the triple scalar product (a · (b × c))
    ///
    /// This gives the signed volume of the parallelepiped formed by three vectors.
    #[must_use]
    pub fn triple_product(a: Vec3, b: Vec3, c: Vec3) -> f32 {
        a.dot(b.cross(c))
    }

    /// Check if three vectors are coplanar (lie in the same plane)
    #[must_use]
    pub fn are_coplanar(a: Vec3, b: Vec3, c: Vec3) -> bool {
        Self::triple_product(a, b, c).abs() < EPSILON
    }
}

// ============================================
// Trait Implementations
// ============================================

impl Default for Vec3 {
    fn default() -> Self {
        Self::zero()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3 {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vector3: index {}, but length is 3", index),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vector3: index {}, but length is 3", index),
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self::new(x, y, z)
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from([x, y, z]: [f32; 3]) -> Self {
        Self::new(x, y, z)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    fn from(v: Vec3) -> Self {
        (v.x, v.y, v.z)
    }
}

impl From<Vec3> for [f32; 3] {
    fn from(v: Vec3) -> Self {
        [v.x, v.y, v.z]
    }
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_zero_vector() {
        let zero = Vec3::zero();
        assert_eq!(zero.normalize(), Vec3::zero());
        assert_eq!(zero.try_normalize(), None);
    }

    #[test]
    fn test_approx_eq() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0 + EPSILON * 0.5, 2.0, 3.0);
        assert!(v1.approx_eq(v2));
    }

    #[test]
    fn test_cross_product() {
        let x = Vec3::right();
        let y = Vec3::up();
        let z = x.cross(y);
        assert!(z.approx_eq(Vec3::forward()));

        // Cross product is perpendicular to both vectors
        assert!((x.dot(z)).abs() < EPSILON);
        assert!((y.dot(z)).abs() < EPSILON);
    }

    #[test]
    fn test_lerp() {
        let v1 = Vec3::zero();
        let v2 = Vec3::new(10.0, 10.0, 10.0);
        let mid = v1.lerp(v2, 0.5);
        assert!(mid.approx_eq(Vec3::new(5.0, 5.0, 5.0)));
    }

    #[test]
    fn test_operators() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert!((v1 + v2).approx_eq(Vec3::new(5.0, 7.0, 9.0)));
        assert!((v1 - v2).approx_eq(Vec3::new(-3.0, -3.0, -3.0)));
        assert!((v1 * 2.0).approx_eq(Vec3::new(2.0, 4.0, 6.0)));
        assert!((2.0 * v1).approx_eq(Vec3::new(2.0, 4.0, 6.0)));
        assert!((-v1).approx_eq(Vec3::new(-1.0, -2.0, -3.0)));
    }

    #[test]
    fn test_triple_product() {
        let x = Vec3::right();
        let y = Vec3::up();
        let z = Vec3::forward();

        // Right-handed coordinate system has volume 1
        assert!((Vec3::triple_product(x, y, z) - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_rotation() {
        use std::f32::consts::PI;

        let v = Vec3::right();
        let rotated = v.rotate_around_axis(Vec3::up(), PI / 2.0);
        assert!(rotated.approx_eq(Vec3::forward()));
    }
}