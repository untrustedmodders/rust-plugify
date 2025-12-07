use std::sync::{OnceLock};
use crate::{dynlink_impl, PlgString, PlgVariant, Vector2, Vector3, Vector4, Matrix4x4};

// Vector constructors
dynlink_impl!(construct_vector_bool, CONSTRUCT_VECTOR_BOOL, init_construct_vector_bool, (data: *const bool, size: usize) -> PlgVector<bool>);
dynlink_impl!(construct_vector_char8, CONSTRUCT_VECTOR_CHAR8, init_construct_vector_char8, (data: *const i8, size: usize) -> PlgVector<i8>);
dynlink_impl!(construct_vector_char16, CONSTRUCT_VECTOR_CHAR16, init_construct_vector_char16, (data: *const u16, size: usize) -> PlgVector<u16>);
dynlink_impl!(construct_vector_int8, CONSTRUCT_VECTOR_INT8, init_construct_vector_int8, (data: *const i8, size: usize) -> PlgVector<i8>);
dynlink_impl!(construct_vector_int16, CONSTRUCT_VECTOR_INT16, init_construct_vector_int16, (data: *const i16, size: usize) -> PlgVector<i16>);
dynlink_impl!(construct_vector_int32, CONSTRUCT_VECTOR_INT32, init_construct_vector_int32, (data: *const i32, size: usize) -> PlgVector<i32>);
dynlink_impl!(construct_vector_int64, CONSTRUCT_VECTOR_INT64, init_construct_vector_int64, (data: *const i64, size: usize) -> PlgVector<i64>);
dynlink_impl!(construct_vector_uint8, CONSTRUCT_VECTOR_UINT8, init_construct_vector_uint8, (data: *const u8, size: usize) -> PlgVector<u8>);
dynlink_impl!(construct_vector_uint16, CONSTRUCT_VECTOR_UINT16, init_construct_vector_uint16, (data: *const u16, size: usize) -> PlgVector<u16>);
dynlink_impl!(construct_vector_uint32, CONSTRUCT_VECTOR_UINT32, init_construct_vector_uint32, (data: *const u32, size: usize) -> PlgVector<u32>);
dynlink_impl!(construct_vector_uint64, CONSTRUCT_VECTOR_UINT64, init_construct_vector_uint64, (data: *const u64, size: usize) -> PlgVector<u64>);
dynlink_impl!(construct_vector_pointer, CONSTRUCT_VECTOR_POINTER, init_construct_vector_pointer, (data: *const usize, size: usize) -> PlgVector<usize>);
dynlink_impl!(construct_vector_float, CONSTRUCT_VECTOR_FLOAT, init_construct_vector_float, (data: *const f32, size: usize) -> PlgVector<f32>);
dynlink_impl!(construct_vector_double, CONSTRUCT_VECTOR_DOUBLE, init_construct_vector_double, (data: *const f64, size: usize) -> PlgVector<f64>);
dynlink_impl!(construct_vector_string, CONSTRUCT_VECTOR_STRING, init_construct_vector_string, (data: *const PlgString, size: usize) -> PlgVector<PlgString>);
dynlink_impl!(construct_vector_variant, CONSTRUCT_VECTOR_VARIANT, init_construct_vector_variant, (data: *const PlgVariant, size: usize) -> PlgVector<PlgVariant>);
dynlink_impl!(construct_vector_vector2, CONSTRUCT_VECTOR_VECTOR2, init_construct_vector_vector2, (data: *const Vector2, size: usize) -> PlgVector<Vector2>);
dynlink_impl!(construct_vector_vector3, CONSTRUCT_VECTOR_VECTOR3, init_construct_vector_vector3, (data: *const Vector3, size: usize) -> PlgVector<Vector3>);
dynlink_impl!(construct_vector_vector4, CONSTRUCT_VECTOR_VECTOR4, init_construct_vector_vector4, (data: *const Vector4, size: usize) -> PlgVector<Vector4>);
dynlink_impl!(construct_vector_matrix4x4, CONSTRUCT_VECTOR_MATRIX4X4, init_construct_vector_matrix4x4, (data: *const Matrix4x4, size: usize) -> PlgVector<Matrix4x4>);

