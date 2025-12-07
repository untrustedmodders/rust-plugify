use std::sync::OnceLock;
use crate::dynlink_impl;

dynlink_impl!(construct_string, CONSTRUCT_STRING, init_construct_string, (data:*const u8, size:usize) -> PlgString);
dynlink_impl!(destroy_string, DESTROY_STRING, init_destroy_string, (str:*mut PlgString) -> ());
dynlink_impl!(get_string_data, GET_STRING_DATA, init_get_string_data, (str:*const PlgString) -> *const u8);
dynlink_impl!(get_string_length, GET_STRING_LENGTH, init_get_string_length, (str:*const PlgString) -> usize);
dynlink_impl!(assign_string, ASSIGN_STRING, init_assign_string, (str:*mut PlgString, data:*const u8, size:usize) -> ());

#[repr(C)]
#[derive(Debug)]
pub struct PlgString {
    pub data: usize,
    pub size: usize,
    pub cap: usize,
}
const _: () = assert!(size_of::<PlgString>() == 3 * size_of::<*const ()>());

impl PlgString {
    pub fn new(s: &str) -> Self {
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

    pub fn to_string(&self) -> String {
        unsafe {
            let len = get_string_length(self);
            if len == 0 { return String::new(); }
            let ptr = get_string_data(self);
            let slice = std::slice::from_raw_parts(ptr, len);
            String::from_utf8_lossy(slice).into_owned() // copies into Rust String
        }
    }

    pub fn len(&self) -> usize {
        get_string_length(self)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn assign(&mut self, s: &str) {
        assign_string(self, s.as_ptr(), s.len());
    }

    pub fn destroy(&mut self) {
        destroy_string(self);
    }
}

impl Drop for PlgString {
    fn drop(&mut self) { self.destroy(); }
}

impl From<&str> for PlgString {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for PlgString {
    fn from(s: String) -> Self {
        Self::new(&s)
    }
}