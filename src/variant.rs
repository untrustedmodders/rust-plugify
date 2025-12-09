use std::mem::{ManuallyDrop};
use std::sync::OnceLock;
use crate::{import_symbol, PlgString, PlgVector, Vector2, Vector3, Vector4, Matrix4x4};

import_symbol!(destroy_variant, DESTROY_VARIANT, init_destroy_variant, (variant: *mut PlgVariant) -> ());

// Variant type enum
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlgType {
    Invalid,

    // C types
    Void,
    Bool,
    Char8,
    Char16,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Pointer,
    Float,
    Double,
    Function,

    // std::string
    String,

    // std::any
    Any,

    // std::vector
    ArrayBool,
    ArrayChar8,
    ArrayChar16,
    ArrayInt8,
    ArrayInt16,
    ArrayInt32,
    ArrayInt64,
    ArrayUInt8,
    ArrayUInt16,
    ArrayUInt32,
    ArrayUInt64,
    ArrayPointer,
    ArrayFloat,
    ArrayDouble,
    ArrayString,
    ArrayAny,
    ArrayVector2,
    ArrayVector3,
    ArrayVector4,
    ArrayMatrix4x4,

    // glm:vec
    Vector2,
    Vector3,
    Vector4,

    // glm:mat
    Matrix4x4,
    //Matrix2x2,
    //Matrix2x3,
    //Matrix2x4,
    //Matrix3x2,
    //Matrix3x3,
    //Matrix3x4,
    //Matrix4x2,
    //Matrix4x3,
}

/// Union containing all possible variant data types
///
/// # Safety
///
/// This is a C-compatible union. Only the field corresponding to the `PlgType`
/// discriminant in `PlgVariant::current` may be accessed. Accessing the wrong
/// field is undefined behavior.
///
/// Fields wrapped in `ManuallyDrop` must be manually dropped when the variant
/// is destroyed - this is handled by the C++ `destroy_variant` function.
#[repr(C)]
union PlgVariantData {
    // Scalar types (Copy, no Drop)
    boolean: bool,
    char8: i8,
    char16: u16,
    int8: i8,
    int16: i16,
    int32: i32,
    int64: i64,
    uint8: u8,
    uint16: u16,
    uint32: u32,
    uint64: u64,
    ptr: usize,
    flt: f32,
    dbl: f64,

    // Owned types (require Drop, wrapped in ManuallyDrop)
    str: ManuallyDrop<PlgString>,
    vec_bool: ManuallyDrop<PlgVector<bool>>,
    vec_c8: ManuallyDrop<PlgVector<i8>>,
    vec_c16: ManuallyDrop<PlgVector<u16>>,
    vec_i8: ManuallyDrop<PlgVector<i8>>,
    vec_i16: ManuallyDrop<PlgVector<i16>>,
    vec_i32: ManuallyDrop<PlgVector<i32>>,
    vec_i64: ManuallyDrop<PlgVector<i64>>,
    vec_u8: ManuallyDrop<PlgVector<u8>>,
    vec_u16: ManuallyDrop<PlgVector<u16>>,
    vec_u32: ManuallyDrop<PlgVector<u32>>,
    vec_u64: ManuallyDrop<PlgVector<u64>>,
    vec_usize: ManuallyDrop<PlgVector<usize>>,
    vec_f32: ManuallyDrop<PlgVector<f32>>,
    vec_f64: ManuallyDrop<PlgVector<f64>>,
    vec_str: ManuallyDrop<PlgVector<PlgString>>,
    vec_vec2: ManuallyDrop<PlgVector<Vector2>>,
    vec_vec3: ManuallyDrop<PlgVector<Vector3>>,
    vec_vec4: ManuallyDrop<PlgVector<Vector4>>,
    vec_mat4x4: ManuallyDrop<PlgVector<Matrix4x4>>,

    // Vector types (Copy, no Drop)
    vec2: Vector2,
    vec3: Vector3,
    vec4: Vector4,
}