// Vector destructors
dynlink_impl!(destroy_vector_bool, DESTROY_VECTOR_BOOL, init_destroy_vector_bool, (vec: *mut PlgVector<bool>) -> ());
dynlink_impl!(destroy_vector_char8, DESTROY_VECTOR_CHAR8, init_destroy_vector_char8, (vec: *mut PlgVector<i8>) -> ());
dynlink_impl!(destroy_vector_char16, DESTROY_VECTOR_CHAR16, init_destroy_vector_char16, (vec: *mut PlgVector<u16>) -> ());
dynlink_impl!(destroy_vector_int8, DESTROY_VECTOR_INT8, init_destroy_vector_int8, (vec: *mut PlgVector<i8>) -> ());
dynlink_impl!(destroy_vector_int16, DESTROY_VECTOR_INT16, init_destroy_vector_int16, (vec: *mut PlgVector<i16>) -> ());
dynlink_impl!(destroy_vector_int32, DESTROY_VECTOR_INT32, init_destroy_vector_int32, (vec: *mut PlgVector<i32>) -> ());
dynlink_impl!(destroy_vector_int64, DESTROY_VECTOR_INT64, init_destroy_vector_int64, (vec: *mut PlgVector<i64>) -> ());
dynlink_impl!(destroy_vector_uint8, DESTROY_VECTOR_UINT8, init_destroy_vector_uint8, (vec: *mut PlgVector<u8>) -> ());
dynlink_impl!(destroy_vector_uint16, DESTROY_VECTOR_UINT16, init_destroy_vector_uint16, (vec: *mut PlgVector<u16>) -> ());
dynlink_impl!(destroy_vector_uint32, DESTROY_VECTOR_UINT32, init_destroy_vector_uint32, (vec: *mut PlgVector<u32>) -> ());
dynlink_impl!(destroy_vector_uint64, DESTROY_VECTOR_UINT64, init_destroy_vector_uint64, (vec: *mut PlgVector<u64>) -> ());
dynlink_impl!(destroy_vector_pointer, DESTROY_VECTOR_POINTER, init_destroy_vector_pointer, (vec: *mut PlgVector<usize>) -> ());
dynlink_impl!(destroy_vector_float, DESTROY_VECTOR_FLOAT, init_destroy_vector_float, (vec: *mut PlgVector<f32>) -> ());
dynlink_impl!(destroy_vector_double, DESTROY_VECTOR_DOUBLE, init_destroy_vector_double, (vec: *mut PlgVector<f64>) -> ());
dynlink_impl!(destroy_vector_string, DESTROY_VECTOR_STRING, init_destroy_vector_string, (vec: *mut PlgVector<PlgString>) -> ());
dynlink_impl!(destroy_vector_variant, DESTROY_VECTOR_VARIANT, init_destroy_vector_variant, (vec: *mut PlgVector<PlgVariant>) -> ());
dynlink_impl!(destroy_vector_vector2, DESTROY_VECTOR_VECTOR2, init_destroy_vector_vector2, (vec: *mut PlgVector<Vector2>) -> ());
dynlink_impl!(destroy_vector_vector3, DESTROY_VECTOR_VECTOR3, init_destroy_vector_vector3, (vec: *mut PlgVector<Vector3>) -> ());
dynlink_impl!(destroy_vector_vector4, DESTROY_VECTOR_VECTOR4, init_destroy_vector_vector4, (vec: *mut PlgVector<Vector4>) -> ());
dynlink_impl!(destroy_vector_matrix4x4, DESTROY_VECTOR_MATRIX4X4, init_destroy_vector_matrix4x4, (vec: *mut PlgVector<Matrix4x4>) -> ());

