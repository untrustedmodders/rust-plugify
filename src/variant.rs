use std::mem::{ManuallyDrop};
use std::sync::OnceLock;
use crate::{dynlink_impl, PlgString, PlgVector, Vector2, Vector3, Vector4, Matrix4x4};

dynlink_impl!(destroy_variant, DESTROY_VARIANT, init_destroy_variant, (variant: *mut PlgVariant) -> ());

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

// Union matching C layout
#[repr(C)]
union PlgVariantData {
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
    vec2: Vector2,
    vec3: Vector3,
    vec4: Vector4,
}

#[repr(C)]
pub struct PlgVariant {
    data: PlgVariantData,
    #[cfg(target_pointer_width = "32")]
    pad: [u8; 8],
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

// ============================================
// Rust-native value type (similar to Go's `any`)
// ============================================

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
    //Matrix4x4(Matrix4x4),
}

// ============================================
// Core implementation
// ============================================

impl PlgVariant {
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

    pub fn construct(&mut self, value: &PlgAny) {
        macro_rules! assign_scalar {
            ($field:ident, $variant:expr, $type:expr) => {
                {
                    self.data.$field = *$variant;
                    self.current = $type;
                }
            };
        }

        macro_rules! assign_owned {
            ($field:ident, $variant:expr, $type:expr) => {
                {
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

    pub fn set(&mut self, value: &PlgAny){
        self.destroy();
        self.construct(value);
    }

    pub fn get(&self) -> PlgAny {
        unsafe {
            match self.current {
                PlgType::Invalid => PlgAny::Invalid,
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
                PlgType::String => PlgAny::String(self.data.str.as_str().to_string()),
                PlgType::ArrayBool => PlgAny::ArrayBool(self.data.vec_bool.to_vec()),
                PlgType::ArrayChar8 => PlgAny::ArrayChar8(self.data.vec_c8.to_vec()),
                PlgType::ArrayChar16 => PlgAny::ArrayChar16(self.data.vec_c16.to_vec()),
                PlgType::ArrayInt8 => PlgAny::ArrayInt8(self.data.vec_i8.to_vec()),
                PlgType::ArrayInt16 => PlgAny::ArrayInt16(self.data.vec_i16.to_vec()),
                PlgType::ArrayInt32 => PlgAny::ArrayInt32(self.data.vec_i32.to_vec()),
                PlgType::ArrayInt64 =>  PlgAny::ArrayInt64(self.data.vec_i64.to_vec()),
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
                PlgType::Vector2 => PlgAny::Vector2(self.data.vec2),
                PlgType::Vector3 => PlgAny::Vector3(self.data.vec3),
                PlgType::Vector4 => PlgAny::Vector4(self.data.vec4),
                _ => PlgAny::Invalid,
            }
        }
    }

    pub fn current(&self) -> PlgType {
        self.current
    }

    pub fn destroy(&mut self) {
        destroy_variant(self);
    }
}

impl Drop for PlgVariant {
    fn drop(&mut self) { self.destroy(); }
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

// Usage
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