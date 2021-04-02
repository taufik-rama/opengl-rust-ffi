#![allow(dead_code)]

use super::ffi;

// Currently use single global state
// TODO :: Use "user data" provided by the GLFW window
pub static mut IS_RUNNING: bool = false;

pub struct GLFWWindow {
    pub window: *mut ffi::CGLFWwindow,
}

impl Drop for GLFWWindow {
    fn drop(&mut self) {
        unsafe {
            ffi::glfwDestroyWindow(self.window);
        }
    }
}

impl GLFWWindow {
    pub fn new() -> GLFWWindow {
        let window = unsafe {
            ffi::glfwCreateWindow(
                960,
                540,
                std::ffi::CString::new("title")
                    .expect("Invalid title")
                    .into_raw(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };
        if super::is_null(window as *const isize) {
            panic!("Uninitialized!");
        }
        unsafe {
            IS_RUNNING = true;
        }
        GLFWWindow { window: window }
    }

    // Global state should be moved to GLFW object state
    pub fn is_running(&self) -> bool {
        unsafe { IS_RUNNING }
    }

    pub fn set_window_close_callback(&mut self) {
        unsafe {
            ffi::glfwSetWindowCloseCallback(self.window, super::glfw_window_close_callback);
        }
    }

    pub fn set_window_key_callback(&mut self) {
        unsafe {
            ffi::glfwSetKeyCallback(self.window, super::glfw_key_callback);
        }
    }

    pub fn swap_buffer(&mut self) {
        unsafe {
            ffi::glfwSwapBuffers(self.window);
        }
    }
}
