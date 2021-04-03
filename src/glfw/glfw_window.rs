#![allow(dead_code)]

use super::glfw_bindgen;

// Currently use single global state
// TODO :: Use "user data" provided by the GLFW window
pub static mut IS_RUNNING: bool = false;

pub struct GLFWWindow {
    pub window: *mut glfw_bindgen::GLFWwindow,
}

impl Drop for GLFWWindow {
    fn drop(&mut self) {
        unsafe {
            glfw_bindgen::glfwDestroyWindow(self.window);
        }
    }
}

impl GLFWWindow {
    pub fn new() -> GLFWWindow {
        let window = unsafe {
            glfw_bindgen::glfwCreateWindow(
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
            glfw_bindgen::glfwSetWindowCloseCallback(self.window, Some(super::glfw_window_close_callback));
        }
    }

    pub fn set_window_key_callback(&mut self) {
        unsafe {
            glfw_bindgen::glfwSetKeyCallback(self.window, Some(super::glfw_key_callback));
        }
    }

    pub fn swap_buffer(&mut self) {
        unsafe {
            glfw_bindgen::glfwSwapBuffers(self.window);
        }
    }
}
