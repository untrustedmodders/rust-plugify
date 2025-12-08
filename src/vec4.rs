use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut, AddAssign, SubAssign, MulAssign, DivAssign};

/// A 4D vector with f32 components
///
/// This type is FFI-compatible with C/C++ due to `#[repr(C)]` and has
/// the same memory layout as a struct with four f32 fields.
///
/// Vector4 is commonly used for:
/// - Homogeneous coordinates (w=1 for points, w=0 for directions/vectors)
/// - RGBA colors (x=r, y=g, z=b, w=a)
/// - Quaternions (though typically use a dedicated Quaternion type)
/// - 4D mathematical operations
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// Epsilon for floating point comparisons
const EPSILON: f32 = 1e-6;

impl Vector4 {
    /// Create a new Vector4 with the given x, y, z, and w components
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Create a zero vector (0, 0, 0, 0)
    #[must_use]
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    /// Create a unit vector (1, 1, 1, 1)
    #[must_use]
    pub const fn unit() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }
    }

    /// Create a homogeneous position vector (x, y, z, 1)
    ///
    /// In homogeneous coordinates, w=1 represents a position/point
    #[must_use]
    pub const fn position(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    /// Create a homogeneous direction vector (x, y, z, 0)
    ///
    /// In homogeneous coordinates, w=0 represents a direction/vector
    #[must_use]
    pub const fn direction(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    /// Create an RGBA color vector
    ///
    /// Components should typically be in the range [0.0, 1.0]
    #[must_use]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { x: r, y: g, z: b, w: a }
    }

    /// Add another vector to this one
    ///
    /// **Deprecated:** Use the `+` operator instead: `v1 + v2`
    #[deprecated(since = "0.1.0", note = "Use the + operator instead")]
    #[must_use]
    pub fn add(&self, vector: Vector4) -> Vector4 {
        *self + vector
    }

    /// Subtract another vector from this one
    ///
    /// **Deprecated:** Use the `-` operator instead: `v1 - v2`
    #[deprecated(since = "0.1.0", note = "Use the - operator instead")]
    #[must_use]
    pub fn subtract(&self, vector: Vector4) -> Vector4 {
        *self - vector
    }

    /// Multiply this vector by a scalar
    ///
    /// **Deprecated:** Use the `*` operator instead: `v * scalar`
    #[deprecated(since = "0.1.0", note = "Use the * operator instead")]
    #[must_use]
    pub fn scale(&self, scalar: f32) -> Vector4 {
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
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// Normalize the vector to unit length
    ///
    /// Returns a zero vector if the magnitude is too small (near zero).
    #[must_use]
    pub fn normalize(&self) -> Vector4 {
        let mag_sq = self.magnitude_squared();

        // Use squared magnitude to avoid unnecessary sqrt when checking for zero
        if mag_sq < EPSILON * EPSILON {
            return Vector4::zero();
        }

        let mag = mag_sq.sqrt();
        *self / mag
    }

    /// Normalize the vector to unit length
    ///
    /// Returns None if the magnitude is too small (near zero).
    #[must_use]
    pub fn try_normalize(&self) -> Option<Vector4> {
        let mag_sq = self.magnitude_squared();

        if mag_sq < EPSILON * EPSILON {
            return None;
        }

        let mag = mag_sq.sqrt();
        Some(*self / mag)
    }

    /// Calculate the dot product with another vector
    #[must_use]
    pub fn dot(&self, vector: Vector4) -> f32 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z + self.w * vector.w
    }

    /// Calculate the distance to another vector
    #[must_use]
    pub fn distance_to(&self, vector: Vector4) -> f32 {
        (*self - vector).magnitude()
    }

    /// Calculate the squared distance to another vector
    ///
    /// This is faster than `distance_to()` since it avoids the square root.
    #[must_use]
    pub fn distance_squared_to(&self, vector: Vector4) -> f32 {
        (*self - vector).magnitude_squared()
    }

    /// Linear interpolation between this vector and another
    ///
    /// `t` should be between 0.0 and 1.0:
    /// - t = 0.0 returns self
    /// - t = 1.0 returns other
    /// - t = 0.5 returns the midpoint
    #[must_use]
    pub fn lerp(&self, other: Vector4, t: f32) -> Vector4 {
        Vector4 {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
            w: self.w + (other.w - self.w) * t,
        }
    }

    /// Clamp the vector's magnitude to a maximum value
    #[must_use]
    pub fn clamp_magnitude(&self, max_magnitude: f32) -> Vector4 {
        let mag_sq = self.magnitude_squared();
        let max_sq = max_magnitude * max_magnitude;

        if mag_sq <= max_sq {
            *self
        } else {
            let mag = mag_sq.sqrt();
            *self * (max_magnitude / mag)
        }
    }

    /// Project this vector onto another vector
    #[must_use]
    pub fn project_onto(&self, other: Vector4) -> Vector4 {
        let dot = self.dot(other);
        let mag_sq = other.magnitude_squared();

        if mag_sq < EPSILON * EPSILON {
            return Vector4::zero();
        }

        other * (dot / mag_sq)
    }

    /// Get the angle between this vector and another, in radians
    ///
    /// Always returns a positive angle in the range [0, π]
    #[must_use]
    pub fn angle_to(&self, other: Vector4) -> f32 {
        let dot = self.dot(other);
        let mags = self.magnitude() * other.magnitude();

        if mags < EPSILON {
            return 0.0;
        }

        (dot / mags).clamp(-1.0, 1.0).acos()
    }

    /// Check if this vector is approximately equal to another
    ///
    /// Uses an epsilon value for floating point comparison
    #[must_use]
    pub fn approx_eq(&self, other: Vector4) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
            && (self.w - other.w).abs() < EPSILON
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

    /// Get a specific component by index (0 = x, 1 = y, 2 = z, 3 = w)
    ///
    /// Returns None if index is out of bounds
    #[must_use]
    pub fn get(&self, index: usize) -> Option<f32> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            3 => Some(self.w),
            _ => None,
        }
    }

    /// Get a mutable reference to a specific component by index (0 = x, 1 = y, 2 = z, 3 = w)
    ///
    /// Returns None if index is out of bounds
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut f32> {
        match index {
            0 => Some(&mut self.x),
            1 => Some(&mut self.y),
            2 => Some(&mut self.z),
            3 => Some(&mut self.w),
            _ => None,
        }
    }

    /// Return the component-wise minimum of two vectors
    #[must_use]
    pub fn min(&self, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
            w: self.w.min(other.w),
        }
    }

    /// Return the component-wise maximum of two vectors
    #[must_use]
    pub fn max(&self, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
            w: self.w.max(other.w),
        }
    }

    /// Clamp each component to a range
    #[must_use]
    pub fn clamp(&self, min: Vector4, max: Vector4) -> Vector4 {
        Vector4 {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
            z: self.z.clamp(min.z, max.z),
            w: self.w.clamp(min.w, max.w),
        }
    }

    /// Return the component-wise absolute value
    #[must_use]
    pub fn abs(&self) -> Vector4 {
        Vector4 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
            w: self.w.abs(),
        }
    }

    /// Perform perspective division (divide xyz by w)
    ///
    /// This converts from homogeneous coordinates to 3D Cartesian coordinates.
    /// Returns None if w is too close to zero.
    #[must_use]
    pub fn perspective_divide(&self) -> Option<(f32, f32, f32)> {
        if self.w.abs() < EPSILON {
            return None;
        }

        Some((self.x / self.w, self.y / self.w, self.z / self.w))
    }

    /// Get the xyz components as a 3-tuple, ignoring w
    #[must_use]
    pub fn xyz(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    /// Check if this is a homogeneous position (w ≈ 1)
    #[must_use]
    pub fn is_position(&self) -> bool {
        (self.w - 1.0).abs() < EPSILON
    }

    /// Check if this is a homogeneous direction (w ≈ 0)
    #[must_use]
    pub fn is_direction(&self) -> bool {
        self.w.abs() < EPSILON
    }
}