// Vector size getters
dynlink_impl!(get_vector_size_bool, GET_VECTOR_SIZE_BOOL, init_get_vector_size_bool, (vec: *const PlgVector<bool>) -> usize);
dynlink_impl!(get_vector_size_char8, GET_VECTOR_SIZE_CHAR8, init_get_vector_size_char8, (vec: *const PlgVector<i8>) -> usize);
dynlink_impl!(get_vector_size_char16, GET_VECTOR_SIZE_CHAR16, init_get_vector_size_char16, (vec: *const PlgVector<u16>) -> usize);
dynlink_impl!(get_vector_size_int8, GET_VECTOR_SIZE_INT8, init_get_vector_size_int8, (vec: *const PlgVector<i8>) -> usize);
dynlink_impl!(get_vector_size_int16, GET_VECTOR_SIZE_INT16, init_get_vector_size_int16, (vec: *const PlgVector<i16>) -> usize);
dynlink_impl!(get_vector_size_int32, GET_VECTOR_SIZE_INT32, init_get_vector_size_int32, (vec: *const PlgVector<i32>) -> usize);
dynlink_impl!(get_vector_size_int64, GET_VECTOR_SIZE_INT64, init_get_vector_size_int64, (vec: *const PlgVector<i64>) -> usize);
dynlink_impl!(get_vector_size_uint8, GET_VECTOR_SIZE_UINT8, init_get_vector_size_uint8, (vec: *const PlgVector<u8>) -> usize);
dynlink_impl!(get_vector_size_uint16, GET_VECTOR_SIZE_UINT16, init_get_vector_size_uint16, (vec: *const PlgVector<u16>) -> usize);
dynlink_impl!(get_vector_size_uint32, GET_VECTOR_SIZE_UINT32, init_get_vector_size_uint32, (vec: *const PlgVector<u32>) -> usize);
dynlink_impl!(get_vector_size_uint64, GET_VECTOR_SIZE_UINT64, init_get_vector_size_uint64, (vec: *const PlgVector<u64>) -> usize);
dynlink_impl!(get_vector_size_pointer, GET_VECTOR_SIZE_POINTER, init_get_vector_size_pointer, (vec: *const PlgVector<usize>) -> usize);
dynlink_impl!(get_vector_size_float, GET_VECTOR_SIZE_FLOAT, init_get_vector_size_float, (vec: *const PlgVector<f32>) -> usize);
dynlink_impl!(get_vector_size_double, GET_VECTOR_SIZE_DOUBLE, init_get_vector_size_double, (vec: *const PlgVector<f64>) -> usize);
dynlink_impl!(get_vector_size_string, GET_VECTOR_SIZE_STRING, init_get_vector_size_string, (vec: *const PlgVector<PlgString>) -> usize);
dynlink_impl!(get_vector_size_variant, GET_VECTOR_SIZE_VARIANT, init_get_vector_size_variant, (vec: *const PlgVector<PlgVariant>) -> usize);
dynlink_impl!(get_vector_size_vector2, GET_VECTOR_SIZE_VECTOR2, init_get_vector_size_vector2, (vec: *const PlgVector<Vector2>) -> usize);
dynlink_impl!(get_vector_size_vector3, GET_VECTOR_SIZE_VECTOR3, init_get_vector_size_vector3, (vec: *const PlgVector<Vector3>) -> usize);
dynlink_impl!(get_vector_size_vector4, GET_VECTOR_SIZE_VECTOR4, init_get_vector_size_vector4, (vec: *const PlgVector<Vector4>) -> usize);
dynlink_impl!(get_vector_size_matrix4x4, GET_VECTOR_SIZE_MATRIX4X4, init_get_vector_size_matrix4x4, (vec: *const PlgVector<Matrix4x4>) -> usize);

