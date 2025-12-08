use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut, AddAssign, SubAssign, MulAssign, DivAssign};

/// A 2D vector with f32 components
///
/// This type is FFI-compatible with C/C++ due to `#[repr(C)]` and has
/// the same memory layout as a struct with two f32 fields.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

/// Epsilon for floating point comparisons
const EPSILON: f32 = 1e-6;

impl Vector2 {
    /// Create a new Vector2 with the given x and y components
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Create a zero vector (0, 0)
    #[must_use]
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Create a unit vector (1, 1)
    #[must_use]
    pub const fn unit() -> Self {
        Self { x: 1.0, y: 1.0 }
    }

    /// Create a vector pointing right (1, 0)
    #[must_use]
    pub const fn right() -> Self {
        Self { x: 1.0, y: 0.0 }
    }

    /// Create a vector pointing left (-1, 0)
    #[must_use]
    pub const fn left() -> Self {
        Self { x: -1.0, y: 0.0 }
    }

    /// Create a vector pointing up (0, 1)
    #[must_use]
    pub const fn up() -> Self {
        Self { x: 0.0, y: 1.0 }
    }

    /// Create a vector pointing down (0, -1)
    #[must_use]
    pub const fn down() -> Self {
        Self { x: 0.0, y: -1.0 }
    }

    /// Add another vector to this one
    ///
    /// **Deprecated:** Use the `+` operator instead: `v1 + v2`
    #[deprecated(since = "0.1.0", note = "Use the + operator instead")]
    #[must_use]
    pub fn add(&self, vector: Vector2) -> Vector2 {
        *self + vector
    }

    /// Subtract another vector from this one
    ///
    /// **Deprecated:** Use the `-` operator instead: `v1 - v2`
    #[deprecated(since = "0.1.0", note = "Use the - operator instead")]
    #[must_use]
    pub fn subtract(&self, vector: Vector2) -> Vector2 {
        *self - vector
    }

    /// Multiply this vector by a scalar
    ///
    /// **Deprecated:** Use the `*` operator instead: `v * scalar`
    #[deprecated(since = "0.1.0", note = "Use the * operator instead")]
    #[must_use]
    pub fn scale(&self, scalar: f32) -> Vector2 {
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
        self.x * self.x + self.y * self.y
    }

    /// Normalize the vector to unit length
    ///
    /// Returns a zero vector if the magnitude is too small (near zero).
    #[must_use]
    pub fn normalize(&self) -> Vector2 {
        let mag_sq = self.magnitude_squared();

        // Use squared magnitude to avoid unnecessary sqrt when checking for zero
        if mag_sq < EPSILON * EPSILON {
            return Vector2::zero();
        }

        let mag = mag_sq.sqrt();
        *self / mag
    }

    /// Normalize the vector to unit length
    ///
    /// Returns None if the magnitude is too small (near zero).
    #[must_use]
    pub fn try_normalize(&self) -> Option<Vector2> {
        let mag_sq = self.magnitude_squared();

        if mag_sq < EPSILON * EPSILON {
            return None;
        }

        let mag = mag_sq.sqrt();
        Some(*self / mag)
    }

    /// Calculate the dot product with another vector
    #[must_use]
    pub fn dot(&self, vector: Vector2) -> f32 {
        self.x * vector.x + self.y * vector.y
    }

    /// Calculate the 2D cross product (returns a scalar)
    ///
    /// This gives the z-component of the 3D cross product if both vectors
    /// were in the XY plane. Useful for determining clockwise/counterclockwise.
    #[must_use]
    pub fn cross(&self, vector: Vector2) -> f32 {
        self.x * vector.y - self.y * vector.x
    }

    /// Calculate the distance to another vector
    #[must_use]
    pub fn distance_to(&self, vector: Vector2) -> f32 {
        (*self - vector).magnitude()
    }

    /// Calculate the squared distance to another vector
    ///
    /// This is faster than `distance_to()` since it avoids the square root.
    #[must_use]
    pub fn distance_squared_to(&self, vector: Vector2) -> f32 {
        (*self - vector).magnitude_squared()
    }