// ============================================
// Trait Implementations
// ============================================

impl Default for Vector4 {
    fn default() -> Self {
        Self::zero()
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

impl Mul<Vector4> for f32 {
    type Output = Vector4;

    fn mul(self, vector: Vector4) -> Vector4 {
        Vector4 {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
            w: vector.w * self,
        }
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, scalar: f32) -> Vector4 {
        Vector4 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

impl Neg for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Vector4 {
        Vector4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl AddAssign for Vector4 {
    fn add_assign(&mut self, other: Vector4) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl SubAssign for Vector4 {
    fn sub_assign(&mut self, other: Vector4) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
    }
}

impl DivAssign<f32> for Vector4 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
        self.w /= scalar;
    }
}

impl Index<usize> for Vector4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds for Vector4: index {}, but length is 4", index),
        }
    }
}

impl IndexMut<usize> for Vector4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds for Vector4: index {}, but length is 4", index),
        }
    }
}

impl fmt::Display for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl From<(f32, f32, f32, f32)> for Vector4 {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        Self::new(x, y, z, w)
    }
}

impl From<[f32; 4]> for Vector4 {
    fn from([x, y, z, w]: [f32; 4]) -> Self {
        Self::new(x, y, z, w)
    }
}

impl From<Vector4> for (f32, f32, f32, f32) {
    fn from(v: Vector4) -> Self {
        (v.x, v.y, v.z, v.w)
    }
}

