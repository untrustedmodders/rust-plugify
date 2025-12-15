use std::mem::ManuallyDrop;
use crate::{import_symbol, Str, Arr, Vec2, Vec3, Vec4, Mat4x4};

import_symbol!(destroy_variant, DESTROY_VARIANT, init_destroy_variant, (variant: *mut Var) -> ());

// Variant type enum
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
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
/// This is a C-compatible union. Only the field corresponding to the `Type`
/// discriminant in `Var::current` may be accessed. Accessing the wrong
/// field is undefined behavior.
///
/// Fields wrapped in `ManuallyDrop` must be manually dropped when the variant
/// is destroyed - this is handled by the C++ `destroy_variant` function.
#[repr(C)]
union Data {
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
    str: ManuallyDrop<Str>,
    vec_bool: ManuallyDrop<Arr<bool>>,
    vec_c8: ManuallyDrop<Arr<i8>>,
    vec_c16: ManuallyDrop<Arr<u16>>,
    vec_i8: ManuallyDrop<Arr<i8>>,
    vec_i16: ManuallyDrop<Arr<i16>>,
    vec_i32: ManuallyDrop<Arr<i32>>,
    vec_i64: ManuallyDrop<Arr<i64>>,
    vec_u8: ManuallyDrop<Arr<u8>>,
    vec_u16: ManuallyDrop<Arr<u16>>,
    vec_u32: ManuallyDrop<Arr<u32>>,
    vec_u64: ManuallyDrop<Arr<u64>>,
    vec_usize: ManuallyDrop<Arr<usize>>,
    vec_f32: ManuallyDrop<Arr<f32>>,
    vec_f64: ManuallyDrop<Arr<f64>>,
    vec_str: ManuallyDrop<Arr<Str>>,
    vec_vec2: ManuallyDrop<Arr<Vec2>>,
    vec_vec3: ManuallyDrop<Arr<Vec3>>,
    vec_vec4: ManuallyDrop<Arr<Vec4>>,
    vec_mat4x4: ManuallyDrop<Arr<Mat4x4>>,

    // Vector types (Copy, no Drop)
    vec2: Vec2,
    vec3: Vec3,
    vec4: Vec4,
}

/// FFI-compatible variant type matching the memory layout of C++ plg::variant
///
/// # Memory Layout
///
/// ```text
/// [data: 24 bytes union] [pad: 8 bytes on 32-bit] [type: 1 byte Type]
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
/// - Var owns the data in its union
/// - Fields wrapped in `ManuallyDrop` have their cleanup handled by C++ `destroy_variant`
/// - Must call `destroy()` exactly once (handled automatically by Drop)
/// - After `destroy()`, the variant is in an invalid state until reconstructed
///
/// # Safety
///
/// This type is only safe to use through its public API. Direct field access is unsafe.
#[repr(C)]
pub struct Var {
    data: Data,
    /// Padding to ensure 32-byte alignment on 32-bit architectures
    #[cfg(target_pointer_width = "32")]
    pad: [u8; 8],
    /// Type discriminant - indicates which union field is currently active
    current: Type,
}
const _: () = assert!(size_of::<Var>() == 32);