// Vector data getters
dynlink_impl!(get_vector_data_bool, GET_VECTOR_DATA_BOOL, init_get_vector_data_bool, (vec: *const PlgVector<bool>) -> *const bool);
dynlink_impl!(get_vector_data_char8, GET_VECTOR_DATA_CHAR8, init_get_vector_data_char8, (vec: *const PlgVector<i8>) -> *const i8);
dynlink_impl!(get_vector_data_char16, GET_VECTOR_DATA_CHAR16, init_get_vector_data_char16, (vec: *const PlgVector<u16>) -> *const u16);
dynlink_impl!(get_vector_data_int8, GET_VECTOR_DATA_INT8, init_get_vector_data_int8, (vec: *const PlgVector<i8>) -> *const i8);
dynlink_impl!(get_vector_data_int16, GET_VECTOR_DATA_INT16, init_get_vector_data_int16, (vec: *const PlgVector<i16>) -> *const i16);
dynlink_impl!(get_vector_data_int32, GET_VECTOR_DATA_INT32, init_get_vector_data_int32, (vec: *const PlgVector<i32>) -> *const i32);
dynlink_impl!(get_vector_data_int64, GET_VECTOR_DATA_INT64, init_get_vector_data_int64, (vec: *const PlgVector<i64>) -> *const i64);
dynlink_impl!(get_vector_data_uint8, GET_VECTOR_DATA_UINT8, init_get_vector_data_uint8, (vec: *const PlgVector<u8>) -> *const u8);
dynlink_impl!(get_vector_data_uint16, GET_VECTOR_DATA_UINT16, init_get_vector_data_uint16, (vec: *const PlgVector<u16>) -> *const u16);
dynlink_impl!(get_vector_data_uint32, GET_VECTOR_DATA_UINT32, init_get_vector_data_uint32, (vec: *const PlgVector<u32>) -> *const u32);
dynlink_impl!(get_vector_data_uint64, GET_VECTOR_DATA_UINT64, init_get_vector_data_uint64, (vec: *const PlgVector<u64>) -> *const u64);
dynlink_impl!(get_vector_data_pointer, GET_VECTOR_DATA_POINTER, init_get_vector_data_pointer, (vec: *const PlgVector<usize>) -> *const usize);
dynlink_impl!(get_vector_data_float, GET_VECTOR_DATA_FLOAT, init_get_vector_data_float, (vec: *const PlgVector<f32>) -> *const f32);
dynlink_impl!(get_vector_data_double, GET_VECTOR_DATA_DOUBLE, init_get_vector_data_double, (vec: *const PlgVector<f64>) -> *const f64);
dynlink_impl!(get_vector_data_string, GET_VECTOR_DATA_STRING, init_get_vector_data_string, (vec: *const PlgVector<PlgString>) -> *const PlgString);
dynlink_impl!(get_vector_data_variant, GET_VECTOR_DATA_VARIANT, init_get_vector_data_variant, (vec: *const PlgVector<PlgVariant>) -> *const PlgVariant);
dynlink_impl!(get_vector_data_vector2, GET_VECTOR_DATA_VECTOR2, init_get_vector_data_vector2, (vec: *const PlgVector<Vector2>) -> *const Vector2);
dynlink_impl!(get_vector_data_vector3, GET_VECTOR_DATA_VECTOR3, init_get_vector_data_vector3, (vec: *const PlgVector<Vector3>) -> *const Vector3);
dynlink_impl!(get_vector_data_vector4, GET_VECTOR_DATA_VECTOR4, init_get_vector_data_vector4, (vec: *const PlgVector<Vector4>) -> *const Vector4);
dynlink_impl!(get_vector_data_matrix4x4, GET_VECTOR_DATA_MATRIX4X4, init_get_vector_data_matrix4x4, (vec: *const PlgVector<Matrix4x4>) -> *const Matrix4x4);

