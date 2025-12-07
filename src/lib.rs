pub mod string;
pub use string::PlgString;

pub mod vector;
pub use vector::{PlgVector, CEnumRepr};

pub mod variant;
pub use variant::{PlgVariant, PlgAny};

pub mod mat4x4;
pub use mat4x4::Matrix4x4;

pub mod vec2;
pub use vec2::Vector2;

pub mod vec3;
pub use vec3::Vector3;

pub mod vec4;
pub use vec4::Vector4;

pub mod plugin;
pub use plugin::{on_plugin_start, on_plugin_end, on_plugin_update};

#[macro_export]
macro_rules! dynlink_impl {
    ($name:ident, $func_name:ident, $init_name:ident, ($($arg_name:ident : $arg_ty:ty),*) -> $ret:ty) => {
        static $func_name: OnceLock<unsafe extern "C" fn($($arg_ty),*) -> $ret> = OnceLock::new();

        #[inline]
        pub fn $init_name(addr: usize) {
            unsafe {
                let ptr = std::mem::transmute::<usize, unsafe extern "C" fn($($arg_ty),*) -> $ret>(addr);
                $func_name.set(ptr).expect("Function can only be set once");
            }
        }

        #[inline]
        pub fn $name($($arg_name: $arg_ty),*) -> $ret {
            unsafe {
                let func = $func_name.get().expect("Function not initialized");
                func($($arg_name),*)
            }
        }
    };
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
