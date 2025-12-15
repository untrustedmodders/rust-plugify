use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use crate::{import_symbol, Str, Var, Any, Vec2, Vec3, Vec4, Mat4x4};

// Vector constructors
import_symbol!(construct_vector_bool, CONSTRUCT_VECTOR_BOOL, init_construct_vector_bool, (data: *const bool, size: usize) -> Arr<bool>);
import_symbol!(construct_vector_char8, CONSTRUCT_VECTOR_CHAR8, init_construct_vector_char8, (data: *const i8, size: usize) -> Arr<i8>);
import_symbol!(construct_vector_char16, CONSTRUCT_VECTOR_CHAR16, init_construct_vector_char16, (data: *const u16, size: usize) -> Arr<u16>);
import_symbol!(construct_vector_int8, CONSTRUCT_VECTOR_INT8, init_construct_vector_int8, (data: *const i8, size: usize) -> Arr<i8>);
import_symbol!(construct_vector_int16, CONSTRUCT_VECTOR_INT16, init_construct_vector_int16, (data: *const i16, size: usize) -> Arr<i16>);
import_symbol!(construct_vector_int32, CONSTRUCT_VECTOR_INT32, init_construct_vector_int32, (data: *const i32, size: usize) -> Arr<i32>);
import_symbol!(construct_vector_int64, CONSTRUCT_VECTOR_INT64, init_construct_vector_int64, (data: *const i64, size: usize) -> Arr<i64>);
import_symbol!(construct_vector_uint8, CONSTRUCT_VECTOR_UINT8, init_construct_vector_uint8, (data: *const u8, size: usize) -> Arr<u8>);
import_symbol!(construct_vector_uint16, CONSTRUCT_VECTOR_UINT16, init_construct_vector_uint16, (data: *const u16, size: usize) -> Arr<u16>);
import_symbol!(construct_vector_uint32, CONSTRUCT_VECTOR_UINT32, init_construct_vector_uint32, (data: *const u32, size: usize) -> Arr<u32>);
import_symbol!(construct_vector_uint64, CONSTRUCT_VECTOR_UINT64, init_construct_vector_uint64, (data: *const u64, size: usize) -> Arr<u64>);
import_symbol!(construct_vector_pointer, CONSTRUCT_VECTOR_POINTER, init_construct_vector_pointer, (data: *const usize, size: usize) -> Arr<usize>);
import_symbol!(construct_vector_float, CONSTRUCT_VECTOR_FLOAT, init_construct_vector_float, (data: *const f32, size: usize) -> Arr<f32>);
import_symbol!(construct_vector_double, CONSTRUCT_VECTOR_DOUBLE, init_construct_vector_double, (data: *const f64, size: usize) -> Arr<f64>);
import_symbol!(construct_vector_string, CONSTRUCT_VECTOR_STRING, init_construct_vector_string, (data: *const Str, size: usize) -> Arr<Str>);
import_symbol!(construct_vector_variant, CONSTRUCT_VECTOR_VARIANT, init_construct_vector_variant, (data: *const Var, size: usize) -> Arr<Var>);
import_symbol!(construct_vector_vector2, CONSTRUCT_VECTOR_VECTOR2, init_construct_vector_vector2, (data: *const Vec2, size: usize) -> Arr<Vec2>);
import_symbol!(construct_vector_vector3, CONSTRUCT_VECTOR_VECTOR3, init_construct_vector_vector3, (data: *const Vec3, size: usize) -> Arr<Vec3>);
import_symbol!(construct_vector_vector4, CONSTRUCT_VECTOR_VECTOR4, init_construct_vector_vector4, (data: *const Vec4, size: usize) -> Arr<Vec4>);
import_symbol!(construct_vector_matrix4x4, CONSTRUCT_VECTOR_MATRIX4X4, init_construct_vector_matrix4x4, (data: *const Mat4x4, size: usize) -> Arr<Mat4x4>);

