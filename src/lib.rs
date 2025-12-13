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

/// Export plugify runtime symbols to prevent linker stripping.
///
/// Creates `#[used]` static references to plugify runtime functions, ensuring they
/// remain in the plugin's export table for dynamic loading via `dlsym`/`GetProcAddress`.
///
/// This macro is automatically called by `register_plugin!`, so you typically don't
/// need to call it manually. Only call directly if you're not using `register_plugin!`.
///
/// # Example
///
/// ```
/// use plugify::*;
///
/// export_symbols!();
///
/// register_plugin! {
///     start: my_start,
/// }
/// # fn my_start() {}
/// ```
///
/// # Safety
///
/// Safe to use - generates only extern declarations and static references
#[macro_export]
macro_rules! export_symbols {
    () => {
        unsafe extern "C" {
            fn plugify_init() -> *const ();
            fn plugify_plugin_start() -> *const ();
            fn plugify_plugin_update() -> *const ();
            fn plugify_plugin_end() -> *const ();
            fn plugify_plugin_context() -> *const ();
        }

        #[used]
        static PLUGIFY_INIT_REF: unsafe extern "C" fn() -> *const () = plugify_init;

        #[used]
        static PLUGIFY_PLUGIN_START_REF: unsafe extern "C" fn() -> *const () = plugify_plugin_start;

        #[used]
        static PLUGIFY_PLUGIN_UPDATE_REF: unsafe extern "C" fn() -> *const () = plugify_plugin_update;

        #[used]
        static PLUGIFY_PLUGIN_END_REF: unsafe extern "C" fn() -> *const () = plugify_plugin_end;

        #[used]
        static PLUGIFY_PLUGIN_CONTEXT_REF: unsafe extern "C" fn() -> *const () = plugify_plugin_context;
    };
}

/// Create a type-safe wrapper for a dynamically loaded host function.
///
/// Generates static storage, an init function, and a safe wrapper for calling
/// function pointers provided by the host at runtime.
///
/// # Parameters
///
/// - `$name` - Safe wrapper function name
/// - `$func_name` - Static variable name for storage
/// - `$init_name` - Initialization function name
/// - Signature: `($arg: $type, ...) -> $return`
///
/// # Example
///
/// ```
/// # use plugify::*;
/// import_symbol!(
///     log_message,
///     LOG_MESSAGE_PTR,
///     init_log_message,
///     (message: *const u8, len: usize) -> ()
/// );
///
/// // In plugify_init:
/// fn setup(api: &[usize]) {
///     init_log_message(api[0]);
/// }
///
/// // Now callable:
/// fn my_function() {
///     let msg = b"Hello\0";
///     log_message(msg.as_ptr(), msg.len());
/// }
/// ```
///
/// # Safety
///
/// Uses `transmute` to convert `usize` to function pointer. Host must provide:
/// - Valid function pointer for plugin lifetime
/// - Matching signature with `extern "C"` ABI
///
/// # Panics
///
/// - Init function called twice
/// - Wrapper called before initialization
#[macro_export]
macro_rules! import_symbol {
    ($name:ident, $func_name:ident, $init_name:ident, ($($arg_name:ident : $arg_ty:ty),*) -> $ret:ty) => {
        static mut $func_name: Option<unsafe extern "C" fn($($arg_ty),*) -> $ret> = None;

        pub fn $init_name(addr: usize) {
            unsafe {
                let ptr = std::mem::transmute::<usize, unsafe extern "C" fn($($arg_ty),*) -> $ret>(addr);
                $func_name = Some(ptr);
            }
        }

        pub fn $name($($arg_name: $arg_ty),*) -> $ret {
            unsafe {
                let func = $func_name.expect("Function not initialized");
                func($($arg_name),*)
            }
        }
    };
}

/// Register plugin lifecycle callbacks with the host.
///
/// Generates an exported `plugify_main()` function that the host calls to register
/// your plugin's callbacks. Automatically calls `export_symbols!()`.
///
/// All callbacks are optional - register any combination you need.
///
/// # Callback Signatures
///
/// - `start: fn()` - Called when plugin starts
/// - `update: fn(f32)` - Called every frame with delta time
/// - `end: fn()` - Called when plugin shuts down
///
/// # Examples
///
/// ```
/// use plugify::*;
///
/// fn on_start() {
///     println!("Plugin started!");
/// }
///
/// fn on_update(dt: f32) {
///     println!("Frame time: {}", dt);
/// }
///
/// fn on_end() {
///     println!("Plugin ended!");
/// }
///
/// register_plugin! {
///     start: on_start,
///     update: on_update,
///     end: on_end,
/// }
/// ```
///
/// Register only some callbacks:
///
/// ```
/// # use plugify::*;
/// # fn init() {}
/// # fn cleanup() {}
/// register_plugin! {
///     start: init,
///     end: cleanup,
/// }
/// ```
///
/// # Panics
///
/// Panics if called more than once per plugin
#[macro_export]
macro_rules! register_plugin {
    (
        $(start: $start:expr,)?
        $(update: $update:expr,)?
        $(end: $end:expr$(,)?)?
    ) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn plugify_main() {
            $(
                $crate::on_plugin_start($start);
            )?
            $(
                $crate::on_plugin_update($update);
            )?
            $(
                $crate::on_plugin_end($end);
            )?
        }

        $crate::export_symbols!();
    };
}
