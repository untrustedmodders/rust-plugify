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

// ============================================
// export_symbols! - Symbol export macro
// ============================================

/// Export all required plugify symbols from your plugin
///
/// This macro ensures that the plugify runtime functions are properly exported
/// and visible to the dynamic linker when your plugin is loaded. It creates
/// static references to prevent the linker from removing these symbols during
/// dead code elimination.
///
/// # What It Does
///
/// 1. Declares external C functions from the plugify runtime
/// 2. Creates `#[used]` static references to prevent symbol stripping
/// 3. Ensures symbols are exported in the plugin's symbol table
///
/// # Generated Code
///
/// The macro generates:
/// - External function declarations for plugify runtime functions
/// - Static function pointers marked with `#[used]` to prevent optimization
///
/// # Usage
///
/// Call this macro **once** at the root of your plugin crate:
///
/// ```
/// use plugify::*;
///
/// // Export plugify symbols so they're visible to dlsym
/// export_symbols!();
///
/// // Then register your callbacks
/// register_plugin! {
///     start: my_start,
/// }
/// # fn my_start() {}
/// ```
///
/// # Why This Is Needed
///
/// Dynamic loading requires symbols to be present in the plugin's export table.
/// Without this macro:
///
/// ```text
/// // In C++ host:
/// dlsym(handle, "plugify_init")  // Returns NULL - symbol not found!
/// ```
///
/// With this macro:
///
/// ```text
/// dlsym(handle, "plugify_init")  // Returns function pointer - success!
/// ```
///
/// # Technical Details
///
/// The `#[used]` attribute prevents LLVM from optimizing away the static references,
/// even though they appear unused in the Rust code. This ensures the symbols remain
/// in the binary's export table.
///
/// The host will use `dlsym()` (POSIX) or `GetProcAddress()` (Windows) to find:
/// - `plugify_init` - Initialize the plugin
/// - `plugify_plugin_start` - Start callback
/// - `plugify_plugin_update` - Update callback
/// - `plugify_plugin_end` - End callback
/// - `plugify_plugin_context` - Get plugin context
///
/// # Platform Support
///
/// This macro works on all platforms that support dynamic linking:
/// - Linux (ELF format)
/// - macOS (Mach-O format)
/// - Windows (PE format)
/// - BSD variants
///
/// # Example: Complete Plugin
///
/// ```
/// use plugify::*;
///
/// // Step 1: Export symbols
/// export_symbols!();
///
/// // Step 2: Define callbacks
/// fn on_start() {
///     println!("Plugin started!");
///
///     // Access plugin info
///     let plugin = PLUGIN.get().unwrap();
///     println!("Name: {}", plugin.name);
///     println!("Version: {}", plugin.version);
///
///     // Access directories
///     let data_dir = DATA_DIR.get().unwrap();
///     println!("Data directory: {}", data_dir);
/// }
///
/// fn on_update(dt: f32) {
///     // Update logic here
/// }
///
/// fn on_end() {
///     println!("Plugin shutting down...");
/// }
///
/// // Step 3: Register callbacks
/// register_plugin! {
///     start: on_start,
///     update: on_update,
///     end: on_end,
/// }
/// ```
///
/// # Troubleshooting
///
/// ## Symbol Not Found
///
/// If you get "symbol not found" errors:
///
/// 1. Make sure you called `export_symbols!()`
/// 2. Check that you're compiling as a `cdylib`:
///
/// ```toml
/// [lib]
/// crate-type = ["cdylib"]
/// ```
///
/// 3. Verify symbols are exported:
///
/// ```bash
/// # Linux/macOS
/// nm -D libmyplugin.so | grep plugify
///
/// # Windows
/// dumpbin /EXPORTS myplugin.dll | findstr plugify
/// ```
///
/// ## Multiple Definition Errors
///
/// If you call this macro more than once in your crate, you'll get linker errors
/// for duplicate symbols. Call it **exactly once** at the crate root.
///
/// # Safety
///
/// This macro uses `unsafe extern "C"` declarations but is safe to use because:
/// - The functions are provided by the plugify runtime
/// - They're only called by the host through proper FFI
/// - The static references don't execute any code
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