// Vector destructors
import_symbol!(destroy_vector_bool, DESTROY_VECTOR_BOOL, init_destroy_vector_bool, (vec: *mut Arr<bool>) -> ());
import_symbol!(destroy_vector_char8, DESTROY_VECTOR_CHAR8, init_destroy_vector_char8, (vec: *mut Arr<i8>) -> ());
import_symbol!(destroy_vector_char16, DESTROY_VECTOR_CHAR16, init_destroy_vector_char16, (vec: *mut Arr<u16>) -> ());
import_symbol!(destroy_vector_int8, DESTROY_VECTOR_INT8, init_destroy_vector_int8, (vec: *mut Arr<i8>) -> ());
import_symbol!(destroy_vector_int16, DESTROY_VECTOR_INT16, init_destroy_vector_int16, (vec: *mut Arr<i16>) -> ());
import_symbol!(destroy_vector_int32, DESTROY_VECTOR_INT32, init_destroy_vector_int32, (vec: *mut Arr<i32>) -> ());
import_symbol!(destroy_vector_int64, DESTROY_VECTOR_INT64, init_destroy_vector_int64, (vec: *mut Arr<i64>) -> ());
import_symbol!(destroy_vector_uint8, DESTROY_VECTOR_UINT8, init_destroy_vector_uint8, (vec: *mut Arr<u8>) -> ());
import_symbol!(destroy_vector_uint16, DESTROY_VECTOR_UINT16, init_destroy_vector_uint16, (vec: *mut Arr<u16>) -> ());
import_symbol!(destroy_vector_uint32, DESTROY_VECTOR_UINT32, init_destroy_vector_uint32, (vec: *mut Arr<u32>) -> ());
import_symbol!(destroy_vector_uint64, DESTROY_VECTOR_UINT64, init_destroy_vector_uint64, (vec: *mut Arr<u64>) -> ());
import_symbol!(destroy_vector_pointer, DESTROY_VECTOR_POINTER, init_destroy_vector_pointer, (vec: *mut Arr<usize>) -> ());
import_symbol!(destroy_vector_float, DESTROY_VECTOR_FLOAT, init_destroy_vector_float, (vec: *mut Arr<f32>) -> ());
import_symbol!(destroy_vector_double, DESTROY_VECTOR_DOUBLE, init_destroy_vector_double, (vec: *mut Arr<f64>) -> ());
import_symbol!(destroy_vector_string, DESTROY_VECTOR_STRING, init_destroy_vector_string, (vec: *mut Arr<Str>) -> ());
import_symbol!(destroy_vector_variant, DESTROY_VECTOR_VARIANT, init_destroy_vector_variant, (vec: *mut Arr<Var>) -> ());
import_symbol!(destroy_vector_vector2, DESTROY_VECTOR_VECTOR2, init_destroy_vector_vector2, (vec: *mut Arr<Vec2>) -> ());
import_symbol!(destroy_vector_vector3, DESTROY_VECTOR_VECTOR3, init_destroy_vector_vector3, (vec: *mut Arr<Vec3>) -> ());
import_symbol!(destroy_vector_vector4, DESTROY_VECTOR_VECTOR4, init_destroy_vector_vector4, (vec: *mut Arr<Vec4>) -> ());
import_symbol!(destroy_vector_matrix4x4, DESTROY_VECTOR_MATRIX4X4, init_destroy_vector_matrix4x4, (vec: *mut Arr<Mat4x4>) -> ());

// Vector size getters
import_symbol!(get_vector_size_bool, GET_VECTOR_SIZE_BOOL, init_get_vector_size_bool, (vec: *const Arr<bool>) -> usize);
import_symbol!(get_vector_size_char8, GET_VECTOR_SIZE_CHAR8, init_get_vector_size_char8, (vec: *const Arr<i8>) -> usize);
import_symbol!(get_vector_size_char16, GET_VECTOR_SIZE_CHAR16, init_get_vector_size_char16, (vec: *const Arr<u16>) -> usize);
import_symbol!(get_vector_size_int8, GET_VECTOR_SIZE_INT8, init_get_vector_size_int8, (vec: *const Arr<i8>) -> usize);
import_symbol!(get_vector_size_int16, GET_VECTOR_SIZE_INT16, init_get_vector_size_int16, (vec: *const Arr<i16>) -> usize);
import_symbol!(get_vector_size_int32, GET_VECTOR_SIZE_INT32, init_get_vector_size_int32, (vec: *const Arr<i32>) -> usize);
import_symbol!(get_vector_size_int64, GET_VECTOR_SIZE_INT64, init_get_vector_size_int64, (vec: *const Arr<i64>) -> usize);
import_symbol!(get_vector_size_uint8, GET_VECTOR_SIZE_UINT8, init_get_vector_size_uint8, (vec: *const Arr<u8>) -> usize);
import_symbol!(get_vector_size_uint16, GET_VECTOR_SIZE_UINT16, init_get_vector_size_uint16, (vec: *const Arr<u16>) -> usize);
import_symbol!(get_vector_size_uint32, GET_VECTOR_SIZE_UINT32, init_get_vector_size_uint32, (vec: *const Arr<u32>) -> usize);
import_symbol!(get_vector_size_uint64, GET_VECTOR_SIZE_UINT64, init_get_vector_size_uint64, (vec: *const Arr<u64>) -> usize);
import_symbol!(get_vector_size_pointer, GET_VECTOR_SIZE_POINTER, init_get_vector_size_pointer, (vec: *const Arr<usize>) -> usize);
import_symbol!(get_vector_size_float, GET_VECTOR_SIZE_FLOAT, init_get_vector_size_float, (vec: *const Arr<f32>) -> usize);
import_symbol!(get_vector_size_double, GET_VECTOR_SIZE_DOUBLE, init_get_vector_size_double, (vec: *const Arr<f64>) -> usize);
import_symbol!(get_vector_size_string, GET_VECTOR_SIZE_STRING, init_get_vector_size_string, (vec: *const Arr<Str>) -> usize);
import_symbol!(get_vector_size_variant, GET_VECTOR_SIZE_VARIANT, init_get_vector_size_variant, (vec: *const Arr<Var>) -> usize);
import_symbol!(get_vector_size_vector2, GET_VECTOR_SIZE_VECTOR2, init_get_vector_size_vector2, (vec: *const Arr<Vec2>) -> usize);
import_symbol!(get_vector_size_vector3, GET_VECTOR_SIZE_VECTOR3, init_get_vector_size_vector3, (vec: *const Arr<Vec3>) -> usize);
import_symbol!(get_vector_size_vector4, GET_VECTOR_SIZE_VECTOR4, init_get_vector_size_vector4, (vec: *const Arr<Vec4>) -> usize);
import_symbol!(get_vector_size_matrix4x4, GET_VECTOR_SIZE_MATRIX4X4, init_get_vector_size_matrix4x4, (vec: *const Arr<Mat4x4>) -> usize);

