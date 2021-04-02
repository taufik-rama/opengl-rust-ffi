#[repr(C)]
struct GLFWwindow(std::ffi::c_void);
#[repr(C)]
struct GLFWmonitor(std::ffi::c_void);

type GLFWerrorfun = extern "C" fn(isize, *const std::os::raw::c_char);

extern "C" fn glfw_error_callback(error_code: isize, desc: *const std::os::raw::c_char) {
    println!(
        "GLFW error: {} (error code {})",
        unsafe { std::ffi::CStr::from_ptr(desc).to_str().unwrap() },
        error_code
    );
}

type GLFWwindowclosefun = extern "C" fn(*mut GLFWwindow);

extern "C" fn glfw_window_close_callback(_window: *mut GLFWwindow) {
    unsafe {
        IS_RUNNING = false;
    }
}

// type GLFWglproc = extern "C" fn();

type GLADloadproc = unsafe extern "C" fn (name: *const std::os::raw::c_char) -> *mut std::ffi::c_void;

extern "C" {
    fn gladLoadGLLoader(load: GLADloadproc) -> isize;
    fn glfwInit() -> isize;
    fn glfwCreateWindow(
        width: isize,
        height: isize,
        title: *const std::os::raw::c_char,
        monitor: *mut GLFWwindow,
        share: *mut GLFWmonitor,
    ) -> *mut GLFWwindow;
    fn glfwSetErrorCallback(cb: GLFWerrorfun) -> GLFWerrorfun;
    fn glfwSetWindowCloseCallback(
        window: *mut GLFWwindow,
        cb: GLFWwindowclosefun,
    ) -> GLFWwindowclosefun;
    fn glfwPollEvents();
    fn glfwSwapBuffers(window: *mut GLFWwindow);
    fn glfwGetProcAddress(procname: *const std::os::raw::c_char) -> *mut std::ffi::c_void;
    fn glfwMakeContextCurrent(window: *mut GLFWwindow);
}

fn is_null(address: *const isize) -> bool {
    address == (0x0 as *const isize)
}

static mut IS_RUNNING: bool = true;

fn main() {
    unsafe {
        glfwSetErrorCallback(glfw_error_callback);
    }

    assert_eq!(unsafe { glfwInit() }, 1);

    let title = std::ffi::CString::new("title").unwrap();
    let window = unsafe {
        glfwCreateWindow(
            1920,
            1080,
            title.into_raw(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    if is_null(window as *const isize) {
        panic!("Uninitialized!");
    }

    unsafe {
        glfwMakeContextCurrent(window);
        assert_ne!(gladLoadGLLoader(glfwGetProcAddress as GLADloadproc), 0);
        glfwSetWindowCloseCallback(window, glfw_window_close_callback);
    }

    while unsafe { IS_RUNNING } {
        unsafe {
            glfwPollEvents();
            glfwSwapBuffers(window);
        }
    }
}
