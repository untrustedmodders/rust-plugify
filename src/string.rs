use crate::import_symbol;

import_symbol!(construct_string, CONSTRUCT_STRING, init_construct_string, (data:*const u8, size:usize) -> Str);
import_symbol!(destroy_string, DESTROY_STRING, init_destroy_string, (str:*mut Str) -> ());
import_symbol!(get_string_data, GET_STRING_DATA, init_get_string_data, (str:*const Str) -> *mut u8);
import_symbol!(get_string_length, GET_STRING_LENGTH, init_get_string_length, (str:*const Str) -> usize);
import_symbol!(assign_string, ASSIGN_STRING, init_assign_string, (str:*mut Str, data:*const u8, size:usize) -> ());

/// FFI-compatible string type matching the memory layout of the C++ plg::string
///
/// # Memory Layout
///
/// This struct uses `#[repr(C)]` to match the C++ plg::string layout (data/size/capacity pointers).
/// This layout is guaranteed by the LLVM-based plg library and must not be changed.
///
/// # Ownership Contract
///
/// - Str owns a foreign heap allocation managed by the C++ plg library
/// - The foreign side provides: `construct_string`, `assign_string`, `destroy_string`
/// - `get_string_data`/`get_string_length` return stable pointers until the next assignment
/// - UTF-8 encoding is guaranteed by the foreign implementation
/// - `destroy_string` must be invoked exactly once per allocation (handled by Drop)
/// - After destruction: data/size/cap are invalidated
/// - The foreign side guarantees valid UTF-8 data at all times
///
/// # Safety
///
/// This type is only safe to use through its public API, which provides safe abstractions
/// over the unsafe FFI calls. Direct field access is unsafe and undefined behavior.
///
/// # Warning
///
/// Do NOT use `as_mut_bytes()` to modify the byte content, as this could break the
/// UTF-8 invariant and cause undefined behavior. Use `set()` for modifications.
#[repr(C)]
pub struct Str {
    data: usize,
    size: usize,
    cap: usize,
}
const _: () = assert!(size_of::<Str>() == 3 * size_of::<*const ()>());

impl std::fmt::Debug for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Str")
            .field(&self.to_string())
            .finish()
    }
}

impl Str {
    /// Create a new empty Str
    ///
    /// # Panics
    ///
    /// May panic if the C++ allocation fails. The panic is safe - no resources will leak.
    pub fn new() -> Self {
        construct_string("".as_ptr(), 0)
    }

    /// Create a new Str from a string slice
    ///
    /// # Panics
    ///
    /// May panic if the C++ allocation fails. The panic is safe - no resources will leak.
    pub fn from_str(s: &str) -> Self {
        construct_string(s.as_ptr(), s.len())
    }

    /// Get the string as a borrowed &str (zero-copy view)
    ///
    /// # Safety
    ///
    /// This is safe because:
    /// - The C++ side guarantees valid UTF-8 data
    /// - The pointer is valid for the lifetime of the borrow
    /// - The length is accurate
    #[must_use]
    pub fn as_str(&self) -> &str {
        unsafe {
            let len = get_string_length(self);
            if len == 0 {
                return "";
            }
            let ptr = get_string_data(self);
            // SAFETY:
            // - C++ guarantees the pointer is valid for `len` bytes
            // - C++ guarantees valid UTF-8 encoding
            // - The lifetime is tied to self's borrow
            let slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(slice)
        }
    }

    /// Get the string as a mutable &mut str (zero-copy view)
    ///
    /// # Safety
    ///
    /// This is safe because:
    /// - The C++ side guarantees valid UTF-8 data
    /// - The pointer is valid for the lifetime of the mutable borrow
    /// - We have exclusive access via &mut self
    ///
    /// # Warning
    ///
    /// While you can modify the characters, you MUST maintain UTF-8 validity.
    /// If you break UTF-8, subsequent operations will cause undefined behavior.
    /// For safe modifications, use the `set()` method instead.
    #[must_use]
    pub fn as_mut_str(&mut self) -> &mut str {
        unsafe {
            let len = get_string_length(self);
            if len == 0 {
                // Return a valid empty mutable string slice
                return std::str::from_utf8_unchecked_mut(&mut []);
            }
            let ptr = get_string_data(self);
            // SAFETY:
            // - C++ guarantees the pointer is valid for `len` bytes
            // - C++ guarantees valid UTF-8 encoding
            // - We have exclusive mutable access
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            std::str::from_utf8_unchecked_mut(slice)
        }
    }

