extern crate glfw;
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
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| 
        window.get_proc_address(symbol) as *const _);

    (glfw, window, events)
}