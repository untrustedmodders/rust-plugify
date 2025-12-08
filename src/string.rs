use std::sync::OnceLock;
use std::ops::Deref;
use crate::dynlink_impl;

dynlink_impl!(construct_string, CONSTRUCT_STRING, init_construct_string, (data:*const u8, size:usize) -> PlgString);
dynlink_impl!(destroy_string, DESTROY_STRING, init_destroy_string, (str:*mut PlgString) -> ());
dynlink_impl!(get_string_data, GET_STRING_DATA, init_get_string_data, (str:*const PlgString) -> *mut u8);
dynlink_impl!(get_string_length, GET_STRING_LENGTH, init_get_string_length, (str:*const PlgString) -> usize);
dynlink_impl!(assign_string, ASSIGN_STRING, init_assign_string, (str:*mut PlgString, data:*const u8, size:usize) -> ());

#[repr(C)]
pub struct PlgString {
    data: *const u8,
    size: usize,
    cap: usize,
}
const _: () = assert!(size_of::<PlgString>() == 3 * size_of::<*const ()>());

impl std::fmt::Debug for PlgString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PlgString")
            .field(&self.to_string())
            .finish()
    }
}

// Ownership Contract:
// - PlgString owns a foreign heap allocation.
// - Foreign side provides: construct_string, assign_string, destroy_string.
// - get_string_data/get_string_length return stable pointers until the next assignment.
// - UTF-8 is guaranteed by the foreign implementation.
// - Destroy must be invoked exactly once per allocation.
// - After destruction: data/size/cap = 0 to prevent accidental reuse.
// - No aliasing guarantees from the foreign side; caller must avoid holding
//   mutable and immutable references simultaneously.
impl PlgString {
    pub fn new() -> Self {
        construct_string("".as_ptr(), 0)
    }

    pub fn from_str(s: &str) -> Self {
        construct_string(s.as_ptr(), s .len())
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let len = get_string_length(self);
            if len == 0 { return ""; }
            let ptr = get_string_data(self);
            let slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(slice)
        }
    }

    pub fn as_mut_str(&mut self) -> &mut str {
        unsafe {
            let len = get_string_length(self);
            let ptr = get_string_data(self);
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            std::str::from_utf8_unchecked_mut(slice)
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let len = get_string_length(self);
            if len == 0 {
                return &[];
            }
            let ptr = get_string_data(self);
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        unsafe {
            let len = get_string_length(self);
            let ptr = get_string_data(self);
            std::slice::from_raw_parts_mut(ptr, len)
        }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let len = get_string_length(self);
            if len == 0 { return String::new(); }
            let ptr = get_string_data(self);
            let slice = std::slice::from_raw_parts(ptr, len);
            String::from_utf8_unchecked(slice.to_vec())
        }
    }

    pub fn len(&self) -> usize {
        get_string_length(self)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&mut self, s: &str) {
        assign_string(self, s.as_ptr(), s.len());
    }

    pub fn destroy(&mut self) {
        destroy_string(self);
    }
}

impl Drop for PlgString {
    fn drop(&mut self) { self.destroy(); }
}

impl Clone for PlgString {
    fn clone(&self) -> Self {
        PlgString::from_str(self.as_str())
    }
}

impl Default for PlgString {
    fn default() -> Self {
        PlgString::new()
    }
}

impl Deref for PlgString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl From<&str> for PlgString {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<String> for PlgString {
    fn from(s: String) -> Self {
        Self::from_str(s.as_str())
    }
}
impl From<&String> for PlgString {
    fn from(s: &String) -> Self {
        Self::from_str(s.as_str())
    }
}