/// FFI-compatible variant type matching the memory layout of C++ plg::variant
///
/// # Memory Layout
///
/// ```text
/// [data: 24 bytes union] [pad: 8 bytes on 32-bit] [type: 1 byte PlgType]
/// Total: 32 bytes
/// ```
///
/// The padding on 32-bit architectures ensures the struct is 32 bytes to match
/// the C++ layout, which likely uses alignment requirements or explicit padding.
///
/// # Type Safety Invariant
///
/// **CRITICAL**: The `current` field MUST always accurately reflect which union field
/// is active. Accessing a union field that doesn't match `current` is undefined behavior.
/// This invariant is maintained by:
/// - Only setting union fields through `construct()`
/// - Always updating `current` atomically with union field assignment
/// - Calling `destroy()` before changing types
///
/// # Ownership Contract
///
/// - PlgVariant owns the data in its union
/// - Fields wrapped in `ManuallyDrop` have their cleanup handled by C++ `destroy_variant`
/// - Must call `destroy()` exactly once (handled automatically by Drop)
/// - After `destroy()`, the variant is in an invalid state until reconstructed
///
/// # Safety
///
/// This type is only safe to use through its public API. Direct field access is unsafe.
#[repr(C)]
pub struct PlgVariant {
    data: PlgVariantData,
    /// Padding to ensure 32-byte alignment on 32-bit architectures
    #[cfg(target_pointer_width = "32")]
    pad: [u8; 8],
    /// Type discriminant - indicates which union field is currently active
    current: PlgType,
}
const _: () = assert!(size_of::<PlgVariant>() == 32);

impl std::fmt::Debug for PlgVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlgVariant")
            .field("value", &self.get())
            .field("current", &self.current)
            .finish()
    }
}

impl std::fmt::Display for PlgVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.get() {
            PlgAny::Invalid => write!(f, "Invalid"),
            PlgAny::Bool(v) => write!(f, "{}", v),
            PlgAny::Char8(v) => write!(f, "{}", v),
            PlgAny::Char16(v) => write!(f, "{}", v),
            PlgAny::Int8(v) => write!(f, "{}", v),
            PlgAny::Int16(v) => write!(f, "{}", v),
            PlgAny::Int32(v) => write!(f, "{}", v),
            PlgAny::Int64(v) => write!(f, "{}", v),
            PlgAny::UInt8(v) => write!(f, "{}", v),
            PlgAny::UInt16(v) => write!(f, "{}", v),
            PlgAny::UInt32(v) => write!(f, "{}", v),
            PlgAny::UInt64(v) => write!(f, "{}", v),
            PlgAny::Pointer(v) => write!(f, "0x{:x}", v),
            PlgAny::Float(v) => write!(f, "{}", v),
            PlgAny::Double(v) => write!(f, "{}", v),
            PlgAny::String(v) => write!(f, "{}", v),
            PlgAny::ArrayBool(v) => write!(f, "{:?}", v),
            PlgAny::ArrayChar8(v) => write!(f, "{:?}", v),
            PlgAny::ArrayChar16(v) => write!(f, "{:?}", v),
            PlgAny::ArrayInt8(v) => write!(f, "{:?}", v),
            PlgAny::ArrayInt16(v) => write!(f, "{:?}", v),
            PlgAny::ArrayInt32(v) => write!(f, "{:?}", v),
            PlgAny::ArrayInt64(v) => write!(f, "{:?}", v),
            PlgAny::ArrayUInt8(v) => write!(f, "{:?}", v),
            PlgAny::ArrayUInt16(v) => write!(f, "{:?}", v),
            PlgAny::ArrayUInt32(v) => write!(f, "{:?}", v),
            PlgAny::ArrayUInt64(v) => write!(f, "{:?}", v),
            PlgAny::ArrayPointer(v) => write!(f, "{:?}", v),
            PlgAny::ArrayFloat(v) => write!(f, "{:?}", v),
            PlgAny::ArrayDouble(v) => write!(f, "{:?}", v),
            PlgAny::ArrayString(v) => write!(f, "{:?}", v),
            PlgAny::ArrayVector2(v) => write!(f, "{:?}", v),
            PlgAny::ArrayVector3(v) => write!(f, "{:?}", v),
            PlgAny::ArrayVector4(v) => write!(f, "{:?}", v),
            PlgAny::ArrayMatrix4x4(v) => write!(f, "{:?}", v),
            PlgAny::Vector2(v) => write!(f, "{:?}", v),
            PlgAny::Vector3(v) => write!(f, "{:?}", v),
            PlgAny::Vector4(v) => write!(f, "{:?}", v),
        }
    }
}

