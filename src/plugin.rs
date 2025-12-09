use std::sync::OnceLock;
use crate::{import_symbol, vector::*, string::*, variant::*};

import_symbol!(get_method_ptr, GET_METHOD_PTR, init_get_method_ptr, (name:&PlgString) -> usize);
import_symbol!(get_base_dir, GET_BASE_DIR, init_get_base_dir, () -> PlgString);
import_symbol!(get_extensions_dir, GET_EXTENSIONS_DIR, init_get_extensions_dir, () -> PlgString);
import_symbol!(get_configs_dir, GET_CONFIGS_DIR, init_get_configs_dir, () -> PlgString);
import_symbol!(get_data_dir, GET_DATA_DIR, init_get_data_dir, () -> PlgString);
import_symbol!(get_logs_dir, GET_LOGS_DIR, init_get_logs_dir, () -> PlgString);
import_symbol!(get_cache_dir, GET_CACHE_DIR, init_get_cache_dir, () -> PlgString);
import_symbol!(is_extension_loaded, IS_EXTENSION_LOADED, init_is_extension_loaded, (name:&PlgString, constraint:&PlgString) -> bool);

import_symbol!(get_plugin_id, GET_PLUGIN_ID, init_get_plugin_id, (handle:PluginHandle) -> usize);
import_symbol!(get_plugin_name, GET_PLUGIN_NAME, init_get_plugin_name, (handle:PluginHandle) -> PlgString);
import_symbol!(get_plugin_description, GET_PLUGIN_DESCRIPTION, init_get_plugin_description, (handle:PluginHandle) -> PlgString);
import_symbol!(get_plugin_version, GET_PLUGIN_VERSION, init_get_plugin_version, (handle:PluginHandle) -> PlgString);
import_symbol!(get_plugin_author, GET_PLUGIN_AUTHOR, init_get_plugin_author, (handle:PluginHandle) -> PlgString);
import_symbol!(get_plugin_website, GET_PLUGIN_WEBSITE, init_get_plugin_website, (handle:PluginHandle) -> PlgString);
import_symbol!(get_plugin_license, GET_PLUGIN_LICENSE, init_get_plugin_license, (handle:PluginHandle) -> PlgString);
import_symbol!(get_plugin_location, GET_PLUGIN_LOCATION, init_get_plugin_location, (handle:PluginHandle) -> PlgString);
import_symbol!(get_plugin_dependencies, GET_PLUGIN_DEPENDENCIES, init_get_plugin_dependencies, (handle:PluginHandle) -> PlgVector<PlgString>);

// Constants
const K_API_VERSION: i32 = 1;

// Plugin handle type
type PluginHandle = usize;
const _: () = assert!(size_of::<PluginHandle>() == size_of::<*const ()>());
const _: () = assert!(align_of::<PluginHandle>() == align_of::<*const ()>());

// Global plugin state
#[derive(Debug)]
pub struct PluginInfo {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub website: String,
    pub license: String,
    pub location: String,
    pub dependencies: Vec<String>,
}

#[repr(C)]
#[derive(Debug)]
pub struct PluginContext {
    has_update: bool,
    has_start: bool,
    has_end: bool,
}

impl Default for PluginContext {
    fn default() -> Self {
        Self {
            has_update: false,
            has_start: false,
            has_end: false,
        }
    }
}

pub struct PluginCallbacks {
    pub update_callback: OnceLock<fn(f32)>,
    pub start_callback: OnceLock<fn()>,
    pub end_callback: OnceLock<fn()>,
}

impl Default for PluginCallbacks {
    fn default() -> Self {
        Self {
            update_callback: OnceLock::new(),
            start_callback: OnceLock::new(),
            end_callback: OnceLock::new(),
        }
    }
}

