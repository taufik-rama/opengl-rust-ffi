#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_unsafe)]

mod glad_bindgen;
mod glfw_bindgen;
pub mod glfw_window;

// re-exports
pub use glfw_window::GLFWWindow;

unsafe extern "C" fn glfw_error_callback(
    error_code: std::os::raw::c_int,
    desc: *const std::os::raw::c_char,
) {
    println!(
        "GLFW error: {} (error code {})",
        unsafe { std::ffi::CStr::from_ptr(desc).to_str().unwrap() },
        error_code
    );
}

unsafe extern "C" fn glfw_key_callback(
    _window: *mut glfw_bindgen::GLFWwindow,
    key: std::os::raw::c_int,
    _scancode: std::os::raw::c_int,
    action: std::os::raw::c_int,
    _mods: std::os::raw::c_int,
) {
    println!("Pressed: {}, action: {}", key, action);
}

unsafe extern "C" fn glfw_window_close_callback(_window: *mut glfw_bindgen::GLFWwindow) {
    println!("Closing window...");
    unsafe {
        glfw_window::IS_RUNNING = false;
    }
}

pub fn is_null(address: *const isize) -> bool {
    address == (0x0 as *const isize)
}

pub fn init() {
    unsafe {
        glfw_bindgen::glfwSetErrorCallback(Some(glfw_error_callback));
        assert_eq!(glfw_bindgen::glfwInit(), 1);
    }
}

unsafe extern "C" fn glad_loadproc(
    name: *const ::std::os::raw::c_char,
) -> *mut std::os::raw::c_void {
    match glfw_bindgen::glfwGetProcAddress(name) {
        Some(e) => e as *mut std::os::raw::c_void,
        None => std::ptr::null_mut() as *mut std::os::raw::c_void,
    }
}

pub fn set_context(window: &glfw_window::GLFWWindow) {
    unsafe {
        glfw_bindgen::glfwMakeContextCurrent(window.window);
        assert_ne!(glad_bindgen::gladLoadGLLoader(Some(glad_loadproc)), 0);
    }
}

pub fn poll_events() {
    unsafe {
        glfw_bindgen::glfwPollEvents();
    }
}