// ============================================
// Rust-native value type (similar to Go's `any`)
// ============================================

/// Rust-native variant type that owns its data
///
/// This is a safe Rust enum that can hold any of the variant types.
/// Use this for Rust code; it converts to/from PlgVariant for FFI.
#[derive(Debug, Clone)]
pub enum PlgAny {
    Invalid,
    Bool(bool),
    Char8(i8),
    Char16(u16),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Pointer(usize),
    Float(f32),
    Double(f64),
    String(String),
    ArrayBool(Vec<bool>),
    ArrayChar8(Vec<i8>),
    ArrayChar16(Vec<u16>),
    ArrayInt8(Vec<i8>),
    ArrayInt16(Vec<i16>),
    ArrayInt32(Vec<i32>),
    ArrayInt64(Vec<i64>),
    ArrayUInt8(Vec<u8>),
    ArrayUInt16(Vec<u16>),
    ArrayUInt32(Vec<u32>),
    ArrayUInt64(Vec<u64>),
    ArrayPointer(Vec<usize>),
    ArrayFloat(Vec<f32>),
    ArrayDouble(Vec<f64>),
    ArrayString(Vec<String>),
    ArrayVector2(Vec<Vector2>),
    ArrayVector3(Vec<Vector3>),
    ArrayVector4(Vec<Vector4>),
    ArrayMatrix4x4(Vec<Matrix4x4>),
    Vector2(Vector2),
    Vector3(Vector3),
    Vector4(Vector4),
}

// ============================================
// Core implementation
// ============================================

impl PlgVariant {
    /// Create a new PlgVariant from a PlgAny value
    ///
    /// # Panics
    ///
    /// May panic if C++ allocation fails (e.g., for String or Vector types).
    /// The panic is safe - no resources will leak.
    pub fn new(value: &PlgAny) -> Self {
        let mut variant = PlgVariant {
            data: PlgVariantData { int64: 0 },
            #[cfg(target_pointer_width = "32")]
            pad: [0; 8],
            current: PlgType::Invalid,
        };
        variant.construct(value);
        variant
    }

