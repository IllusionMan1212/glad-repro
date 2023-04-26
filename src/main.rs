use glfw::{Action, Context, Key};
use glad_gl::gl;

use glad_repro::shader::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

    glfw::WindowHint::ContextVersion(3, 3);
    glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core);
    glfw::WindowHint::OpenGlForwardCompat(true);

    let (mut window, events) = glfw.create_window(800, 600, "rust-gl", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 800, 600);

        let shader2 = Shader::new("shaders/grid_v.glsl", "shaders/grid_f.glsl")?;

        while !window.should_close() {
            // camera matrices
            let view_mat = glm::ext::look_at(glm::vec3(0.0, 0.0, 0.0), glm::vec3(0.0, 0.0, -1.0), glm::vec3(0.0, 1.0, 0.0));
            let (win_width, win_height) = window.get_size();
            let projection_mat = glm::ext::perspective(glm::radians(45.0), win_width as f32 / win_height as f32, 0.1, 100.0);

            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, &event);
            }

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // drawing objects
            shader2.use_shader();
            shader2.set_mat4fv("view", &view_mat);
            shader2.set_mat4fv("projection", &projection_mat);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            
            glfw.poll_events();
            window.swap_buffers();
        }
    }

    Ok(())
}

fn handle_window_event(window: &mut glfw::Window, event: &glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                gl::Viewport(0, 0, *width, *height);
            }
        }
        _ => {}
    }
}