// Global statics (you may want to use a different pattern in real code)
pub static BASE_DIR: OnceLock<String> = OnceLock::new();
pub static EXTENSIONS_DIR: OnceLock<String> = OnceLock::new();
pub static CONFIGS_DIR: OnceLock<String> = OnceLock::new();
pub static DATA_DIR: OnceLock<String> = OnceLock::new();
pub static LOGS_DIR: OnceLock<String> = OnceLock::new();
pub static CACHE_DIR: OnceLock<String> = OnceLock::new();
pub static PLUGIN: OnceLock<PluginInfo> = OnceLock::new();
pub static HANDLE: OnceLock<PluginHandle> = OnceLock::new();
pub static CONTEXT: OnceLock<PluginContext> = OnceLock::new();
pub static CALLBACKS: OnceLock<PluginCallbacks> = OnceLock::new();

pub fn on_plugin_start(func: fn()) {
    let callbacks = CALLBACKS.get_or_init(||PluginCallbacks::default());
    callbacks.start_callback.set(func).expect("start_callback: already set");
}

pub fn on_plugin_update(func: fn(f32)) {
    let callbacks = CALLBACKS.get_or_init(||PluginCallbacks::default());
    callbacks.update_callback.set(func).expect("update_callback: already set");
}

pub fn on_plugin_end(func: fn()) {
    let callbacks = CALLBACKS.get_or_init(||PluginCallbacks::default());
    callbacks.end_callback.set(func).expect("end_callback: already set");
}