// Vector assign
dynlink_impl!(assign_vector_bool, ASSIGN_VECTOR_BOOL, init_assign_vector_bool, (vec: *mut PlgVector<bool>, data: *const bool, size: usize) -> ());
dynlink_impl!(assign_vector_char8, ASSIGN_VECTOR_CHAR8, init_assign_vector_char8, (vec: *mut PlgVector<i8>, data: *const i8, size: usize) -> ());
dynlink_impl!(assign_vector_char16, ASSIGN_VECTOR_CHAR16, init_assign_vector_char16, (vec: *mut PlgVector<u16>, data: *const u16, size: usize) -> ());
dynlink_impl!(assign_vector_int8, ASSIGN_VECTOR_INT8, init_assign_vector_int8, (vec: *mut PlgVector<i8>, data: *const i8, size: usize) -> ());
dynlink_impl!(assign_vector_int16, ASSIGN_VECTOR_INT16, init_assign_vector_int16, (vec: *mut PlgVector<i16>, data: *const i16, size: usize) -> ());
dynlink_impl!(assign_vector_int32, ASSIGN_VECTOR_INT32, init_assign_vector_int32, (vec: *mut PlgVector<i32>, data: *const i32, size: usize) -> ());
dynlink_impl!(assign_vector_int64, ASSIGN_VECTOR_INT64, init_assign_vector_int64, (vec: *mut PlgVector<i64>, data: *const i64, size: usize) -> ());
dynlink_impl!(assign_vector_uint8, ASSIGN_VECTOR_UINT8, init_assign_vector_uint8, (vec: *mut PlgVector<u8>, data: *const u8, size: usize) -> ());
dynlink_impl!(assign_vector_uint16, ASSIGN_VECTOR_UINT16, init_assign_vector_uint16, (vec: *mut PlgVector<u16>, data: *const u16, size: usize) -> ());
dynlink_impl!(assign_vector_uint32, ASSIGN_VECTOR_UINT32, init_assign_vector_uint32, (vec: *mut PlgVector<u32>, data: *const u32, size: usize) -> ());
dynlink_impl!(assign_vector_uint64, ASSIGN_VECTOR_UINT64, init_assign_vector_uint64, (vec: *mut PlgVector<u64>, data: *const u64, size: usize) -> ());
dynlink_impl!(assign_vector_pointer, ASSIGN_VECTOR_POINTER, init_assign_vector_pointer, (vec: *mut PlgVector<usize>, data: *const usize, size: usize) -> ());
dynlink_impl!(assign_vector_float, ASSIGN_VECTOR_FLOAT, init_assign_vector_float, (vec: *mut PlgVector<f32>, data: *const f32, size: usize) -> ());
dynlink_impl!(assign_vector_double, ASSIGN_VECTOR_DOUBLE, init_assign_vector_double, (vec: *mut PlgVector<f64>, data: *const f64, size: usize) -> ());
dynlink_impl!(assign_vector_string, ASSIGN_VECTOR_STRING, init_assign_vector_string, (vec: *mut PlgVector<PlgString>, data: *const PlgString, size: usize) -> ());
dynlink_impl!(assign_vector_variant, ASSIGN_VECTOR_VARIANT, init_assign_vector_variant, (vec: *mut PlgVector<PlgVariant>, data: *const PlgVariant, size: usize) -> ());
dynlink_impl!(assign_vector_vector2, ASSIGN_VECTOR_VECTOR2, init_assign_vector_vector2, (vec: *mut PlgVector<Vector2>, data: *const Vector2, size: usize) -> ());
dynlink_impl!(assign_vector_vector3, ASSIGN_VECTOR_VECTOR3, init_assign_vector_vector3, (vec: *mut PlgVector<Vector3>, data: *const Vector3, size: usize) -> ());
dynlink_impl!(assign_vector_vector4, ASSIGN_VECTOR_VECTOR4, init_assign_vector_vector4, (vec: *mut PlgVector<Vector4>, data: *const Vector4, size: usize) -> ());
dynlink_impl!(assign_vector_matrix4x4, ASSIGN_VECTOR_MATRIX4X4, init_assign_vector_matrix4x4, (vec: *mut PlgVector<Matrix4x4>, data: *const Matrix4x4, size: usize) -> ());

#[repr(C)]
#[derive(Debug)]
pub struct PlgVector<T: PlgVectorOps> {
    pub begin: *mut T,
    pub end: *mut T,
    pub capacity: *mut T,
}
const _: () = assert!(size_of::<PlgVector<usize>>() == 3 * size_of::<*const ()>());

// ============================================
// Trait definitions
// ============================================

/// Unified trait for all PlgVector operations
pub trait PlgVectorOps: Sized {
    fn construct(data: &[Self]) -> PlgVector<Self>;
    fn destroy(vec: &mut PlgVector<Self>);
    fn len(vec: &PlgVector<Self>) -> usize;
    fn data(vec: &PlgVector<Self>) -> *const Self;
    fn assign(vec: &mut PlgVector<Self>, data: &[Self]);