// Vector data getters
import_symbol!(get_vector_data_bool, GET_VECTOR_DATA_BOOL, init_get_vector_data_bool, (vec: *const Arr<bool>) -> *mut bool);
import_symbol!(get_vector_data_char8, GET_VECTOR_DATA_CHAR8, init_get_vector_data_char8, (vec: *const Arr<i8>) -> *mut i8);
import_symbol!(get_vector_data_char16, GET_VECTOR_DATA_CHAR16, init_get_vector_data_char16, (vec: *const Arr<u16>) -> *mut u16);
import_symbol!(get_vector_data_int8, GET_VECTOR_DATA_INT8, init_get_vector_data_int8, (vec: *const Arr<i8>) -> *mut i8);
import_symbol!(get_vector_data_int16, GET_VECTOR_DATA_INT16, init_get_vector_data_int16, (vec: *const Arr<i16>) -> *mut i16);
import_symbol!(get_vector_data_int32, GET_VECTOR_DATA_INT32, init_get_vector_data_int32, (vec: *const Arr<i32>) -> *mut i32);
import_symbol!(get_vector_data_int64, GET_VECTOR_DATA_INT64, init_get_vector_data_int64, (vec: *const Arr<i64>) -> *mut i64);
import_symbol!(get_vector_data_uint8, GET_VECTOR_DATA_UINT8, init_get_vector_data_uint8, (vec: *const Arr<u8>) -> *mut u8);
import_symbol!(get_vector_data_uint16, GET_VECTOR_DATA_UINT16, init_get_vector_data_uint16, (vec: *const Arr<u16>) -> *mut u16);
import_symbol!(get_vector_data_uint32, GET_VECTOR_DATA_UINT32, init_get_vector_data_uint32, (vec: *const Arr<u32>) -> *mut u32);
import_symbol!(get_vector_data_uint64, GET_VECTOR_DATA_UINT64, init_get_vector_data_uint64, (vec: *const Arr<u64>) -> *mut u64);
import_symbol!(get_vector_data_pointer, GET_VECTOR_DATA_POINTER, init_get_vector_data_pointer, (vec: *const Arr<usize>) -> *mut usize);
import_symbol!(get_vector_data_float, GET_VECTOR_DATA_FLOAT, init_get_vector_data_float, (vec: *const Arr<f32>) -> *mut f32);
import_symbol!(get_vector_data_double, GET_VECTOR_DATA_DOUBLE, init_get_vector_data_double, (vec: *const Arr<f64>) -> *mut f64);
import_symbol!(get_vector_data_string, GET_VECTOR_DATA_STRING, init_get_vector_data_string, (vec: *const Arr<Str>) -> *mut Str);
import_symbol!(get_vector_data_variant, GET_VECTOR_DATA_VARIANT, init_get_vector_data_variant, (vec: *const Arr<Var>) -> *mut Var);
import_symbol!(get_vector_data_vector2, GET_VECTOR_DATA_VECTOR2, init_get_vector_data_vector2, (vec: *const Arr<Vec2>) -> *mut Vec2);
import_symbol!(get_vector_data_vector3, GET_VECTOR_DATA_VECTOR3, init_get_vector_data_vector3, (vec: *const Arr<Vec3>) -> *mut Vec3);
import_symbol!(get_vector_data_vector4, GET_VECTOR_DATA_VECTOR4, init_get_vector_data_vector4, (vec: *const Arr<Vec4>) -> *mut Vec4);
import_symbol!(get_vector_data_matrix4x4, GET_VECTOR_DATA_MATRIX4X4, init_get_vector_data_matrix4x4, (vec: *const Arr<Mat4x4>) -> *mut Mat4x4);

