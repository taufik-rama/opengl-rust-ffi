#![allow(dead_code)]

#[repr(C)]
pub struct CGLFWwindow(std::ffi::c_void);
#[repr(C)]
pub struct CGLFWmonitor(std::ffi::c_void);

pub type GLFWerrorfun = extern "C" fn(isize, *const std::os::raw::c_char);

pub type CGLFWwindowclosefun = extern "C" fn(*mut CGLFWwindow);

pub type GLFWkeyfun = extern "C" fn(*mut CGLFWwindow, isize, isize, isize, isize);

pub type GLADloadproc =
    unsafe extern "C" fn(name: *const std::os::raw::c_char) -> *mut std::ffi::c_void;

extern "C" {
    pub fn gladLoadGLLoader(load: GLADloadproc) -> isize;
}

extern "C" {
    pub fn glfwCreateWindow(
        width: isize,
        height: isize,
        title: *const std::os::raw::c_char,
        monitor: *mut CGLFWwindow,
        share: *mut CGLFWmonitor,
    ) -> *mut CGLFWwindow;

    pub fn glfwDestroyWindow(window: *mut CGLFWwindow);

    pub fn glfwGetProcAddress(procname: *const std::os::raw::c_char) -> *mut std::ffi::c_void;

    pub fn glfwInit() -> isize;

    pub fn glfwMakeContextCurrent(window: *mut CGLFWwindow);

    pub fn glfwPollEvents();

    pub fn glfwSetErrorCallback(cb: GLFWerrorfun) -> GLFWerrorfun;

    pub fn glfwSetKeyCallback(window: *mut CGLFWwindow, cb: GLFWkeyfun) -> GLFWkeyfun;

    pub fn glfwSetWindowCloseCallback(
        window: *mut CGLFWwindow,
        cb: CGLFWwindowclosefun,
    ) -> CGLFWwindowclosefun;

    pub fn glfwSwapBuffers(window: *mut CGLFWwindow);
}
