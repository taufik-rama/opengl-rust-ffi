mod ffi;
pub mod glfw_window;

// re-exports
pub use glfw_window::GLFWWindow;

extern "C" fn glfw_error_callback(error_code: isize, desc: *const std::os::raw::c_char) {
    println!(
        "GLFW error: {} (error code {})",
        unsafe { std::ffi::CStr::from_ptr(desc).to_str().unwrap() },
        error_code
    );
}

extern "C" fn glfw_key_callback(
    _window: *mut ffi::CGLFWwindow,
    key: isize,
    _scancode: isize,
    action: isize,
    _mods: isize,
) {
    println!("Pressed: {}, action: {}", key, action);
}

extern "C" fn glfw_window_close_callback(_window: *mut ffi::CGLFWwindow) {
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
        ffi::glfwSetErrorCallback(glfw_error_callback);
        assert_eq!(ffi::glfwInit(), 1);
    }
}

pub fn set_context(window: &glfw_window::GLFWWindow) {
    unsafe {
        ffi::glfwMakeContextCurrent(window.window);
        assert_ne!(
            ffi::gladLoadGLLoader(ffi::glfwGetProcAddress as ffi::GLADloadproc),
            0
        );
    }
}

pub fn poll_events() {
    unsafe {
        ffi::glfwPollEvents();
    }
}