// Vector assign
import_symbol!(assign_vector_bool, ASSIGN_VECTOR_BOOL, init_assign_vector_bool, (vec: *mut Arr<bool>, data: *const bool, size: usize) -> ());
import_symbol!(assign_vector_char8, ASSIGN_VECTOR_CHAR8, init_assign_vector_char8, (vec: *mut Arr<i8>, data: *const i8, size: usize) -> ());
import_symbol!(assign_vector_char16, ASSIGN_VECTOR_CHAR16, init_assign_vector_char16, (vec: *mut Arr<u16>, data: *const u16, size: usize) -> ());
import_symbol!(assign_vector_int8, ASSIGN_VECTOR_INT8, init_assign_vector_int8, (vec: *mut Arr<i8>, data: *const i8, size: usize) -> ());
import_symbol!(assign_vector_int16, ASSIGN_VECTOR_INT16, init_assign_vector_int16, (vec: *mut Arr<i16>, data: *const i16, size: usize) -> ());
import_symbol!(assign_vector_int32, ASSIGN_VECTOR_INT32, init_assign_vector_int32, (vec: *mut Arr<i32>, data: *const i32, size: usize) -> ());
import_symbol!(assign_vector_int64, ASSIGN_VECTOR_INT64, init_assign_vector_int64, (vec: *mut Arr<i64>, data: *const i64, size: usize) -> ());
import_symbol!(assign_vector_uint8, ASSIGN_VECTOR_UINT8, init_assign_vector_uint8, (vec: *mut Arr<u8>, data: *const u8, size: usize) -> ());
import_symbol!(assign_vector_uint16, ASSIGN_VECTOR_UINT16, init_assign_vector_uint16, (vec: *mut Arr<u16>, data: *const u16, size: usize) -> ());
import_symbol!(assign_vector_uint32, ASSIGN_VECTOR_UINT32, init_assign_vector_uint32, (vec: *mut Arr<u32>, data: *const u32, size: usize) -> ());
import_symbol!(assign_vector_uint64, ASSIGN_VECTOR_UINT64, init_assign_vector_uint64, (vec: *mut Arr<u64>, data: *const u64, size: usize) -> ());
import_symbol!(assign_vector_pointer, ASSIGN_VECTOR_POINTER, init_assign_vector_pointer, (vec: *mut Arr<usize>, data: *const usize, size: usize) -> ());
import_symbol!(assign_vector_float, ASSIGN_VECTOR_FLOAT, init_assign_vector_float, (vec: *mut Arr<f32>, data: *const f32, size: usize) -> ());
import_symbol!(assign_vector_double, ASSIGN_VECTOR_DOUBLE, init_assign_vector_double, (vec: *mut Arr<f64>, data: *const f64, size: usize) -> ());
import_symbol!(assign_vector_string, ASSIGN_VECTOR_STRING, init_assign_vector_string, (vec: *mut Arr<Str>, data: *const Str, size: usize) -> ());
import_symbol!(assign_vector_variant, ASSIGN_VECTOR_VARIANT, init_assign_vector_variant, (vec: *mut Arr<Var>, data: *const Var, size: usize) -> ());
import_symbol!(assign_vector_vector2, ASSIGN_VECTOR_VECTOR2, init_assign_vector_vector2, (vec: *mut Arr<Vec2>, data: *const Vec2, size: usize) -> ());
import_symbol!(assign_vector_vector3, ASSIGN_VECTOR_VECTOR3, init_assign_vector_vector3, (vec: *mut Arr<Vec3>, data: *const Vec3, size: usize) -> ());
import_symbol!(assign_vector_vector4, ASSIGN_VECTOR_VECTOR4, init_assign_vector_vector4, (vec: *mut Arr<Vec4>, data: *const Vec4, size: usize) -> ());
import_symbol!(assign_vector_matrix4x4, ASSIGN_VECTOR_MATRIX4X4, init_assign_vector_matrix4x4, (vec: *mut Arr<Mat4x4>, data: *const Mat4x4, size: usize) -> ());

/// FFI-compatible vector type matching the memory layout of the C++ plg::vector<T>
///
/// # Memory Layout
///
/// This struct uses `#[repr(C)]` to match the C++ plg::vector layout (begin/end/capacity pointers).
/// This layout is guaranteed by the LLVM-based plg library and must not be changed.
///
/// # Safety
///
/// This type is only safe to use through the ArrOps trait methods, which call into
/// the C++ library functions. Direct field access or construction is unsafe and undefined behavior.
#[repr(C)]
pub struct Arr<T: ArrOps> {
    begin: usize,
    end: usize,
    capacity: usize,
    _phantom: PhantomData<T>,
}
const _: () = assert!(size_of::<Arr<usize>>() == 3 * size_of::<*const ()>());