#[unsafe(no_mangle)]
pub extern "C" fn plugify_init(
    data: *const usize,
    len: usize,
    version: i32,
    handle: usize,
) -> i32 {
    if version < K_API_VERSION {
        return K_API_VERSION;
    }

    let api = unsafe { std::slice::from_raw_parts(data, len) };
    let mut i = 0;

    // Set all the function pointers
    init_get_method_ptr(api[i]); i += 1;
    init_get_base_dir(api[i]); i += 1;
    init_get_extensions_dir(api[i]); i += 1;
    init_get_configs_dir(api[i]); i += 1;
    init_get_data_dir(api[i]); i += 1;
    init_get_logs_dir(api[i]); i += 1;
    init_get_cache_dir(api[i]); i += 1;
    init_is_extension_loaded(api[i]); i += 1;

    init_get_plugin_id(api[i]); i += 1;
    init_get_plugin_name(api[i]); i += 1;
    init_get_plugin_description(api[i]); i += 1;
    init_get_plugin_version(api[i]); i += 1;
    init_get_plugin_author(api[i]); i += 1;
    init_get_plugin_website(api[i]); i += 1;
    init_get_plugin_license(api[i]); i += 1;
    init_get_plugin_location(api[i]); i += 1;
    init_get_plugin_dependencies(api[i]); i += 1;

    init_construct_string(api[i]); i += 1;
    init_destroy_string(api[i]); i += 1;
    init_get_string_data(api[i]); i += 1;
    init_get_string_length(api[i]); i += 1;
    init_assign_string(api[i]); i += 1;

    init_destroy_variant(api[i]); i += 1;

    // Vector constructors (20 total)
    init_construct_vector_bool(api[i]); i += 1;
    init_construct_vector_char8(api[i]); i += 1;
    init_construct_vector_char16(api[i]); i += 1;
    init_construct_vector_int8(api[i]); i += 1;
    init_construct_vector_int16(api[i]); i += 1;
    init_construct_vector_int32(api[i]); i += 1;
    init_construct_vector_int64(api[i]); i += 1;
    init_construct_vector_uint8(api[i]); i += 1;
    init_construct_vector_uint16(api[i]); i += 1;
    init_construct_vector_uint32(api[i]); i += 1;
    init_construct_vector_uint64(api[i]); i += 1;
    init_construct_vector_pointer(api[i]); i += 1;
    init_construct_vector_float(api[i]); i += 1;
    init_construct_vector_double(api[i]); i += 1;
    init_construct_vector_string(api[i]); i += 1;
    init_construct_vector_variant(api[i]); i += 1;
    init_construct_vector_vector2(api[i]); i += 1;
    init_construct_vector_vector3(api[i]); i += 1;
    init_construct_vector_vector4(api[i]); i += 1;
    init_construct_vector_matrix4x4(api[i]); i += 1;

    // Vector destructors (20 total)
    init_destroy_vector_bool(api[i]); i += 1;
    init_destroy_vector_char8(api[i]); i += 1;
    init_destroy_vector_char16(api[i]); i += 1;
    init_destroy_vector_int8(api[i]); i += 1;
    init_destroy_vector_int16(api[i]); i += 1;
    init_destroy_vector_int32(api[i]); i += 1;
    init_destroy_vector_int64(api[i]); i += 1;
    init_destroy_vector_uint8(api[i]); i += 1;
    init_destroy_vector_uint16(api[i]); i += 1;
    init_destroy_vector_uint32(api[i]); i += 1;
    init_destroy_vector_uint64(api[i]); i += 1;
    init_destroy_vector_pointer(api[i]); i += 1;
    init_destroy_vector_float(api[i]); i += 1;
    init_destroy_vector_double(api[i]); i += 1;
    init_destroy_vector_string(api[i]); i += 1;
    init_destroy_vector_variant(api[i]); i += 1;
    init_destroy_vector_vector2(api[i]); i += 1;
    init_destroy_vector_vector3(api[i]); i += 1;
    init_destroy_vector_vector4(api[i]); i += 1;
    init_destroy_vector_matrix4x4(api[i]); i += 1;

    // Vector size getters (20 total)
    init_get_vector_size_bool(api[i]); i += 1;
    init_get_vector_size_char8(api[i]); i += 1;
    init_get_vector_size_char16(api[i]); i += 1;
    init_get_vector_size_int8(api[i]); i += 1;
    init_get_vector_size_int16(api[i]); i += 1;
    init_get_vector_size_int32(api[i]); i += 1;
    init_get_vector_size_int64(api[i]); i += 1;
    init_get_vector_size_uint8(api[i]); i += 1;
    init_get_vector_size_uint16(api[i]); i += 1;
    init_get_vector_size_uint32(api[i]); i += 1;
    init_get_vector_size_uint64(api[i]); i += 1;
    init_get_vector_size_pointer(api[i]); i += 1;
    init_get_vector_size_float(api[i]); i += 1;
    init_get_vector_size_double(api[i]); i += 1;
    init_get_vector_size_string(api[i]); i += 1;
    init_get_vector_size_variant(api[i]); i += 1;
    init_get_vector_size_vector2(api[i]); i += 1;
    init_get_vector_size_vector3(api[i]); i += 1;
    init_get_vector_size_vector4(api[i]); i += 1;
    init_get_vector_size_matrix4x4(api[i]); i += 1;

    // Vector data getters (20 total)
    init_get_vector_data_bool(api[i]); i += 1;
    init_get_vector_data_char8(api[i]); i += 1;
    init_get_vector_data_char16(api[i]); i += 1;
    init_get_vector_data_int8(api[i]); i += 1;
    init_get_vector_data_int16(api[i]); i += 1;
    init_get_vector_data_int32(api[i]); i += 1;
    init_get_vector_data_int64(api[i]); i += 1;
    init_get_vector_data_uint8(api[i]); i += 1;
    init_get_vector_data_uint16(api[i]); i += 1;
    init_get_vector_data_uint32(api[i]); i += 1;
    init_get_vector_data_uint64(api[i]); i += 1;
    init_get_vector_data_pointer(api[i]); i += 1;
    init_get_vector_data_float(api[i]); i += 1;
    init_get_vector_data_double(api[i]); i += 1;
    init_get_vector_data_string(api[i]); i += 1;
    init_get_vector_data_variant(api[i]); i += 1;
    init_get_vector_data_vector2(api[i]); i += 1;
    init_get_vector_data_vector3(api[i]); i += 1;
    init_get_vector_data_vector4(api[i]); i += 1;
    init_get_vector_data_matrix4x4(api[i]); i += 1;

    // Vector assign (20 total)
    init_assign_vector_bool(api[i]); i += 1;
    init_assign_vector_char8(api[i]); i += 1;
    init_assign_vector_char16(api[i]); i += 1;
    init_assign_vector_int8(api[i]); i += 1;
    init_assign_vector_int16(api[i]); i += 1;
    init_assign_vector_int32(api[i]); i += 1;
    init_assign_vector_int64(api[i]); i += 1;
    init_assign_vector_uint8(api[i]); i += 1;
    init_assign_vector_uint16(api[i]); i += 1;
    init_assign_vector_uint32(api[i]); i += 1;
    init_assign_vector_uint64(api[i]); i += 1;
    init_assign_vector_pointer(api[i]); i += 1;
    init_assign_vector_float(api[i]); i += 1;
    init_assign_vector_double(api[i]); i += 1;
    init_assign_vector_string(api[i]); i += 1;
    init_assign_vector_variant(api[i]); i += 1;
    init_assign_vector_vector2(api[i]); i += 1;
    init_assign_vector_vector3(api[i]); i += 1;
    init_assign_vector_vector4(api[i]); i += 1;
    init_assign_vector_matrix4x4(api[i]); //i += 1;

    // Get directory paths
    BASE_DIR.set(get_base_dir().to_string()).expect("BASE_DIR: can only be set once");
    EXTENSIONS_DIR.set(get_extensions_dir().to_string()).expect("EXTENSIONS_DIR: can only be set once");
    CONFIGS_DIR.set(get_configs_dir().to_string()).expect("CONFIGS_DIR: can only be set once");
    DATA_DIR.set(get_data_dir().to_string()).expect("DATA_DIR: can only be set once");
    LOGS_DIR.set(get_logs_dir().to_string()).expect("LOGS_DIR: can only be set once");
    CACHE_DIR.set(get_cache_dir().to_string()).expect("CACHE_DIR: can only be set once");

    // Store plugin handle
    HANDLE.set(handle).expect("HANDLE: can only be set once");

    PLUGIN.set(PluginInfo {
        id: get_plugin_id(handle),
        name: get_plugin_name(handle).to_string(),
        description: get_plugin_description(handle).to_string(),
        version: get_plugin_version(handle).to_string(),
        author: get_plugin_author(handle).to_string(),
        website: get_plugin_website(handle).to_string(),
        license: get_plugin_license(handle).to_string(),
        location: get_plugin_location(handle).to_string(),
        dependencies: get_plugin_dependencies(handle).to_string(),
    }).expect("PLUGIN: can only be set once");

    let callbacks = CALLBACKS.get_or_init(||PluginCallbacks::default());
    CONTEXT.set(PluginContext {
        has_update: callbacks.update_callback.get().is_some(),
        has_start: callbacks.start_callback.get().is_some(),
        has_end: callbacks.end_callback.get().is_some()
    }).expect("PLUGIN: can only be set once");

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn plugify_plugin_start() {
    let callbacks = CALLBACKS.get().expect("CALLBACKS not initialized");
    let callback = callbacks.start_callback.get().expect("start_callback not initialized");
    callback();
}

#[unsafe(no_mangle)]
pub extern "C" fn plugify_plugin_update(dt: f32) {
    let callbacks = CALLBACKS.get().expect("CALLBACKS not initialized");
    let callback = callbacks.update_callback.get().expect("update_callback not initialized");
    callback(dt);
}

#[unsafe(no_mangle)]
pub extern "C" fn plugify_plugin_end() {
    let callbacks = CALLBACKS.get().expect("CALLBACKS not initialized");
    let callback = callbacks.end_callback.get().expect("end_callback not initialized");
    callback();
}

#[unsafe(no_mangle)]
pub extern "C" fn plugify_plugin_context() -> *const PluginContext {
    CONTEXT.get().expect("CONTEXT not initialized")
}