impl std::fmt::Debug for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Var")
            .field("value", &self.get())
            .field("current", &self.current)
            .finish()
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.get() {
            Any::Invalid => write!(f, "Invalid"),
            Any::Bool(v) => write!(f, "{}", v),
            Any::Char8(v) => write!(f, "{}", v),
            Any::Char16(v) => write!(f, "{}", v),
            Any::Int8(v) => write!(f, "{}", v),
            Any::Int16(v) => write!(f, "{}", v),
            Any::Int32(v) => write!(f, "{}", v),
            Any::Int64(v) => write!(f, "{}", v),
            Any::UInt8(v) => write!(f, "{}", v),
            Any::UInt16(v) => write!(f, "{}", v),
            Any::UInt32(v) => write!(f, "{}", v),
            Any::UInt64(v) => write!(f, "{}", v),
            Any::Pointer(v) => write!(f, "0x{:x}", v),
            Any::Float(v) => write!(f, "{}", v),
            Any::Double(v) => write!(f, "{}", v),
            Any::String(v) => write!(f, "{}", v),
            Any::ArrayBool(v) => write!(f, "{:?}", v),
            Any::ArrayChar8(v) => write!(f, "{:?}", v),
            Any::ArrayChar16(v) => write!(f, "{:?}", v),
            Any::ArrayInt8(v) => write!(f, "{:?}", v),
            Any::ArrayInt16(v) => write!(f, "{:?}", v),
            Any::ArrayInt32(v) => write!(f, "{:?}", v),
            Any::ArrayInt64(v) => write!(f, "{:?}", v),
            Any::ArrayUInt8(v) => write!(f, "{:?}", v),
            Any::ArrayUInt16(v) => write!(f, "{:?}", v),
            Any::ArrayUInt32(v) => write!(f, "{:?}", v),
            Any::ArrayUInt64(v) => write!(f, "{:?}", v),
            Any::ArrayPointer(v) => write!(f, "{:?}", v),
            Any::ArrayFloat(v) => write!(f, "{:?}", v),
            Any::ArrayDouble(v) => write!(f, "{:?}", v),
            Any::ArrayString(v) => write!(f, "{:?}", v),
            Any::ArrayVector2(v) => write!(f, "{:?}", v),
            Any::ArrayVector3(v) => write!(f, "{:?}", v),
            Any::ArrayVector4(v) => write!(f, "{:?}", v),
            Any::ArrayMatrix4x4(v) => write!(f, "{:?}", v),
            Any::Vector2(v) => write!(f, "{:?}", v),
            Any::Vector3(v) => write!(f, "{:?}", v),
            Any::Vector4(v) => write!(f, "{:?}", v),
        }
    }
}

// ============================================
// Rust-native value type (similar to Go's `any`)
// ============================================

/// Rust-native variant type that owns its data
///
/// This is a safe Rust enum that can hold any of the variant types.
/// Use this for Rust code; it converts to/from Var for FFI.
#[derive(Debug, Clone)]
pub enum Any {
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
    ArrayVector2(Vec<Vec2>),
    ArrayVector3(Vec<Vec3>),
    ArrayVector4(Vec<Vec4>),
    ArrayMatrix4x4(Vec<Mat4x4>),
    Vector2(Vec2),
    Vector3(Vec3),
    Vector4(Vec4),
}

// ============================================
// Core implementation
// ============================================

impl Var {
    /// Create a new Var from a Any value
    ///
    /// # Panics
    ///
    /// May panic if C++ allocation fails (e.g., for String or Vector types).
    /// The panic is safe - no resources will leak.
    pub fn new(value: &Any) -> Self {
        let mut variant = Var {
            data: Data { int64: 0 },
            #[cfg(target_pointer_width = "32")]
            pad: [0; 8],
            current: Type::Invalid,
        };
        variant.construct(value);
        variant
    }