impl<T: ArrOps + std::fmt::Debug> std::fmt::Debug for Arr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.iter())
            .finish()
    }
}

impl<T: ArrOps + std::fmt::Display> std::fmt::Display for Arr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for item in self.iter() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
            first = false;
        }
        write!(f, "]")
    }
}

// ============================================
// Trait definitions
// ============================================

/// Unified trait for all Arr operations
///
/// This trait provides type-specific FFI bindings to the C++ plg::vector functions.
/// Each type that can be stored in a Arr must implement this trait to provide
/// the appropriate C++ function bindings.
///
/// # Safety
///
/// Implementations must ensure that:
/// - The C++ functions are properly initialized via dynlink
/// - The functions correctly handle the memory layout of type T
/// - The destroy function properly frees C++ allocated memory
pub trait ArrOps: Sized {
    fn new(data: &[Self]) -> Arr<Self>;
    fn destroy(vec: &mut Arr<Self>);
    fn len(vec: &Arr<Self>) -> usize;
    fn data(vec: &Arr<Self>) -> *const Self;
    fn data_mut(vec: &mut Arr<Self>) -> *mut Self;
    fn set(vec: &mut Arr<Self>, data: &[Self]);

    /// Get data as slice (zero-copy view)
    ///
    /// # Safety
    ///
    /// This is safe because:
    /// - We check if len is 0 and return a valid empty slice
    /// - The data pointer from C++ is guaranteed valid for `len` elements
    /// - The lifetime is tied to the Arr borrow, preventing use-after-free
    fn as_slice(vec: &Arr<Self>) -> &[Self] {
        unsafe {
            let len = Self::len(vec);
            if len == 0 {
                // FIXED: Use the actual data pointer even when empty, as C++ may have
                // a valid pointer. Only create an empty slice.
                return &[];
            }
            let data = Self::data(vec);
            // SAFETY: C++ guarantees the data pointer is valid for `len` elements
            std::slice::from_raw_parts(data, len)
        }
    }

    /// Get data as mut slice (zero-copy view)
    ///
    /// # Safety
    ///
    /// This is safe because:
    /// - We check if len is 0 and return a valid empty slice
    /// - The data pointer from C++ is guaranteed valid for `len` elements
    /// - The mutable lifetime is tied to the Arr mutable borrow, ensuring exclusive access
    fn as_mut_slice(vec: &mut Arr<Self>) -> &mut [Self] {
        unsafe {
            let len = Self::len(vec);
            if len == 0 {
                // FIXED: Use the actual data pointer even when empty
                return &mut [];
            }
            let data = Self::data_mut(vec);
            // SAFETY: C++ guarantees the data pointer is valid for `len` elements,
            // and we have exclusive mutable access
            std::slice::from_raw_parts_mut(data, len)
        }
    }

    /// Get data as Vec (copy)
    #[must_use = "this creates a new Vec, use as_slice() if you just need to read the data"]
    fn to_vec(vec: &Arr<Self>) -> Vec<Self> where Self: Clone {
        Self::as_slice(vec).to_vec()
    }

    /// Used for const iteration
    fn iter(vec: &Arr<Self>) -> std::slice::Iter<'_, Self> {
        Self::as_slice(vec).iter()
    }

    /// Used for mut iteration
    fn iter_mut(vec: &mut Arr<Self>) -> std::slice::IterMut<'_, Self> {
        Self::as_mut_slice(vec).iter_mut()
    }
}

/// Marker trait for C-compatible enums with a specific integer representation
///
/// This trait enables automatic ArrOps implementation for enums by treating them
/// as their underlying integer type during FFI calls.
///
/// # Safety
///
/// Implementors must guarantee that:
/// - The type has `#[repr(IntType)]` where IntType is the associated `ReprInt` type
/// - The type has the exact same memory layout and alignment as `ReprInt`
/// - All bit patterns valid for `ReprInt` represent valid enum values, or the code
///   handles potentially invalid values safely without UB
///
/// # Example
///
/// ```rust
/// #[repr(i32)]
/// #[derive(Copy, Clone)]
/// enum MyEnum {
///     A = 0,
///     B = 1,
///     C = 2,
/// }
///
/// // Use the helper macro to implement the trait safely
/// vector_enum_traits!(MyEnum, i32);
/// ```
pub unsafe trait CEnumRepr: Sized + Copy {
    /// The underlying integer type (i8, i16, i32, i64, u8, u16, u32, u64)
    type ReprInt: ArrOps + Copy;
}