// ============================================
// import_symbol! - Dynamic function import macro
// ============================================

/// Create a dynamically loaded function wrapper with type-safe interface
///
/// This macro generates code to safely call functions that are loaded at runtime
/// from the host application via function pointers. It's used internally by the
/// plugify runtime to wrap host-provided API functions.
///
/// # Generated Code
///
/// For each function, the macro generates:
///
/// 1. **Static storage**: A `OnceLock` to store the function pointer thread-safely
/// 2. **Init function**: A function to set the function pointer (called during `plugify_init`)
/// 3. **Wrapper function**: A safe wrapper that calls the underlying function pointer
///
/// # Parameters
///
/// - `$name` - The safe wrapper function name (what you call)
/// - `$func_name` - The static variable name (internal storage)
/// - `$init_name` - The initialization function name (called by `plugify_init`)
/// - Function signature: `($arg_name: $arg_ty, ...) -> $ret`
///
/// # Example: Basic Usage
///
/// ```
/// use std::sync::OnceLock;
/// # use plugify::*;
///
/// // Create a dynamic import for a host function
/// import_symbol!(
///     log_message,              // Safe wrapper name
///     LOG_MESSAGE_PTR,         // Storage variable name
///     init_log_message,        // Init function name
///     (message: *const u8, len: usize) -> ()
/// );
///
/// // Later, during plugify_init:
/// fn setup(api: &[usize]) {
///     init_log_message(api[0]);  // Set the function pointer
/// }
///
/// // Now you can call it safely:
/// fn my_function() {
///     let msg = b"Hello, World!\0";
///     log_message(msg.as_ptr(), msg.len());
/// }
/// ```
///
/// # Generated Code Example
///
/// The above macro call generates:
///
/// ```ignore
/// static LOG_MESSAGE_PTR: OnceLock<unsafe extern "C" fn(*const u8, usize) -> ()>
///     = OnceLock::new();
///
/// pub fn init_log_message(addr: usize) {
///     unsafe {
///         let ptr = std::mem::transmute::<
///             usize,
///             unsafe extern "C" fn(*const u8, usize) -> ()
///         >(addr);
///         LOG_MESSAGE_PTR.set(ptr).expect("Function can only be set once");
///     }
/// }
///
/// pub fn log_message(message: *const u8, len: usize) -> () {
///     unsafe {
///         let func = LOG_MESSAGE_PTR.get().expect("Function not initialized");
///         func(message, len)
///     }
/// }
/// ```
///
/// # Complete Example: Host API Import
///
/// ```
/// use std::sync::OnceLock;
/// # use plugify::*;
///
/// // Import multiple host functions
/// import_symbol!(
///     get_game_time,
///     GET_GAME_TIME_PTR,
///     init_get_game_time,
///     () -> f64
/// );
///
/// import_symbol!(
///     spawn_entity,
///     SPAWN_ENTITY_PTR,
///     init_spawn_entity,
///     (x: f32, y: f32, z: f32) -> usize
/// );
///
/// import_symbol!(
///     destroy_entity,
///     DESTROY_ENTITY_PTR,
///     init_destroy_entity,
///     (id: usize) -> bool
/// );
///
/// // In plugify_init, set all the function pointers
/// fn initialize_api(api: &[usize]) {
///     init_get_game_time(api[0]);
///     init_spawn_entity(api[1]);
///     init_destroy_entity(api[2]);
/// }
///
/// // Now you can use them in your plugin
/// fn on_start() {
///     let time = get_game_time();
///     println!("Game started at time: {}", time);
///
///     let entity_id = spawn_entity(0.0, 0.0, 0.0);
///     println!("Spawned entity: {}", entity_id);
/// }
/// ```
///
/// # Internal Usage in Plugify
///
/// The plugify runtime uses this macro to import host-provided functions:
///
/// ```ignore
/// import_symbol!(
///     get_base_dir,
///     GET_BASE_DIR,
///     init_get_base_dir,
///     () -> PlgString
/// );
///
/// import_symbol!(
///     get_plugin_name,
///     GET_PLUGIN_NAME,
///     init_get_plugin_name,
///     (handle: usize) -> PlgString
/// );
/// ```
///
/// # Safety Considerations
///
/// ## Unsafe Operations
///
/// This macro involves several unsafe operations:
///
/// 1. **`transmute`**: Converts raw `usize` to function pointer
///    - Host must provide valid function pointer
///    - ABI must match (extern "C")
///
/// 2. **Function pointer call**: Dereferencing and calling function pointer
///    - Function must be valid for the lifetime of the plugin
///    - Arguments must match the expected types exactly
///
/// ## Requirements for Safety
///
/// To use this safely, the host **must**:
///
/// - Provide valid function pointers that remain valid for plugin lifetime
/// - Match the exact signature (including calling convention)
/// - Handle all possible input values correctly
///
/// ## Panics
///
/// The generated functions will panic if:
///
/// - Init function is called twice (OnceLock is already set)
/// - Wrapper function is called before initialization
///
/// ```should_panic
/// # use std::sync::OnceLock;
/// # use plugify::*;
/// # import_symbol!(my_func, MY_FUNC_PTR, init_my_func, () -> ());
/// // Calling before initialization - PANIC!
/// my_func();  // Error: "Function not initialized"
/// ```
///
/// # Type Safety
///
/// The macro provides type safety through Rust's type system:
///
/// ```compile_fail
/// # use std::sync::OnceLock;
/// # use plugify::*;
/// import_symbol!(add, ADD_PTR, init_add, (a: i32, b: i32) -> i32);
/// init_add(0x12345678);
///
/// // Type-checked at compile time
/// let result = add(1, 2);        // OK
/// let result = add(1.0, 2.0);    // Error: expected i32, found f64
/// let result = add("hi", "bye"); // Error: expected i32, found &str
/// ```
///
/// # Advanced: Multiple Signatures
///
/// For functions with complex signatures:
///
/// ```
/// use std::sync::OnceLock;
/// # use plugify::*;
///
/// // Function that returns a Result-like value
/// import_symbol!(
///     try_load_texture,
///     TRY_LOAD_TEXTURE_PTR,
///     init_try_load_texture,
///     (path: *const u8, path_len: usize, out_id: *mut usize) -> bool
/// );
///
/// fn load_texture(path: &str) -> Option<usize> {
///     let mut texture_id = 0;
///     let success = try_load_texture(
///         path.as_ptr(),
///         path.len(),
///         &mut texture_id as *mut usize
///     );
///
///     if success {
///         Some(texture_id)
///     } else {
///         None
///     }
/// }
/// ```
///
/// # Comparison with Direct FFI
///
/// Traditional FFI:
/// ```ignore
/// extern "C" {
///     fn host_function(x: i32) -> i32;  // Static, compile-time
/// }
/// ```
///
/// Dynamic import:
/// ```ignore
/// import_symbol!(host_function, HOST_FN, init_host_fn, (x: i32) -> i32);
/// // Loaded at runtime, flexible
/// ```
///
/// # Thread Safety
///
/// - The generated code is thread-safe
/// - `OnceLock` ensures atomic initialization
/// - Multiple threads can call the wrapper simultaneously after initialization
/// - Initialization must happen before any threads call the wrapper
///
/// # Best Practices
///
/// 1. **Naming convention**: Use SCREAMING_SNAKE_CASE for storage variables
/// 2. **Group related imports**: Keep API functions together
/// 3. **Document host requirements**: Note what the host must provide
/// 4. **Error handling**: Wrap unsafe calls in safe interfaces
///
/// ```
/// # use std::sync::OnceLock;
/// # use plugify::*;
/// // Good: Documented and grouped
/// /// Host-provided logging function
/// /// Host must ensure this remains valid for plugin lifetime
/// import_symbol!(
///     host_log,
///     HOST_LOG_PTR,
///     init_host_log,
///     (level: i32, msg: *const u8, len: usize) -> ()
/// );
///
/// /// Safe wrapper around host_log
/// pub fn log(level: LogLevel, message: &str) {
///     host_log(level as i32, message.as_ptr(), message.len());
/// }
///
/// enum LogLevel {
///     Info = 0,
///     Warning = 1,
///     Error = 2,
/// }
/// ```
#[macro_export]
macro_rules! import_symbol {
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

// ============================================
// register_plugin! - User-facing registration macro
// ============================================

/// Register plugin callbacks with the host application
///
/// This macro generates an exported `plugify_main()` function that registers your
/// plugin's lifecycle callbacks. The host will call this function (via `dlsym`)
/// before calling `plugify_init()`.
///
/// All callbacks are **optional** - you can register any combination you need.
///
/// # Generated Function
///
/// ```ignore
/// #[no_mangle]
/// pub extern "C" fn plugify_main() {
///     // Registers your callbacks
/// }
/// ```
///
/// The host must call this function before `plugify_init()`:
///
/// ```cpp
/// // In C++ host
/// auto plugify_main = (void(*)())dlsym(handle, "plugify_main");
/// plugify_main();  // Register callbacks first
///
/// auto plugify_init = (int(*)(const void*, size_t, int, size_t))
///     dlsym(handle, "plugify_init");
/// plugify_init(...);  // Then initialize
/// ```
///
/// # Callback Signatures
///
/// - `start`: `fn()` - Called when the plugin starts
/// - `update`: `fn(f32)` - Called every frame/tick with delta time
/// - `end`: `fn()` - Called when the plugin shuts down
///
/// # Examples
///
/// ## Register all callbacks
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
/// ## Register only start and end (no update)
///
/// ```
/// # use plugify::*;
/// # fn init_resources() {}
/// # fn cleanup_resources() {}
/// register_plugin! {
///     start: init_resources,
///     end: cleanup_resources,
/// }
/// ```
///
/// ## Register only update callback
///
/// ```
/// # use plugify::*;
/// # fn game_tick(dt: f32) {}
/// register_plugin! {
///     update: game_tick,
/// }
/// ```
///
/// # Trailing Commas
///
/// Trailing commas are allowed for all callbacks:
///
/// ```
/// # use plugify::*;
/// # fn my_start() {}
/// # fn my_update(dt: f32) {}
/// register_plugin! {
///     start: my_start,    // Trailing comma OK
///     update: my_update,  // Trailing comma OK
/// }
/// ```
///
/// # Common Use Cases
///
/// ## Game Plugin
///
/// ```
/// # use plugify::*;
/// fn init_game() {
///     println!("Initializing game systems...");
/// }
///
/// fn game_loop(dt: f32) {
///     // Update game logic every frame
/// }
///
/// fn shutdown_game() {
///     println!("Shutting down game systems...");
/// }
///
/// register_plugin! {
///     start: init_game,
///     update: game_loop,
///     end: shutdown_game,
/// }
/// ```
///
/// ## Audio Plugin (No Update)
///
/// ```
/// # use plugify::*;
/// fn init_audio() {
///     println!("Audio system initialized");
/// }
///
/// fn cleanup_audio() {
///     println!("Audio system cleaned up");
/// }
///
/// // Audio runs asynchronously, no update needed
/// register_plugin! {
///     start: init_audio,
///     end: cleanup_audio,
/// }
/// ```
///
/// ## Logger Plugin (Only Update)
///
/// ```
/// # use plugify::*;
/// fn log_tick(dt: f32) {
///     static mut ELAPSED: f32 = 0.0;
///     unsafe {
///         ELAPSED += dt;
///         if ELAPSED >= 1.0 {
///             println!("1 second elapsed");
///             ELAPSED = 0.0;
///         }
///     }
/// }
///
/// register_plugin! {
///     update: log_tick,
/// }
/// ```
///
/// # Panics
///
/// Panics at runtime (during `plugify_main()` execution) if:
/// - A callback is registered more than once
/// - The callback has already been set by a previous call
///
/// ```should_panic
/// # use plugify::*;
/// # fn my_start() {}
/// // First plugin tries to register
/// register_plugin! {
///     start: my_start,
/// }
///
/// // Second plugin tries to register - PANIC!
/// register_plugin! {
///     start: my_start,  // Error: start_callback already set
/// }
/// ```
///
/// # Notes
///
/// - This macro must be used **exactly once** per plugin
/// - The generated `plugify_main()` function must be called by the host before `plugify_init()`
/// - Callbacks are stored in thread-safe `OnceLock` globals
/// - Missing callbacks are treated as "not implemented" - no error occurs
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
