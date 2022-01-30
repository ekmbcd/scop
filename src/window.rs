extern crate glfw;
extern crate gl;
use std::sync::mpsc::Receiver;

use glfw::{Key, Action, MouseButton};

use crate::matrix::Matrix4;

use self::glfw::Context;


// TODO: manage error
pub fn create_window(width: u32, height: u32) -> (
    glfw::Glfw, 
    glfw::Window,
    std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>
) {
    // glfw: initialize and configure
    // ------------------------------
	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .unwrap();
    // set openGl version
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // additional settings for macOs
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = 
        glfw.create_window(
            width, 
            height, 
            "SCOP", 
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window");

    window.make_current();
    // enable window to catch events
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_mouse_button_polling(true);



    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| 
        window.get_proc_address(symbol) as *const _);

    (glfw, window, events)
}

pub fn process_events(
    events: &Receiver<(f64, glfw::WindowEvent)>,
    left_mouse_pressed: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
    transformation: &mut Matrix4,
    projection: &mut Matrix4,
    window: &mut glfw::Window,
    zoom: &mut f32,
    delta_mix: &mut f32,
    view: &mut Matrix4,
    right_mouse_pressed: &mut bool,

) {
    for (_, event) in glfw::flush_messages(events) {
        
        match event {
            glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                *zoom -= yoffset as f32 * 2.0;
                let (width, height) = window.get_size();
                *projection = Matrix4::perspective(
                    *zoom, 
                    width as f32 / height as f32, 
                    0.1, 
                    100.0
                );
            }

            // resize window
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions;
                unsafe { 
                    gl::Viewport(0, 0, width, height);
                    // update projection matrix to keep aspect ratio
                    *projection = Matrix4::perspective(
                        *zoom, 
                        width as f32 / height as f32, 
                        0.1, 
                        100.0
                    );
                }
            }

            glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) => {
                *left_mouse_pressed = true;
            }

            glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Release, _) => {
                *left_mouse_pressed = false;
            }

            glfw::WindowEvent::MouseButton(MouseButton::Button2, Action::Press, _) => {
                *right_mouse_pressed = true;
            }

            glfw::WindowEvent::MouseButton(MouseButton::Button2, Action::Release, _) => {
                *right_mouse_pressed = false;
            }

            glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                *delta_mix = - *delta_mix;
            }

            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }

            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                let (xpos, ypos) = (xpos as f32, ypos as f32);
                if *left_mouse_pressed {
    
                    let xoffset = xpos - *last_x;
                    let yoffset = *last_y - ypos; // reversed since y-coordinates go from bottom to top
                    
                    *transformation = *transformation * Matrix4::from_angle_x(yoffset * 0.01);
                    *transformation = *transformation * Matrix4::from_angle_y(- xoffset * 0.01);
                }
                if *right_mouse_pressed {
    
                    let xoffset = xpos - *last_x;
                    let yoffset = *last_y - ypos; // reversed since y-coordinates go from bottom to top
                    
                    *view = *view * Matrix4::from_angle_x(yoffset * 0.01);
                    *view = *view * Matrix4::from_angle_y(- xoffset * 0.01);
                }
                *last_x = xpos;
                *last_y = ypos;
            }

            _ => {}
        }
    }
}

pub fn process_input(window: &mut glfw::Window, view: &mut Matrix4) {

    if window.get_key(Key::W) == Action::Press {
        *view = *view * Matrix4::from_translation(0.0, 0.0, 0.1);
    }

    if window.get_key(Key::S) == Action::Press {
        *view = *view * Matrix4::from_translation(0.0, 0.0, -0.1);
    }

    if window.get_key(Key::A) == Action::Press {
        *view = *view * Matrix4::from_translation(0.1, 0.0, 0.0);
    }

    if window.get_key(Key::D) == Action::Press {
        *view = *view * Matrix4::from_translation(-0.1, 0.0, 0.0);
    }

    if window.get_key(Key::R) == Action::Press {
        *view = *view * Matrix4::from_translation(0.0, -0.1, 0.0);
    }

    if window.get_key(Key::F) == Action::Press {
        *view = *view * Matrix4::from_translation(0.0, 0.1, 0.0);
    }

    if window.get_key(Key::T) == Action::Press {
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::Disable(gl::CULL_FACE);
        }
    }

    if window.get_key(Key::G) == Action::Press {
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK); 
        }
    }
}