    /// Construct the variant from a Any value
    ///
    /// # Safety
    ///
    /// IMPORTANT: This method does NOT destroy existing data first.
    /// Callers must ensure the variant is in the Invalid state or has been
    /// properly destroyed before calling this.
    ///
    /// The C++ implementation must not throw exceptions. If allocation fails,
    /// the process will abort.
    fn construct(&mut self, value: &Any) {
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
            Any::Invalid => self.current = Type::Invalid,
            Any::Bool(v) => assign_scalar!(boolean, v, Type::Bool),
            Any::Char8(v) => assign_scalar!(char8, v, Type::Char8),
            Any::Char16(v) => assign_scalar!(char16, v, Type::Char16),
            Any::Int8(v) => assign_scalar!(int8, v, Type::Int8),
            Any::Int16(v) => assign_scalar!(int16, v, Type::Int16),
            Any::Int32(v) => assign_scalar!(int32, v, Type::Int32),
            Any::Int64(v) => assign_scalar!(int64, v, Type::Int64),
            Any::UInt8(v) => assign_scalar!(uint8, v, Type::UInt8),
            Any::UInt16(v) => assign_scalar!(uint16, v, Type::UInt16),
            Any::UInt32(v) => assign_scalar!(uint32, v, Type::UInt32),
            Any::UInt64(v) => assign_scalar!(uint64, v, Type::UInt64),
            Any::Pointer(v) => assign_scalar!(ptr, v, Type::Pointer),
            Any::Float(v) => assign_scalar!(flt, v, Type::Float),
            Any::Double(v) => assign_scalar!(dbl, v, Type::Double),
            Any::String(v) => assign_owned!(str, Str::from(v), Type::String),
            Any::ArrayBool(v) => assign_owned!(vec_bool, Arr::from(v), Type::ArrayBool),
            Any::ArrayChar8(v) => assign_owned!(vec_c8, Arr::from(v), Type::ArrayChar8),
            Any::ArrayChar16(v) => assign_owned!(vec_c16, Arr::from(v), Type::ArrayChar16),
            Any::ArrayInt8(v) => assign_owned!(vec_i8, Arr::from(v), Type::ArrayInt8),
            Any::ArrayInt16(v) => assign_owned!(vec_i16, Arr::from(v), Type::ArrayInt16),
            Any::ArrayInt32(v) => assign_owned!(vec_i32, Arr::from(v), Type::ArrayInt32),
            Any::ArrayInt64(v) => assign_owned!(vec_i64, Arr::from(v), Type::ArrayInt64),
            Any::ArrayUInt8(v) => assign_owned!(vec_u8, Arr::from(v), Type::ArrayUInt8),
            Any::ArrayUInt16(v) => assign_owned!(vec_u16, Arr::from(v), Type::ArrayUInt16),
            Any::ArrayUInt32(v) => assign_owned!(vec_u32, Arr::from(v), Type::ArrayUInt32),
            Any::ArrayUInt64(v) => assign_owned!(vec_u64, Arr::from(v), Type::ArrayUInt64),
            Any::ArrayPointer(v) => assign_owned!(vec_usize, Arr::from(v), Type::ArrayPointer),
            Any::ArrayFloat(v) => assign_owned!(vec_f32, Arr::from(v), Type::ArrayFloat),
            Any::ArrayDouble(v) => assign_owned!(vec_f64, Arr::from(v), Type::ArrayDouble),
            Any::ArrayString(v) => assign_owned!(vec_str, Arr::from(v), Type::ArrayString),
            Any::ArrayVector2(v) => assign_owned!(vec_vec2, Arr::from(v), Type::ArrayVector2),
            Any::ArrayVector3(v) => assign_owned!(vec_vec3, Arr::from(v), Type::ArrayVector3),
            Any::ArrayVector4(v) => assign_owned!(vec_vec4, Arr::from(v), Type::ArrayVector4),
            Any::ArrayMatrix4x4(v) => assign_owned!(vec_mat4x4, Arr::from(v), Type::ArrayMatrix4x4),
            Any::Vector2(v) => assign_scalar!(vec2, v, Type::Vector2),
            Any::Vector3(v) => assign_scalar!(vec3, v, Type::Vector3),
            Any::Vector4(v) => assign_scalar!(vec4, v, Type::Vector4),
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
    pub fn set(&mut self, value: &Any) {
        // Destroy existing data BEFORE constructing new data
        // This prevents memory leaks if the variant already holds data
        self.destroy();
        self.construct(value);
    }

    /// Get the current value as a Any (allocates and copies)
    ///
    /// # Safety
    ///
    /// This is safe because:
    /// - We match on `current` to determine which union field to access
    /// - The type invariant guarantees `current` matches the active union field
    /// - We copy/clone the data, so the original remains valid
    #[must_use = "this allocates and copies data into a new Any"]
    pub fn get(&self) -> Any {
        unsafe {
            match self.current {
                Type::Invalid => Any::Invalid,

                // Scalar types: direct copy (they're Copy)
                // SAFETY: current == Type::X means data.x is the active field
                Type::Bool => Any::Bool(self.data.boolean),
                Type::Char8 => Any::Char8(self.data.char8),
                Type::Char16 => Any::Char16(self.data.char16),
                Type::Int8 => Any::Int8(self.data.int8),
                Type::Int16 => Any::Int16(self.data.int16),
                Type::Int32 => Any::Int32(self.data.int32),
                Type::Int64 => Any::Int64(self.data.int64),
                Type::UInt8 => Any::UInt8(self.data.uint8),
                Type::UInt16 => Any::UInt16(self.data.uint16),
                Type::UInt32 => Any::UInt32(self.data.uint32),
                Type::UInt64 => Any::UInt64(self.data.uint64),
                Type::Pointer => Any::Pointer(self.data.ptr),
                Type::Float => Any::Float(self.data.flt),
                Type::Double => Any::Double(self.data.dbl),

                // Owned types: convert to owned Rust types
                // SAFETY: ManuallyDrop doesn't affect reading; we clone/convert the data
                Type::String => Any::String(self.data.str.to_string()),
                Type::ArrayBool => Any::ArrayBool(self.data.vec_bool.to_vec()),
                Type::ArrayChar8 => Any::ArrayChar8(self.data.vec_c8.to_vec()),
                Type::ArrayChar16 => Any::ArrayChar16(self.data.vec_c16.to_vec()),
                Type::ArrayInt8 => Any::ArrayInt8(self.data.vec_i8.to_vec()),
                Type::ArrayInt16 => Any::ArrayInt16(self.data.vec_i16.to_vec()),
                Type::ArrayInt32 => Any::ArrayInt32(self.data.vec_i32.to_vec()),
                Type::ArrayInt64 => Any::ArrayInt64(self.data.vec_i64.to_vec()),
                Type::ArrayUInt8 => Any::ArrayUInt8(self.data.vec_u8.to_vec()),
                Type::ArrayUInt16 => Any::ArrayUInt16(self.data.vec_u16.to_vec()),
                Type::ArrayUInt32 => Any::ArrayUInt32(self.data.vec_u32.to_vec()),
                Type::ArrayUInt64 => Any::ArrayUInt64(self.data.vec_u64.to_vec()),
                Type::ArrayPointer => Any::ArrayPointer(self.data.vec_usize.to_vec()),
                Type::ArrayFloat => Any::ArrayFloat(self.data.vec_f32.to_vec()),
                Type::ArrayDouble => Any::ArrayDouble(self.data.vec_f64.to_vec()),
                Type::ArrayString => Any::ArrayString(self.data.vec_str.to_string()),
                Type::ArrayVector2 => Any::ArrayVector2(self.data.vec_vec2.to_vec()),
                Type::ArrayVector3 => Any::ArrayVector3(self.data.vec_vec3.to_vec()),
                Type::ArrayVector4 => Any::ArrayVector4(self.data.vec_vec4.to_vec()),
                Type::ArrayMatrix4x4 => Any::ArrayMatrix4x4(self.data.vec_mat4x4.to_vec()),

                // Vector types: direct copy (they're Copy)
                Type::Vector2 => Any::Vector2(self.data.vec2),
                Type::Vector3 => Any::Vector3(self.data.vec3),
                Type::Vector4 => Any::Vector4(self.data.vec4),

                // Unknown/unhandled types
                _ => Any::Invalid,
            }
        }
    }

    /// Get the current type of the variant
    #[must_use]
    pub fn current(&self) -> Type {
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

impl Drop for Var {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Clone for Var {
    fn clone(&self) -> Self {
        Var::new(&self.get())
    }
}

impl Default for Var {
    fn default() -> Self {
        Var::new(&Any::Invalid)
    }
}

// ============================================
// Convenient From implementations
// ============================================

macro_rules! variant_from_primitive {
    ($rust_type:ty, $variant:ident) => {
        impl From<$rust_type> for Any {
            fn from(value: $rust_type) -> Self {
                Any::$variant(value)
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
variant_from_primitive!(Vec2, Vector2);
variant_from_primitive!(Vec3, Vector3);
variant_from_primitive!(Vec4, Vector4);

impl From<String> for Any {
    fn from(value: String) -> Self {
        Any::String(value)
    }
}

impl From<&str> for Any {
    fn from(value: &str) -> Self {
        Any::String(value.to_string())
    }
}

macro_rules! variant_from_vec {
    ($t:ty, $variant:ident) => {
        impl From<Vec<$t>> for Any {
            fn from(value: Vec<$t>) -> Self {
                Any::$variant(value)
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
variant_from_vec!(Vec2, ArrayVector2);
variant_from_vec!(Vec3, ArrayVector3);
variant_from_vec!(Vec4, ArrayVector4);
variant_from_vec!(Mat4x4, ArrayMatrix4x4);

// Var From Any
impl From<&Any> for Var {
    fn from(value: &Any) -> Self {
        Var::new(value)
    }
}

impl From<Any> for Var {
    fn from(value: Any) -> Self {
        Var::new(&value)
    }
}