    /// Linear interpolation between this vector and another
    ///
    /// `t` should be between 0.0 and 1.0:
    /// - t = 0.0 returns self
    /// - t = 1.0 returns other
    /// - t = 0.5 returns the midpoint
    #[must_use]
    pub fn lerp(&self, other: Vector2, t: f32) -> Vector2 {
        Vector2 {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    /// Clamp the vector's magnitude to a maximum value
    #[must_use]
    pub fn clamp_magnitude(&self, max_magnitude: f32) -> Vector2 {
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
    pub fn reflect(&self, normal: Vector2) -> Vector2 {
        *self - normal * (2.0 * self.dot(normal))
    }

    /// Project this vector onto another vector
    #[must_use]
    pub fn project_onto(&self, other: Vector2) -> Vector2 {
        let dot = self.dot(other);
        let mag_sq = other.magnitude_squared();

        if mag_sq < EPSILON * EPSILON {
            return Vector2::zero();
        }

        other * (dot / mag_sq)
    }

    /// Get the angle of this vector in radians
    ///
    /// Returns the angle from the positive x-axis, in the range [-π, π]
    #[must_use]
    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    /// Get the angle between this vector and another, in radians
    ///
    /// Always returns a positive angle in the range [0, π]
    #[must_use]
    pub fn angle_to(&self, other: Vector2) -> f32 {
        let dot = self.dot(other);
        let mags = self.magnitude() * other.magnitude();

        if mags < EPSILON {
            return 0.0;
        }

        (dot / mags).clamp(-1.0, 1.0).acos()
    }

    /// Rotate the vector by an angle (in radians)
    #[must_use]
    pub fn rotate(&self, angle: f32) -> Vector2 {
        let cos = angle.cos();
        let sin = angle.sin();
        Vector2 {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    /// Get a perpendicular vector (rotated 90 degrees counter-clockwise)
    #[must_use]
    pub fn perpendicular(&self) -> Vector2 {
        Vector2 {
            x: -self.y,
            y: self.x,
        }
    }

    /// Check if this vector is approximately equal to another
    ///
    /// Uses an epsilon value for floating point comparison
    #[must_use]
    pub fn approx_eq(&self, other: Vector2) -> bool {
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
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

    /// Get a specific component by index (0 = x, 1 = y)
    ///
    /// Returns None if index is out of bounds
    #[must_use]
    pub fn get(&self, index: usize) -> Option<f32> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            _ => None,
        }
    }

    /// Get a mutable reference to a specific component by index (0 = x, 1 = y)
    ///
    /// Returns None if index is out of bounds
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut f32> {
        match index {
            0 => Some(&mut self.x),
            1 => Some(&mut self.y),
            _ => None,
        }
    }

    /// Return the component-wise minimum of two vectors
    #[must_use]
    pub fn min(&self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    /// Return the component-wise maximum of two vectors
    #[must_use]
    pub fn max(&self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    /// Clamp each component to a range
    #[must_use]
    pub fn clamp(&self, min: Vector2, max: Vector2) -> Vector2 {
        Vector2 {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }

    /// Return the component-wise absolute value
    #[must_use]
    pub fn abs(&self) -> Vector2 {
        Vector2 {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

// ============================================
// Trait Implementations
// ============================================

impl Default for Vector2 {
    fn default() -> Self {
        Self::zero()
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

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, vector: Vector2) -> Vector2 {
        Vector2 {
            x: vector.x * self,
            y: vector.y * self,
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Vector2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Vector2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Index<usize> for Vector2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds for Vector2: index {}, but length is 2", index),
        }
    }
}

impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds for Vector2: index {}, but length is 2", index),
        }
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector2({}, {})", self.x, self.y)
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

impl From<[f32; 2]> for Vector2 {
    fn from([x, y]: [f32; 2]) -> Self {
        Self::new(x, y)
    }
}

impl From<Vector2> for (f32, f32) {
    fn from(v: Vector2) -> Self {
        (v.x, v.y)
    }
}

impl From<Vector2> for [f32; 2] {
    fn from(v: Vector2) -> Self {
        [v.x, v.y]
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
        let zero = Vector2::zero();
        assert_eq!(zero.normalize(), Vector2::zero());
        assert_eq!(zero.try_normalize(), None);
    }

    #[test]
    fn test_approx_eq() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(1.0 + EPSILON * 0.5, 2.0);
        assert!(v1.approx_eq(v2));
    }

    #[test]
    fn test_perpendicular() {
        let v = Vector2::new(1.0, 0.0);
        let perp = v.perpendicular();
        assert!(perp.approx_eq(Vector2::new(0.0, 1.0)));
        assert!((v.dot(perp)).abs() < EPSILON); // Should be perpendicular
    }

    #[test]
    fn test_lerp() {
        let v1 = Vector2::new(0.0, 0.0);
        let v2 = Vector2::new(10.0, 10.0);
        let mid = v1.lerp(v2, 0.5);
        assert!(mid.approx_eq(Vector2::new(5.0, 5.0)));
    }

    #[test]
    fn test_operators() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(3.0, 4.0);

        assert!((v1 + v2).approx_eq(Vector2::new(4.0, 6.0)));
        assert!((v1 - v2).approx_eq(Vector2::new(-2.0, -2.0)));
        assert!((v1 * 2.0).approx_eq(Vector2::new(2.0, 4.0)));
        assert!((2.0 * v1).approx_eq(Vector2::new(2.0, 4.0)));
        assert!((-v1).approx_eq(Vector2::new(-1.0, -2.0)));
    }

    #[test]
    fn test_assign_operators() {
        let mut v = Vector2::new(1.0, 2.0);
        v += Vector2::new(1.0, 1.0);
        assert!(v.approx_eq(Vector2::new(2.0, 3.0)));

        v *= 2.0;
        assert!(v.approx_eq(Vector2::new(4.0, 6.0)));
    }
}