    /// Get data as slice (zero-copy view)
    fn as_slice(vec: &PlgVector<Self>) -> &[Self] {
        unsafe {
            let len = Self::len(vec);
            if len == 0 { return &[]; }
            let data = Self::data(vec);
            std::slice::from_raw_parts(data, len)
        }
    }

    /// Get data as Vec (copy)
    fn to_vec(vec: &PlgVector<Self>) -> Vec<Self> where Self: Clone {
        Self::as_slice(vec).to_vec()
    }
}

/// Marker trait for C-compatible enums with a specific integer representation
///
/// # Safety
///
/// Implementors must guarantee that:
/// - The type has `#[repr(IntType)]` where IntType is the associated `ReprInt` type
/// - The type has the exact same memory layout as `ReprInt`
/// - All bit patterns valid for `ReprInt` represent valid enum values (or you handle invalid values safely)
pub unsafe trait CEnumRepr: Sized + Copy {
    /// The underlying integer type (i8, i16, i32, i64, u8, u16, u32, u64)
    type ReprInt: PlgVectorOps + Copy;
}

/// Automatic implementation of PlgVectorOps for enums that implement CEnumRepr
impl<E: CEnumRepr> PlgVectorOps for E {
    fn construct(data: &[Self]) -> PlgVector<Self> {
        unsafe {
            // Cast enum slice to integer slice
            let int_data = std::slice::from_raw_parts(
                data.as_ptr() as *const E::ReprInt,
                data.len()
            );
            // Construct vector using integer type's implementation
            let int_vec = E::ReprInt::construct(int_data);
            // Transmute the PlgVector<ReprInt> to PlgVector<E>
            std::mem::transmute(int_vec)
        }
    }

    fn destroy(vec: &mut PlgVector<Self>) {
        unsafe {
            // Cast to PlgVector<ReprInt> and destroy
            let int_vec: &mut PlgVector<E::ReprInt> = std::mem::transmute(vec);
            E::ReprInt::destroy(int_vec);
        }
    }

    fn len(vec: &PlgVector<Self>) -> usize {
        unsafe {
            let int_vec: &PlgVector<E::ReprInt> = std::mem::transmute(vec);
            E::ReprInt::len(int_vec)
        }
    }

    fn data(vec: &PlgVector<Self>) -> *const Self {
        unsafe {
            let int_vec: &PlgVector<E::ReprInt> = std::mem::transmute(vec);
            E::ReprInt::data(int_vec) as *const Self
        }
    }

    fn assign(vec: &mut PlgVector<Self>, data: &[Self]) {
        unsafe {
            let int_vec: &mut PlgVector<E::ReprInt> = std::mem::transmute(vec);
            let int_data = std::slice::from_raw_parts(
                data.as_ptr() as *const E::ReprInt,
                data.len()
            );
            E::ReprInt::assign(int_vec, int_data);
        }
    }
}

#[macro_export]
macro_rules! impl_vector_traits {
    (
        $t:ty,
        $construct:path,
        $destroy:path,
        $len:path,
        $data:path,
        $assign:path
    ) => {
        impl PlgVectorOps for $t {
            fn construct(data: &[Self]) -> PlgVector<$t> {
                $construct(data.as_ptr(), data.len())
            }

            fn destroy(vec: &mut PlgVector<$t>) {
                $destroy(vec)
            }

            fn len(vec: &PlgVector<$t>) -> usize {
                $len(vec)
            }

            fn data(vec: &PlgVector<$t>) -> *const Self {
                $data(vec)
            }

            fn assign(vec: &mut PlgVector<$t>, data: &[Self]) {
                $assign(vec, data.as_ptr(), data.len())
            }
        }
    };
}

impl_vector_traits!(
    bool,
    construct_vector_bool,
    destroy_vector_bool,
    get_vector_size_bool,
    get_vector_data_bool,
    assign_vector_bool
);

impl_vector_traits!(
    i8,
    construct_vector_int8,
    destroy_vector_int8,
    get_vector_size_int8,
    get_vector_data_int8,
    assign_vector_int8
);