    /// Get the string as a byte slice (zero-copy view)
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let len = get_string_length(self);
            if len == 0 {
                return &[];
            }
            let ptr = get_string_data(self);
            // SAFETY: C++ guarantees the pointer is valid for `len` bytes
            std::slice::from_raw_parts(ptr, len)
        }
    }

    /// Get the string as a mutable byte slice (zero-copy view)
    ///
    /// # Safety
    ///
    /// ⚠️ **DANGER**: Modifying the bytes could break UTF-8 validity!
    ///
    /// This method is marked as `unsafe` because:
    /// - You can modify bytes in a way that creates invalid UTF-8
    /// - Subsequent calls to `as_str()` or `as_mut_str()` would then invoke UB
    ///
    /// # Safety Requirements
    ///
    /// If you modify the bytes, you MUST ensure they remain valid UTF-8.
    /// If you cannot guarantee this, use `set()` instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // SAFE: Converting to uppercase (ASCII range)
    /// let mut s = Str::from_str("hello");
    /// unsafe {
    ///     let bytes = s.as_mut_bytes();
    ///     bytes[0] = b'H';  // OK: still valid UTF-8
    /// }
    ///
    /// // UNSAFE: Breaking UTF-8
    /// let mut s = Str::from_str("hello");
    /// unsafe {
    ///     let bytes = s.as_mut_bytes();
    ///     bytes[0] = 0xFF;  // BAD: invalid UTF-8!
    ///     // Now calling s.as_str() would be UB!
    /// }
    /// ```
    #[must_use]
    pub unsafe fn as_mut_bytes(&mut self) -> &mut [u8] {
        unsafe {
            let len = get_string_length(self);
            if len == 0 {
                return &mut [];
            }
            let ptr = get_string_data(self);
            // SAFETY: Caller must ensure they maintain UTF-8 validity
            std::slice::from_raw_parts_mut(ptr, len)
        }
    }

    /// Convert to an owned Rust String (allocates and copies)
    #[must_use = "this allocates and copies data into a new String"]
    pub fn to_string(&self) -> String {
        unsafe {
            let len = get_string_length(self);
            if len == 0 {
                return String::new();
            }
            let ptr = get_string_data(self);
            // SAFETY: C++ guarantees valid UTF-8 data
            let slice = std::slice::from_raw_parts(ptr, len);
            String::from_utf8_unchecked(slice.to_vec())
        }
    }

    /// Get the length of the string in bytes
    #[must_use]
    pub fn len(&self) -> usize {
        get_string_length(self)
    }

    /// Check if the string is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Set the string to a new value, replacing the previous contents
    ///
    /// # Safety
    ///
    /// The C++ implementation must not throw exceptions. If allocation fails,
    /// the process will abort.
    pub fn set(&mut self, s: &str) {
        assign_string(self, s.as_ptr(), s.len());
    }

    /// Destroy the string (manual cleanup)
    ///
    /// This is typically not needed as Drop handles cleanup automatically.
    /// Only use this if you need explicit control over when cleanup occurs.
    pub fn destroy(&mut self) {
        destroy_string(self);
    }
}

impl Drop for Str {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Clone for Str {
    fn clone(&self) -> Self {
        Str::from_str(self.as_str())
    }
}

impl Default for Str {
    fn default() -> Self {
        Str::new()
    }
}

impl std::ops::Deref for Str {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl From<&str> for Str {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<String> for Str {
    fn from(s: String) -> Self {
        Self::from_str(s.as_str())
    }
}

impl From<&String> for Str {
    fn from(s: &String) -> Self {
        Self::from_str(s.as_str())
    }
}

// Additional helpful trait implementations

impl PartialEq for Str {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Str {}

impl PartialEq<str> for Str {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Str {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<String> for Str {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl std::fmt::Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::hash::Hash for Str {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl PartialOrd for Str {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Str {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}