    /// Construct the variant from a PlgAny value
    ///
    /// # Safety
    ///
    /// IMPORTANT: This method does NOT destroy existing data first.
    /// Callers must ensure the variant is in the Invalid state or has been
    /// properly destroyed before calling this.
    ///
    /// The C++ implementation must not throw exceptions. If allocation fails,
    /// the process will abort.
    fn construct(&mut self, value: &PlgAny) {
        /// Macro to assign scalar (Copy) types to the union
        macro_rules! assign_scalar {
            ($field:ident, $variant:expr, $type:expr) => {
                {
                    // SAFETY: We're setting both the union field and discriminant atomically.
                    // Scalar types are Copy and don't need Drop.
                    self.data.$field = *$variant;
                    self.current = $type;
                }
            };
        }

        /// Macro to assign owned (non-Copy) types to the union
        macro_rules! assign_owned {
            ($field:ident, $variant:expr, $type:expr) => {
                {
                    // SAFETY: We're setting both the union field and discriminant atomically.
                    // ManuallyDrop prevents automatic Drop; cleanup is handled by C++.
                    self.data.$field = ManuallyDrop::new($variant);
                    self.current = $type;
                }
            };
        }

        match value {
            PlgAny::Invalid => self.current = PlgType::Invalid,
            PlgAny::Bool(v) => assign_scalar!(boolean, v, PlgType::Bool),
            PlgAny::Char8(v) => assign_scalar!(char8, v, PlgType::Char8),
            PlgAny::Char16(v) => assign_scalar!(char16, v, PlgType::Char16),
            PlgAny::Int8(v) => assign_scalar!(int8, v, PlgType::Int8),
            PlgAny::Int16(v) => assign_scalar!(int16, v, PlgType::Int16),
            PlgAny::Int32(v) => assign_scalar!(int32, v, PlgType::Int32),
            PlgAny::Int64(v) => assign_scalar!(int64, v, PlgType::Int64),
            PlgAny::UInt8(v) => assign_scalar!(uint8, v, PlgType::UInt8),
            PlgAny::UInt16(v) => assign_scalar!(uint16, v, PlgType::UInt16),
            PlgAny::UInt32(v) => assign_scalar!(uint32, v, PlgType::UInt32),
            PlgAny::UInt64(v) => assign_scalar!(uint64, v, PlgType::UInt64),
            PlgAny::Pointer(v) => assign_scalar!(ptr, v, PlgType::Pointer),
            PlgAny::Float(v) => assign_scalar!(flt, v, PlgType::Float),
            PlgAny::Double(v) => assign_scalar!(dbl, v, PlgType::Double),
            PlgAny::String(v) => assign_owned!(str, PlgString::from(v), PlgType::String),
            PlgAny::ArrayBool(v) => assign_owned!(vec_bool, PlgVector::from(v), PlgType::ArrayBool),
            PlgAny::ArrayChar8(v) => assign_owned!(vec_c8, PlgVector::from(v), PlgType::ArrayChar8),
            PlgAny::ArrayChar16(v) => assign_owned!(vec_c16, PlgVector::from(v), PlgType::ArrayChar16),
            PlgAny::ArrayInt8(v) => assign_owned!(vec_i8, PlgVector::from(v), PlgType::ArrayInt8),
            PlgAny::ArrayInt16(v) => assign_owned!(vec_i16, PlgVector::from(v), PlgType::ArrayInt16),
            PlgAny::ArrayInt32(v) => assign_owned!(vec_i32, PlgVector::from(v), PlgType::ArrayInt32),
            PlgAny::ArrayInt64(v) => assign_owned!(vec_i64, PlgVector::from(v), PlgType::ArrayInt64),
            PlgAny::ArrayUInt8(v) => assign_owned!(vec_u8, PlgVector::from(v), PlgType::ArrayUInt8),
            PlgAny::ArrayUInt16(v) => assign_owned!(vec_u16, PlgVector::from(v), PlgType::ArrayUInt16),
            PlgAny::ArrayUInt32(v) => assign_owned!(vec_u32, PlgVector::from(v), PlgType::ArrayUInt32),
            PlgAny::ArrayUInt64(v) => assign_owned!(vec_u64, PlgVector::from(v), PlgType::ArrayUInt64),
            PlgAny::ArrayPointer(v) => assign_owned!(vec_usize, PlgVector::from(v), PlgType::ArrayPointer),
            PlgAny::ArrayFloat(v) => assign_owned!(vec_f32, PlgVector::from(v), PlgType::ArrayFloat),
            PlgAny::ArrayDouble(v) => assign_owned!(vec_f64, PlgVector::from(v), PlgType::ArrayDouble),
            PlgAny::ArrayString(v) => assign_owned!(vec_str, PlgVector::from(v), PlgType::ArrayString),
            PlgAny::ArrayVector2(v) => assign_owned!(vec_vec2, PlgVector::from(v), PlgType::ArrayVector2),
            PlgAny::ArrayVector3(v) => assign_owned!(vec_vec3, PlgVector::from(v), PlgType::ArrayVector3),
            PlgAny::ArrayVector4(v) => assign_owned!(vec_vec4, PlgVector::from(v), PlgType::ArrayVector4),
            PlgAny::ArrayMatrix4x4(v) => assign_owned!(vec_mat4x4, PlgVector::from(v), PlgType::ArrayMatrix4x4),
            PlgAny::Vector2(v) => assign_scalar!(vec2, v, PlgType::Vector2),
            PlgAny::Vector3(v) => assign_scalar!(vec3, v, PlgType::Vector3),
            PlgAny::Vector4(v) => assign_scalar!(vec4, v, PlgType::Vector4),
        }
    }