impl_vector_traits!(
    i16,
    construct_vector_int16,
    destroy_vector_int16,
    get_vector_size_int16,
    get_vector_data_int16,
    assign_vector_int16
);

impl_vector_traits!(
    i32,
    construct_vector_int32,
    destroy_vector_int32,
    get_vector_size_int32,
    get_vector_data_int32,
    assign_vector_int32
);

impl_vector_traits!(
    i64,
    construct_vector_int64,
    destroy_vector_int64,
    get_vector_size_int64,
    get_vector_data_int64,
    assign_vector_int64
);

impl_vector_traits!(
    u8,
    construct_vector_uint8,
    destroy_vector_uint8,
    get_vector_size_uint8,
    get_vector_data_uint8,
    assign_vector_uint8
);

impl_vector_traits!(
    u16,
    construct_vector_uint16,
    destroy_vector_uint16,
    get_vector_size_uint16,
    get_vector_data_uint16,
    assign_vector_uint16
);

impl_vector_traits!(
    u32,
    construct_vector_uint32,
    destroy_vector_uint32,
    get_vector_size_uint32,
    get_vector_data_uint32,
    assign_vector_uint32
);

impl_vector_traits!(
    u64,
    construct_vector_uint64,
    destroy_vector_uint64,
    get_vector_size_uint64,
    get_vector_data_uint64,
    assign_vector_uint64
);

impl_vector_traits!(
    usize,
    construct_vector_pointer,
    destroy_vector_pointer,
    get_vector_size_pointer,
    get_vector_data_pointer,
    assign_vector_pointer
);

impl_vector_traits!(
    f32,
    construct_vector_float,
    destroy_vector_float,
    get_vector_size_float,
    get_vector_data_float,
    assign_vector_float
);

impl_vector_traits!(
    f64,
    construct_vector_double,
    destroy_vector_double,
    get_vector_size_double,
    get_vector_data_double,
    assign_vector_double
);

impl_vector_traits!(
    Vector2,
    construct_vector_vector2,
    destroy_vector_vector2,
    get_vector_size_vector2,
    get_vector_data_vector2,
    assign_vector_vector2
);

impl_vector_traits!(
    Vector3,
    construct_vector_vector3,
    destroy_vector_vector3,
    get_vector_size_vector3,
    get_vector_data_vector3,
    assign_vector_vector3
);

impl_vector_traits!(
    Vector4,
    construct_vector_vector4,
    destroy_vector_vector4,
    get_vector_size_vector4,
    get_vector_data_vector4,
    assign_vector_vector4
);

impl_vector_traits!(
    Matrix4x4,
    construct_vector_matrix4x4,
    destroy_vector_matrix4x4,
    get_vector_size_matrix4x4,
    get_vector_data_matrix4x4,
    assign_vector_matrix4x4
);

impl_vector_traits!(
    PlgString,
    construct_vector_string,
    destroy_vector_string,
    get_vector_size_string,
    get_vector_data_string,
    assign_vector_string
);

impl_vector_traits!(
    PlgVariant,
    construct_vector_variant,
    destroy_vector_variant,
    get_vector_size_variant,
    get_vector_data_variant,
    assign_vector_variant
);

// ============================================
// Generic methods on PlgVector
// ============================================

impl<T: PlgVectorOps> PlgVector<T> {
    /// Construct a new PlgVector from a slice
    pub fn from_slice(data: &[T]) -> Self {
        T::construct(data)
    }

    /// Get the length of the vector
    pub fn len(&self) -> usize {
        T::len(self)
    }

    /// Check if the vector is empty
    pub fn is_empty(&self) -> bool {
        T::len(self) == 0
    }

    /// Get data as a slice (zero-copy view)
    pub fn as_slice(&self) -> &[T] {
        T::as_slice(self)
    }

    /// Get data as a Rust-owned Vec
    pub fn to_vec(&self) -> Vec<T> where T: Clone {
        T::to_vec(self)
    }

    /// Assign new data to the vector
    pub fn assign(&mut self, data: &[T]) {
        T::assign(self, data);
    }