/// Automatic implementation of ArrOps for enums that implement CEnumRepr
///
/// This implementation transmutes between the enum type and its underlying integer type
/// to reuse the integer type's ArrOps implementation.
///
/// # Safety
///
/// This is safe because:
/// - CEnumRepr is an unsafe trait requiring the enum and ReprInt to have identical layout
/// - We verify size and alignment match at runtime via debug_assert
/// - Arr<E> and Arr<E::ReprInt> have identical memory layout due to same field types
/// - All transmutes preserve the underlying bit representation
impl<E: CEnumRepr> ArrOps for E {
    fn new(data: &[Self]) -> Arr<Self> {
        debug_assert!(size_of::<E>() == size_of::<E::ReprInt>());
        debug_assert!(align_of::<E>() == align_of::<E::ReprInt>());

        unsafe {
            // SAFETY: CEnumRepr guarantees E and ReprInt have identical layout.
            // We cast the enum slice to integer slice for FFI call.
            let int_data = std::slice::from_raw_parts(
                data.as_ptr() as *const E::ReprInt,
                data.len()
            );
            let int_vec = E::ReprInt::new(int_data);
            // SAFETY: Arr<E> and Arr<E::ReprInt> have identical memory layout
            // (both are 3 pointers), so transmute is safe
            std::mem::transmute(int_vec)
        }
    }

    fn destroy(vec: &mut Arr<Self>) {
        unsafe {
            // SAFETY: Transmute to the integer vector type for C++ cleanup
            let int_vec: &mut Arr<E::ReprInt> = std::mem::transmute(vec);
            E::ReprInt::destroy(int_vec);
        }
    }

    fn len(vec: &Arr<Self>) -> usize {
        unsafe {
            // SAFETY: Same memory layout allows transmute for reading size
            let int_vec: &Arr<E::ReprInt> = std::mem::transmute(vec);
            E::ReprInt::len(int_vec)
        }
    }

    fn data(vec: &Arr<Self>) -> *const Self {
        unsafe {
            // SAFETY: Transmute to get the data pointer, then cast back to enum pointer
            let int_vec: &Arr<E::ReprInt> = std::mem::transmute(vec);
            E::ReprInt::data(int_vec) as *const Self
        }
    }

    fn data_mut(vec: &mut Arr<Self>) -> *mut Self {
        unsafe {
            // SAFETY: Transmute to get the mutable data pointer, then cast back to enum pointer
            let int_vec: &mut Arr<E::ReprInt> = std::mem::transmute(vec);
            E::ReprInt::data_mut(int_vec) as *mut Self
        }
    }

    fn set(vec: &mut Arr<Self>, data: &[Self]) {
        unsafe {
            // SAFETY: Cast enum slice to integer slice for FFI call
            let int_vec: &mut Arr<E::ReprInt> = std::mem::transmute(vec);
            let int_data = std::slice::from_raw_parts(
                data.as_ptr() as *const E::ReprInt,
                data.len()
            );
            E::ReprInt::set(int_vec, int_data);
        }
    }
}

#[macro_export]
macro_rules! vector_ops_traits {
    (
        $t:ty,
        $construct:path,
        $destroy:path,
        $len:path,
        $data:path,
        $assign:path
    ) => {
        impl ArrOps for $t {
            fn new(data: &[Self]) -> Arr<$t> {
                $construct(data.as_ptr(), data.len())
            }

            fn destroy(vec: &mut Arr<$t>) {
                $destroy(vec)
            }

            fn len(vec: &Arr<$t>) -> usize {
                $len(vec)
            }

            fn data(vec: &Arr<$t>) -> *const Self {
                $data(vec)
            }

            fn data_mut(vec: &mut Arr<$t>) -> *mut Self {
                $data(vec)
            }

            fn set(vec: &mut Arr<$t>, data: &[Self]) {
                $assign(vec, data.as_ptr(), data.len())
            }
        }
    };
}

vector_ops_traits!(
    bool,
    construct_vector_bool,
    destroy_vector_bool,
    get_vector_size_bool,
    get_vector_data_bool,
    assign_vector_bool
);

vector_ops_traits!(
    i8,
    construct_vector_int8,
    destroy_vector_int8,
    get_vector_size_int8,
    get_vector_data_int8,
    assign_vector_int8
);

vector_ops_traits!(
    i16,
    construct_vector_int16,
    destroy_vector_int16,
    get_vector_size_int16,
    get_vector_data_int16,
    assign_vector_int16
);

vector_ops_traits!(
    i32,
    construct_vector_int32,
    destroy_vector_int32,
    get_vector_size_int32,
    get_vector_data_int32,
    assign_vector_int32
);

vector_ops_traits!(
    i64,
    construct_vector_int64,
    destroy_vector_int64,
    get_vector_size_int64,
    get_vector_data_int64,
    assign_vector_int64
);