    /// Set the variant to a new value, destroying the old value first
    ///
    /// This is the safe way to change a variant's value - it properly destroys
    /// the old data before constructing the new data.
    ///
    /// # Safety
    ///
    /// The C++ implementation must not throw exceptions. If allocation fails,
    /// the process will abort.
    pub fn set(&mut self, value: &PlgAny) {
        // Destroy existing data BEFORE constructing new data
        // This prevents memory leaks if the variant already holds data
        self.destroy();
        self.construct(value);
    }

    /// Get the current value as a PlgAny (allocates and copies)
    ///
    /// # Safety
    ///
    /// This is safe because:
    /// - We match on `current` to determine which union field to access
    /// - The type invariant guarantees `current` matches the active union field
    /// - We copy/clone the data, so the original remains valid
    #[must_use = "this allocates and copies data into a new PlgAny"]
    pub fn get(&self) -> PlgAny {
        unsafe {
            match self.current {
                PlgType::Invalid => PlgAny::Invalid,

                // Scalar types: direct copy (they're Copy)
                // SAFETY: current == PlgType::X means data.x is the active field
                PlgType::Bool => PlgAny::Bool(self.data.boolean),
                PlgType::Char8 => PlgAny::Char8(self.data.char8),
                PlgType::Char16 => PlgAny::Char16(self.data.char16),
                PlgType::Int8 => PlgAny::Int8(self.data.int8),
                PlgType::Int16 => PlgAny::Int16(self.data.int16),
                PlgType::Int32 => PlgAny::Int32(self.data.int32),
                PlgType::Int64 => PlgAny::Int64(self.data.int64),
                PlgType::UInt8 => PlgAny::UInt8(self.data.uint8),
                PlgType::UInt16 => PlgAny::UInt16(self.data.uint16),
                PlgType::UInt32 => PlgAny::UInt32(self.data.uint32),
                PlgType::UInt64 => PlgAny::UInt64(self.data.uint64),
                PlgType::Pointer => PlgAny::Pointer(self.data.ptr),
                PlgType::Float => PlgAny::Float(self.data.flt),
                PlgType::Double => PlgAny::Double(self.data.dbl),

                // Owned types: convert to owned Rust types
                // SAFETY: ManuallyDrop doesn't affect reading; we clone/convert the data
                PlgType::String => PlgAny::String(self.data.str.to_string()),
                PlgType::ArrayBool => PlgAny::ArrayBool(self.data.vec_bool.to_vec()),
                PlgType::ArrayChar8 => PlgAny::ArrayChar8(self.data.vec_c8.to_vec()),
                PlgType::ArrayChar16 => PlgAny::ArrayChar16(self.data.vec_c16.to_vec()),
                PlgType::ArrayInt8 => PlgAny::ArrayInt8(self.data.vec_i8.to_vec()),
                PlgType::ArrayInt16 => PlgAny::ArrayInt16(self.data.vec_i16.to_vec()),
                PlgType::ArrayInt32 => PlgAny::ArrayInt32(self.data.vec_i32.to_vec()),
                PlgType::ArrayInt64 => PlgAny::ArrayInt64(self.data.vec_i64.to_vec()),
                PlgType::ArrayUInt8 => PlgAny::ArrayUInt8(self.data.vec_u8.to_vec()),
                PlgType::ArrayUInt16 => PlgAny::ArrayUInt16(self.data.vec_u16.to_vec()),
                PlgType::ArrayUInt32 => PlgAny::ArrayUInt32(self.data.vec_u32.to_vec()),
                PlgType::ArrayUInt64 => PlgAny::ArrayUInt64(self.data.vec_u64.to_vec()),
                PlgType::ArrayPointer => PlgAny::ArrayPointer(self.data.vec_usize.to_vec()),
                PlgType::ArrayFloat => PlgAny::ArrayFloat(self.data.vec_f32.to_vec()),
                PlgType::ArrayDouble => PlgAny::ArrayDouble(self.data.vec_f64.to_vec()),
                PlgType::ArrayString => PlgAny::ArrayString(self.data.vec_str.to_string()),
                PlgType::ArrayVector2 => PlgAny::ArrayVector2(self.data.vec_vec2.to_vec()),
                PlgType::ArrayVector3 => PlgAny::ArrayVector3(self.data.vec_vec3.to_vec()),
                PlgType::ArrayVector4 => PlgAny::ArrayVector4(self.data.vec_vec4.to_vec()),
                PlgType::ArrayMatrix4x4 => PlgAny::ArrayMatrix4x4(self.data.vec_mat4x4.to_vec()),

                // Vector types: direct copy (they're Copy)
                PlgType::Vector2 => PlgAny::Vector2(self.data.vec2),
                PlgType::Vector3 => PlgAny::Vector3(self.data.vec3),
                PlgType::Vector4 => PlgAny::Vector4(self.data.vec4),

                // Unknown/unhandled types
                _ => PlgAny::Invalid,
            }
        }
    }

