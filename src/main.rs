mod glfw;

fn main() {
    glfw::init();

    let mut window = glfw::GLFWWindow::new();

    glfw::set_context(&window); // for OpenGL context

    window.set_window_close_callback();
    window.set_window_key_callback();

    while window.is_running() {
        glfw::poll_events();
        window.swap_buffer();
    }
}
