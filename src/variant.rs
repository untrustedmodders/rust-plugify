use std::mem::{ManuallyDrop};
use std::sync::{OnceLock};
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
    pub current: PlgType,
}
const _: () = assert!(size_of::<PlgVariant>() == 32);

impl std::fmt::Debug for PlgVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlgVariant")
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
        variant.assign(value);
        variant
    }

    pub fn assign(&mut self, value: &PlgAny) {
        match value {
            PlgAny::Invalid => {
                self.current = PlgType::Invalid;
            }
            PlgAny::Bool(v) => {
                self.data.boolean = *v;
                self.current = PlgType::Bool;
            }
            PlgAny::Char8(v) => {
                self.data.char8 = *v;
                self.current = PlgType::Char8;
            }
            PlgAny::Char16(v) => {
                self.data.char16 = *v;
                self.current = PlgType::Char16;
            }
            PlgAny::Int8(v) => {
                self.data.int8 = *v;
                self.current = PlgType::Int8;
            }
            PlgAny::Int16(v) => {
                self.data.int16 = *v;
                self.current = PlgType::Int16;
            }
            PlgAny::Int32(v) => {
                self.data.int32 = *v;
                self.current = PlgType::Int32;
            }
            PlgAny::Int64(v) => {
                self.data.int64 = *v;
                self.current = PlgType::Int64;
            }
            PlgAny::UInt8(v) => {
                self.data.uint8 = *v;
                self.current = PlgType::UInt8;
            }
            PlgAny::UInt16(v) => {
                self.data.uint16 = *v;
                self.current = PlgType::UInt16;
            }
            PlgAny::UInt32(v) => {
                self.data.uint32 = *v;
                self.current = PlgType::UInt32;
            }
            PlgAny::UInt64(v) => {
                self.data.uint64 = *v;
                self.current = PlgType::UInt64;
            }
            PlgAny::Pointer(v) => {
                self.data.ptr = *v;
                self.current = PlgType::Pointer;
            }
            PlgAny::Float(v) => {
                self.data.flt = *v;
                self.current = PlgType::Float;
            }
            PlgAny::Double(v) => {
                self.data.dbl = *v;
                self.current = PlgType::Double;
            }
            PlgAny::String(v) => {
                self.data.str = ManuallyDrop::new(PlgString::new(v));
                self.current = PlgType::String;
            }
            PlgAny::ArrayBool(v) => {
                self.data.vec_bool = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayBool;
            }
            PlgAny::ArrayChar8(v) => {
                self.data.vec_c8 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayChar8;
            }
            PlgAny::ArrayChar16(v) => {
                self.data.vec_c16 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayChar16;
            }
            PlgAny::ArrayInt8(v) => {
                self.data.vec_i8 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayInt8;
            }
            PlgAny::ArrayInt16(v) => {
                self.data.vec_i16 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayInt16;
            }
            PlgAny::ArrayInt32(v) => {
                self.data.vec_i32 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayInt32;
            }
            PlgAny::ArrayInt64(v) => {
                self.data.vec_i64 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayInt64;
            }
            PlgAny::ArrayUInt8(v) => {
                self.data.vec_u8 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayUInt8;
            }
            PlgAny::ArrayUInt16(v) => {
                self.data.vec_u16 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayUInt16;
            }
            PlgAny::ArrayUInt32(v) => {
                self.data.vec_u32 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayUInt32;
            }
            PlgAny::ArrayUInt64(v) => {
                self.data.vec_u64 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayUInt64;
            }
            PlgAny::ArrayPointer(v) => {
                self.data.vec_usize = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayPointer;
            }
            PlgAny::ArrayFloat(v) => {
                self.data.vec_f32 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayFloat;
            }
            PlgAny::ArrayDouble(v) => {
                self.data.vec_f64 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayDouble;
            }
            PlgAny::ArrayString(v) => {
                self.data.vec_str = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayString;
            }
            PlgAny::ArrayVector2(v) => {
                self.data.vec_vec2 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayVector2;
            }
            PlgAny::ArrayVector3(v) => {
                self.data.vec_vec3 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayVector3;
            }
            PlgAny::ArrayVector4(v) => {
                self.data.vec_vec4 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayVector4;
            }
            PlgAny::ArrayMatrix4x4(v) => {
                self.data.vec_mat4x4 = ManuallyDrop::new(PlgVector::from(v.as_slice()));
                self.current = PlgType::ArrayMatrix4x4;
            }
            PlgAny::Vector2(v) => {
                self.data.vec2 = *v;
                self.current = PlgType::Vector2;
            }
            PlgAny::Vector3(v) => {
                self.data.vec3 = *v;
                self.current = PlgType::Vector3;
            }
            PlgAny::Vector4(v) => {
                self.data.vec4 = *v;
                self.current = PlgType::Vector4;
            }
        }
    }

    pub fn get_value(&self) -> PlgAny {
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
                PlgType::ArrayString => PlgAny::ArrayString(self.data.vec_str.to_strings()),
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

    pub fn destroy(&mut self) {
        destroy_variant(self);
    }
}

impl Drop for PlgVariant {
    fn drop(&mut self) { self.destroy(); }
}

// ============================================
// Convenient From implementations
// ============================================

#[macro_export]
macro_rules! impl_from_primitive {
    ($rust_type:ty, $variant:ident) => {
        impl From<$rust_type> for PlgAny {
            fn from(value: $rust_type) -> Self {
                PlgAny::$variant(value)
            }
        }
    };
}

impl_from_primitive!(bool, Bool);
impl_from_primitive!(i8, Int8);
impl_from_primitive!(i16, Int16);
impl_from_primitive!(i32, Int32);
impl_from_primitive!(i64, Int64);
impl_from_primitive!(u8, UInt8);
impl_from_primitive!(u16, UInt16);
impl_from_primitive!(u32, UInt32);
impl_from_primitive!(u64, UInt64);
impl_from_primitive!(usize, Pointer);
impl_from_primitive!(f32, Float);
impl_from_primitive!(f64, Double);
impl_from_primitive!(Vector2, Vector2);
impl_from_primitive!(Vector3, Vector3);
impl_from_primitive!(Vector4, Vector4);

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

#[macro_export]
macro_rules! impl_from_vec {
    ($t:ty, $variant:ident) => {
        impl From<Vec<$t>> for PlgAny {
            fn from(value: Vec<$t>) -> Self {
                PlgAny::$variant(value)
            }
        }
    };
}

// Usage
impl_from_vec!(bool, ArrayBool);
impl_from_vec!(i8, ArrayInt8);
impl_from_vec!(i16, ArrayInt16);
impl_from_vec!(i32, ArrayInt32);
impl_from_vec!(i64, ArrayInt64);
impl_from_vec!(u8, ArrayUInt8);
impl_from_vec!(u16, ArrayUInt16);
impl_from_vec!(u32, ArrayUInt32);
impl_from_vec!(u64, ArrayUInt64);
impl_from_vec!(usize, ArrayPointer);
impl_from_vec!(f32, ArrayFloat);
impl_from_vec!(f64, ArrayDouble);
impl_from_vec!(String, ArrayString);
impl_from_vec!(Vector2, ArrayVector2);
impl_from_vec!(Vector3, ArrayVector3);
impl_from_vec!(Vector4, ArrayVector4);
impl_from_vec!(Matrix4x4, ArrayMatrix4x4);

// PlgVariant From PlgAny
impl From<PlgAny> for PlgVariant {
    fn from(value: PlgAny) -> Self {
        PlgVariant::new(&value)
    }
}