vector_ops_traits!(
    u8,
    construct_vector_uint8,
    destroy_vector_uint8,
    get_vector_size_uint8,
    get_vector_data_uint8,
    assign_vector_uint8
);

vector_ops_traits!(
    u16,
    construct_vector_uint16,
    destroy_vector_uint16,
    get_vector_size_uint16,
    get_vector_data_uint16,
    assign_vector_uint16
);

vector_ops_traits!(
    u32,
    construct_vector_uint32,
    destroy_vector_uint32,
    get_vector_size_uint32,
    get_vector_data_uint32,
    assign_vector_uint32
);

vector_ops_traits!(
    u64,
    construct_vector_uint64,
    destroy_vector_uint64,
    get_vector_size_uint64,
    get_vector_data_uint64,
    assign_vector_uint64
);

vector_ops_traits!(
    usize,
    construct_vector_pointer,
    destroy_vector_pointer,
    get_vector_size_pointer,
    get_vector_data_pointer,
    assign_vector_pointer
);

vector_ops_traits!(
    f32,
    construct_vector_float,
    destroy_vector_float,
    get_vector_size_float,
    get_vector_data_float,
    assign_vector_float
);

vector_ops_traits!(
    f64,
    construct_vector_double,
    destroy_vector_double,
    get_vector_size_double,
    get_vector_data_double,
    assign_vector_double
);

vector_ops_traits!(
    Vec2,
    construct_vector_vector2,
    destroy_vector_vector2,
    get_vector_size_vector2,
    get_vector_data_vector2,
    assign_vector_vector2
);

vector_ops_traits!(
    Vec3,
    construct_vector_vector3,
    destroy_vector_vector3,
    get_vector_size_vector3,
    get_vector_data_vector3,
    assign_vector_vector3
);

vector_ops_traits!(
    Vec4,
    construct_vector_vector4,
    destroy_vector_vector4,
    get_vector_size_vector4,
    get_vector_data_vector4,
    assign_vector_vector4
);

vector_ops_traits!(
    Mat4x4,
    construct_vector_matrix4x4,
    destroy_vector_matrix4x4,
    get_vector_size_matrix4x4,
    get_vector_data_matrix4x4,
    assign_vector_matrix4x4
);

vector_ops_traits!(
    Str,
    construct_vector_string,
    destroy_vector_string,
    get_vector_size_string,
    get_vector_data_string,
    assign_vector_string
);

vector_ops_traits!(
    Var,
    construct_vector_variant,
    destroy_vector_variant,
    get_vector_size_variant,
    get_vector_data_variant,
    assign_vector_variant
);

// ============================================
// Generic methods on Arr
// ============================================

impl<T: ArrOps> Arr<T> {
    /// Construct a new empty Arr
    pub fn new() -> Self {
        T::new(&[])
    }

    /// Construct a new Arr from a slice
    ///
    /// # Panics
    ///
    /// May panic if the C++ allocation fails. The panic is safe - no resources will leak.
    pub fn from_slice(data: &[T]) -> Self {
        T::new(data)
    }

    /// Get the length of the vector
    #[must_use]
    pub fn len(&self) -> usize {
        T::len(self)
    }

    /// Check if the vector is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        T::len(self) == 0
    }

    /// Get data as a slice (zero-copy view)
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        T::as_slice(self)
    }

    /// Get data as a mutable slice (zero-copy view)
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        T::as_mut_slice(self)
    }

    /// Get data as a Rust-owned Vec (allocates and copies)
    #[must_use = "this allocates and copies data into a new Vec"]
    pub fn to_vec(&self) -> Vec<T> where T: Clone {
        T::to_vec(self)
    }

    /// Get data by index
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        T::as_slice(self).get(index)
    }

    /// Set new data to the vector, replacing previous contents
    ///
    /// # Safety
    ///
    /// The C++ implementation must not throw exceptions. If allocation fails,
    /// the process will abort.
    pub fn set(&mut self, data: &[T]) {
        T::set(self, data);
    }

    /// Destroy the vector (manual cleanup)
    ///
    /// This is typically not needed as Drop handles cleanup automatically.
    /// Only use this if you need explicit control over when cleanup occurs.
    pub fn destroy(&mut self) {
        T::destroy(self);
    }

    /// Get an iterator over the vector elements
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        T::iter(self)
    }

    /// Get a mutable iterator over the vector elements
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        T::iter_mut(self)
    }
}

impl<T: ArrOps> Index<usize> for Arr<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T: ArrOps> IndexMut<usize> for Arr<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut T::as_mut_slice(self)[idx]
    }
}

impl<T: ArrOps> Drop for Arr<T>  {
    fn drop(&mut self) {
        T::destroy(self);
    }
}

// ============================================
// Convenient String/Any implementations
// ============================================