    /// Get the current type of the variant
    #[must_use]
    pub fn current(&self) -> PlgType {
        self.current
    }

    /// Destroy the variant (manual cleanup)
    ///
    /// This calls the C++ destroy function to free any owned resources.
    /// After calling this, the variant is in an invalid state until reconstructed.
    ///
    /// This is typically not needed as Drop handles cleanup automatically.
    /// Only use this if you need explicit control over when cleanup occurs.
    pub fn destroy(&mut self) {
        destroy_variant(self);
    }
}

impl Drop for PlgVariant {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Clone for PlgVariant {
    fn clone(&self) -> Self {
        PlgVariant::new(&self.get())
    }
}

impl Default for PlgVariant {
    fn default() -> Self {
        PlgVariant::new(&PlgAny::Invalid)
    }
}

// ============================================
// Convenient From implementations
// ============================================

macro_rules! variant_from_primitive {
    ($rust_type:ty, $variant:ident) => {
        impl From<$rust_type> for PlgAny {
            fn from(value: $rust_type) -> Self {
                PlgAny::$variant(value)
            }
        }
    };
}

variant_from_primitive!(bool, Bool);
variant_from_primitive!(i8, Int8);
variant_from_primitive!(i16, Int16);
variant_from_primitive!(i32, Int32);
variant_from_primitive!(i64, Int64);
variant_from_primitive!(u8, UInt8);
variant_from_primitive!(u16, UInt16);
variant_from_primitive!(u32, UInt32);
variant_from_primitive!(u64, UInt64);
variant_from_primitive!(usize, Pointer);
variant_from_primitive!(f32, Float);
variant_from_primitive!(f64, Double);
variant_from_primitive!(Vector2, Vector2);
variant_from_primitive!(Vector3, Vector3);
variant_from_primitive!(Vector4, Vector4);

impl From<String> for PlgAny {
    fn from(value: String) -> Self {
        PlgAny::String(value)
    }
}

impl From<&str> for PlgAny {
    fn from(value: &str) -> Self {
        PlgAny::String(value.to_string())
    }
}

macro_rules! variant_from_vec {
    ($t:ty, $variant:ident) => {
        impl From<Vec<$t>> for PlgAny {
            fn from(value: Vec<$t>) -> Self {
                PlgAny::$variant(value)
            }
        }
    };
}

variant_from_vec!(bool, ArrayBool);
variant_from_vec!(i8, ArrayInt8);
variant_from_vec!(i16, ArrayInt16);
variant_from_vec!(i32, ArrayInt32);
variant_from_vec!(i64, ArrayInt64);
variant_from_vec!(u8, ArrayUInt8);
variant_from_vec!(u16, ArrayUInt16);
variant_from_vec!(u32, ArrayUInt32);
variant_from_vec!(u64, ArrayUInt64);
variant_from_vec!(usize, ArrayPointer);
variant_from_vec!(f32, ArrayFloat);
variant_from_vec!(f64, ArrayDouble);
variant_from_vec!(String, ArrayString);
variant_from_vec!(Vector2, ArrayVector2);
variant_from_vec!(Vector3, ArrayVector3);
variant_from_vec!(Vector4, ArrayVector4);
variant_from_vec!(Matrix4x4, ArrayMatrix4x4);

// PlgVariant From PlgAny
impl From<&PlgAny> for PlgVariant {
    fn from(value: &PlgAny) -> Self {
        PlgVariant::new(value)
    }
}

impl From<PlgAny> for PlgVariant {
    fn from(value: PlgAny) -> Self {
        PlgVariant::new(&value)
    }
}