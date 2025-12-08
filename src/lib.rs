pub mod string;
pub mod vector;
pub mod variant;
pub mod mat4x4;
pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod plugin;

pub use string::*;
pub use vector::*;
pub use variant::*;
pub use mat4x4::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
pub use plugin::*;

#[macro_export]
macro_rules! dynlink_impl {
    ($name:ident, $func_name:ident, $init_name:ident, ($($arg_name:ident : $arg_ty:ty),*) -> $ret:ty) => {
        static $func_name: OnceLock<unsafe extern "C" fn($($arg_ty),*) -> $ret> = OnceLock::new();

        pub fn $init_name(addr: usize) {
            unsafe {
                let ptr = std::mem::transmute::<usize, unsafe extern "C" fn($($arg_ty),*) -> $ret>(addr);
                $func_name.set(ptr).expect("Function can only be set once");
            }
        }

        pub fn $name($($arg_name: $arg_ty),*) -> $ret {
            unsafe {
                let func = $func_name.get().expect("Function not initialized");
                func($($arg_name),*)
            }
        }
    };
}