impl Arr<Str> {
    #[must_use = "this allocates and converts to Vec<String>"]
    pub fn to_string(&self) -> Vec<String> {
        self.as_slice()
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
}

impl From<&[String]> for Arr<Str> {
    fn from(data: &[String]) -> Self {
        let views: Vec<Str> = data.iter()
            .map(|s| Str::from(s))
            .collect();
        Arr::from_slice(&views)
    }
}

impl From<&Vec<String>> for Arr<Str> {
    fn from(data: &Vec<String>) -> Self {
        Arr::from(data.as_slice())
    }
}

impl From<Vec<String>> for Arr<Str> {
    fn from(data: Vec<String>) -> Self {
        Arr::from(data.as_slice())
    }
}

impl Arr<Var> {
    #[must_use = "this allocates and converts to Vec<Any>"]
    pub fn to_any(&self) -> Vec<Any> {
        self.as_slice()
            .iter()
            .map(|s| s.get())
            .collect()
    }
}

impl From<&[Any]> for Arr<Var> {
    fn from(data: &[Any]) -> Self {
        let views: Vec<Var> = data.iter()
            .map(|s| Var::new(s))
            .collect();
        Arr::from_slice(&views)
    }
}

impl From<&Vec<Any>> for Arr<Var> {
    fn from(data: &Vec<Any>) -> Self {
        Arr::from(data.as_slice())
    }
}

impl From<Vec<Any>> for Arr<Var> {
    fn from(data: Vec<Any>) -> Self {
        Arr::from(data.as_slice())
    }
}

// ============================================
// Convenient From implementations
// ============================================

macro_rules! vector_from_vec {
    ($t:ty) => {
        impl From<&Vec<$t>> for Arr<$t> {
            fn from(value: &Vec<$t>) -> Self {
                Arr::from_slice(value.as_slice())
            }
        }
        impl From<Vec<$t>> for Arr<$t> {
            fn from(value: Vec<$t>) -> Self {
                Arr::from_slice(value.as_slice())
            }
        }
    };
}

vector_from_vec!(bool);
vector_from_vec!(i8);
vector_from_vec!(i16);
vector_from_vec!(i32);
vector_from_vec!(i64);
vector_from_vec!(u8);
vector_from_vec!(u16);
vector_from_vec!(u32);
vector_from_vec!(u64);
vector_from_vec!(usize);
vector_from_vec!(f32);
vector_from_vec!(f64);
vector_from_vec!(Str);
vector_from_vec!(Var);
vector_from_vec!(Vec2);
vector_from_vec!(Vec3);
vector_from_vec!(Vec4);
vector_from_vec!(Mat4x4);

macro_rules! vector_from_slice {
    ($t:ty) => {
        impl From<&[$t]> for Arr<$t> {
            fn from(value: &[$t]) -> Self {
                Arr::from_slice(value)
            }
        }
    };
}

vector_from_slice!(bool);
vector_from_slice!(i8);
vector_from_slice!(i16);
vector_from_slice!(i32);
vector_from_slice!(i64);
vector_from_slice!(u8);
vector_from_slice!(u16);
vector_from_slice!(u32);
vector_from_slice!(u64);
vector_from_slice!(usize);
vector_from_slice!(f32);
vector_from_slice!(f64);
vector_from_slice!(Str);
vector_from_slice!(Var);
vector_from_slice!(Vec2);
vector_from_slice!(Vec3);
vector_from_slice!(Vec4);
vector_from_slice!(Mat4x4);

// ============================================
// Helper macro for C-compatible enums
// ============================================

/// Helper macro to safely implement CEnumRepr for C-compatible enums
///
/// This macro:
/// 1. Implements the unsafe CEnumRepr trait
/// 2. Adds compile-time assertions to verify size and alignment match
///
/// # Example
///
/// ```rust
/// #[repr(i32)]
/// #[derive(Copy, Clone, Debug)]
/// enum MyStatus {
///     Idle = 0,
///     Running = 1,
///     Stopped = 2,
/// }
///
/// vector_enum_traits!(MyStatus, i32);
///
/// // Now you can use Arr<MyStatus>
/// let statuses = Arr::from_slice(&[MyStatus::Idle, MyStatus::Running]);
/// ```
#[macro_export]
macro_rules! vector_enum_traits {
    ($enum_ty:ty, $repr:ty) => {
        unsafe impl $crate::CEnumRepr for $enum_ty {
            type ReprInt = $repr;
        }

        // Compile-time checks to ensure safety invariants
        const _: () = assert!(
            std::mem::size_of::<$enum_ty>() == std::mem::size_of::<$repr>(),
            "Enum size must match its repr type size"
        );
        const _: () = assert!(
            std::mem::align_of::<$enum_ty>() == std::mem::align_of::<$repr>(),
            "Enum alignment must match its repr type alignment"
        );
    };
}