    /// Destroy the vector (manual cleanup)
    pub fn destroy(&mut self) {
        T::destroy(self);
    }
}

impl<T: PlgVectorOps> Drop for PlgVector<T>  {
    fn drop(&mut self) {
        T::destroy(self);
    }
}

impl PlgVector<PlgString> {
    /// Convert to Vec<String>
    pub fn to_strings(&self) -> Vec<String> {
        self.as_slice()
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
}

impl From<&[String]> for PlgVector<PlgString> {
    fn from(data: &[String]) -> Self {
        let views: Vec<PlgString> = data.iter()
            .map(|s| PlgString::new(s))
            .collect();
        PlgVector::from_slice(&views)
    }
}

// Or for Vec<String>
impl From<Vec<String>> for PlgVector<PlgString> {
    fn from(data: Vec<String>) -> Self {
        PlgVector::from(&data[..])
    }
}

// ============================================
// Convenient From implementations
// ============================================

#[macro_export]
macro_rules! impl_from_vec {
    ($t:ty) => {
        impl From<Vec<$t>> for PlgVector<$t> {
            fn from(value: Vec<$t>) -> Self {
                PlgVector::from_slice(&value)
            }
        }
    };
}

impl_from_vec!(bool);
impl_from_vec!(i8);
impl_from_vec!(i16);
impl_from_vec!(i32);
impl_from_vec!(i64);
impl_from_vec!(u8);
impl_from_vec!(u16);
impl_from_vec!(u32);
impl_from_vec!(u64);
impl_from_vec!(usize);
impl_from_vec!(f32);
impl_from_vec!(f64);
impl_from_vec!(PlgString);
impl_from_vec!(PlgVariant);
impl_from_vec!(Vector2);
impl_from_vec!(Vector3);
impl_from_vec!(Vector4);
impl_from_vec!(Matrix4x4);

#[macro_export]
macro_rules! impl_from_slice {
    ($t:ty) => {
        impl From<&[$t]> for PlgVector<$t> {
            fn from(value: &[$t]) -> Self {
                PlgVector::from_slice(value)
            }
        }
    };
}

impl_from_slice!(bool);
impl_from_slice!(i8);
impl_from_slice!(i16);
impl_from_slice!(i32);
impl_from_slice!(i64);
impl_from_slice!(u8);
impl_from_slice!(u16);
impl_from_slice!(u32);
impl_from_slice!(u64);
impl_from_slice!(usize);
impl_from_slice!(f32);
impl_from_slice!(f64);
impl_from_slice!(PlgString);
impl_from_slice!(PlgVariant);
impl_from_slice!(Vector2);
impl_from_slice!(Vector3);
impl_from_slice!(Vector4);
impl_from_slice!(Matrix4x4);

// ============================================
// Helper macro for C-compatible enums
// ============================================

#[macro_export]
macro_rules! impl_cenum_repr {
    ($enum_ty:ty, i8) => {
        unsafe impl $crate::CEnumRepr for $enum_ty {
            type ReprInt = i8;
        }
    };
    ($enum_ty:ty, i16) => {
        unsafe impl $crate::CEnumRepr for $enum_ty {
            type ReprInt = i16;
        }
    };
    ($enum_ty:ty, i32) => {
        unsafe impl $crate::CEnumRepr for $enum_ty {
            type ReprInt = i32;
        }
    };
    ($enum_ty:ty, i64) => {
        unsafe impl $crate::CEnumRepr for $enum_ty {
            type ReprInt = i64;
        }
    };
    ($enum_ty:ty, u8) => {
        unsafe impl $crate::CEnumRepr for $enum_ty {
            type ReprInt = u8;
        }
    };
    ($enum_ty:ty, u16) => {
        unsafe impl $crate::CEnumRepr for $enum_ty {
            type ReprInt = u16;
        }
    };
    ($enum_ty:ty, u32) => {
        unsafe impl $crate::CEnumRepr for $enum_ty {
            type ReprInt = u32;
        }
    };
    ($enum_ty:ty, u64) => {
        unsafe impl $crate::CEnumRepr for $enum_ty {
            type ReprInt = u64;
        }
    };
}
