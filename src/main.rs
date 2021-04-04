mod glfw;

use glfw::glad_bindgen::*;

const VERTEX_SHADER: &'static str = r#"
#version 450 core
void main() {}
\0"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 450 core
layout(location = 0) out vec4 color;
void main() {
    color = vec4(1.0, 1.0, 1.0, 1.0);
}
\0"#;

fn main() {
    glfw::init();

    let mut window = glfw::GLFWWindow::new();

    glfw::set_context(&window); // for OpenGL context
    window.set_window_close_callback();
    window.set_window_key_callback();

    let (vertex_shader, fragment_shader) = unsafe {
        (
            glfw::glad_bindgen::glad_glCreateShader.expect("GLAD uninitialized!")(
                glfw::glad_bindgen::GL_VERTEX_SHADER,
            ),
            glfw::glad_bindgen::glad_glCreateShader.expect("GLAD uninitialized!")(
                glfw::glad_bindgen::GL_FRAGMENT_SHADER,
            ),
        )
    };
    unsafe {
        let shader = VERTEX_SHADER.as_ptr() as *const std::os::raw::c_char;
        glad_glShaderSource.expect("GLAD uninitialized!")(
            vertex_shader,
            1,
            &shader,
            std::ptr::null_mut(),
        );
        glad_glCompileShader.expect("GLAD uninitialized!")(vertex_shader);
    }
    unsafe {
        let shader = FRAGMENT_SHADER.as_ptr() as *const std::os::raw::c_char;
        glad_glShaderSource.expect("GLAD uninitialized!")(
            fragment_shader,
            1,
            &shader,
            std::ptr::null_mut(),
        );
        glad_glCompileShader.expect("GLAD uninitialized!")(fragment_shader);
    }

    let program = unsafe { glad_glCreateProgram.expect("GLAD uninitialized!")() };
    unsafe {
        glad_glAttachShader.expect("GLAD uninitialized!")(program, vertex_shader);
        glad_glAttachShader.expect("GLAD uninitialized!")(program, fragment_shader);
        glad_glLinkProgram.expect("GLAD uninitialized!")(program);
    }

    let buffer_data = [
        -1.0f32, -1.0f32, 0.0f32, // bottom-left
        1.0f32, -1.0f32, 0.0f32, // bottom-right
        0.0f32, 1.0f32, 0.0f32, // top
    ];
    let mut vb: glfw::glad_bindgen::GLuint = 0;
    unsafe {
        glfw::glad_bindgen::glad_glGenBuffers.expect("GLAD uninitialized!")(1, &mut vb);
        glfw::glad_bindgen::glad_glBindBuffer.expect("GLAD uninitialized!")(GL_ARRAY_BUFFER, vb);
        glfw::glad_bindgen::glad_glBufferData.expect("GLAD uninitialized!")(
            GL_ARRAY_BUFFER,
            std::mem::size_of_val(&buffer_data) as i64,
            buffer_data.as_ptr() as *const std::ffi::c_void,
            GL_STATIC_DRAW,
        );
    }

    while window.is_running() {
        glfw::poll_events();
        window.swap_buffer();

        unsafe {
            glfw::glad_bindgen::glad_glClear.expect("GLAD uninitialized!")(
                glfw::glad_bindgen::GL_COLOR_BUFFER_BIT,
            );

            glad_glUseProgram.expect("GLAD uninitialized!")(program);
            glad_glEnableVertexAttribArray.expect("GLAD uninitialized!")(0);
            // glad_glBindBuffer.expect("GLAD uninitialized!")(GL_ARRAY_BUFFER, vb); // already bound
            glad_glVertexAttribPointer.expect("GLAD uninitialized!")(0, 3, GL_FLOAT, GL_FALSE as u8, 0, std::ptr::null_mut());
            glad_glDrawArrays.expect("GLAD uninitialized!")(GL_TRIANGLES, 0, 3);
        }
    }

    // GLFW state, commented because RAII is needed
    // (otherwise `window` is cleaned up _after_ termination)
    // glfw::terminate();
}