impl From<Vector4> for [f32; 4] {
    fn from(v: Vector4) -> Self {
        [v.x, v.y, v.z, v.w]
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
        let zero = Vector4::zero();
        assert_eq!(zero.normalize(), Vector4::zero());
        assert_eq!(zero.try_normalize(), None);
    }

    #[test]
    fn test_approx_eq() {
        let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vector4::new(1.0 + EPSILON * 0.5, 2.0, 3.0, 4.0);
        assert!(v1.approx_eq(v2));
    }

    #[test]
    fn test_homogeneous_coordinates() {
        let pos = Vector4::position(1.0, 2.0, 3.0);
        assert!(pos.is_position());
        assert!(!pos.is_direction());

        let dir = Vector4::direction(1.0, 0.0, 0.0);
        assert!(dir.is_direction());
        assert!(!dir.is_position());
    }

    #[test]
    fn test_perspective_divide() {
        let v = Vector4::new(10.0, 20.0, 30.0, 2.0);
        let (x, y, z) = v.perspective_divide().unwrap();
        assert!((x - 5.0).abs() < EPSILON);
        assert!((y - 10.0).abs() < EPSILON);
        assert!((z - 15.0).abs() < EPSILON);

        let zero_w = Vector4::new(1.0, 2.0, 3.0, 0.0);
        assert_eq!(zero_w.perspective_divide(), None);
    }

    #[test]
    fn test_lerp() {
        let v1 = Vector4::zero();
        let v2 = Vector4::unit();
        let mid = v1.lerp(v2, 0.5);
        assert!(mid.approx_eq(Vector4::new(0.5, 0.5, 0.5, 0.5)));
    }

    #[test]
    fn test_operators() {
        let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vector4::new(5.0, 6.0, 7.0, 8.0);

        assert!((v1 + v2).approx_eq(Vector4::new(6.0, 8.0, 10.0, 12.0)));
        assert!((v1 - v2).approx_eq(Vector4::new(-4.0, -4.0, -4.0, -4.0)));
        assert!((v1 * 2.0).approx_eq(Vector4::new(2.0, 4.0, 6.0, 8.0)));
        assert!((2.0 * v1).approx_eq(Vector4::new(2.0, 4.0, 6.0, 8.0)));
        assert!((-v1).approx_eq(Vector4::new(-1.0, -2.0, -3.0, -4.0)));
    }

    #[test]
    fn test_rgba() {
        let color = Vector4::rgba(1.0, 0.5, 0.25, 1.0);
        assert_eq!(color.x, 1.0);   // red
        assert_eq!(color.y, 0.5);   // green
        assert_eq!(color.z, 0.25);  // blue
        assert_eq!(color.w, 1.0);   // alpha
    }
}