extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

mod macros;

mod mesh;
mod model;
mod parse_obj;

mod shader;
use shader::Shader;

mod texture;

mod window;

use cgmath::{Matrix4, Vector3, vec3,  Deg, Rad, perspective, Vector4};
mod math;
use cgmath::prelude::*;


// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

// const MODEL_PATH: &str = "resources/objects/redcube/cube.obj";
// const MODEL_PATH: &str = "resources/objects/redcube/cube2.obj";
// const MODEL_PATH: &str = "resources/objects/statue/statue.obj";
const MODEL_PATH: &str = "resources/objects/42/42.obj";
// const MODEL_PATH: &str = "resources/objects/teapot/teapot2.obj";

fn main() {

    let (mut glfw, mut window, events) = 
        window::create_window(SCR_WIDTH, SCR_HEIGHT);
    

    let (our_shader, vbo, vao, texture1, texture2, indices, model) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // build and compile our shader program
        // ------------------------------------
        let our_shader = Shader::new(
            "src/shaders/shader.vs",
            "src/shaders/shader.fs");

        let (vertices, indices) = parse_obj::load_model(MODEL_PATH);
        // println!("{:?}", ourModel);
            
        let model: Matrix4<f32> = model::generate_model_matrix(&vertices);

        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                        (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &indices[0] as *const u32 as *const c_void,
                        gl::STATIC_DRAW);
                        
                        let stride = 3 * mem::size_of::<GLfloat>() as GLsizei;
                        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // texture coord attribute
        // gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        // gl::EnableVertexAttribArray(1);
        
        let texture1 = texture::load_texture("resources/textures/container.jpg");
        let texture2 = texture::load_texture("resources/textures/awesomeface.png");
        
        // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
        // -------------------------------------------------------------------------------------------
        our_shader.use_program();
        our_shader.setInt(c_str!("texture1"), 0);
        our_shader.setInt(c_str!("texture2"), 1);

        (our_shader, vbo, vao, texture1, texture2, indices, model)
    };

    // needed for fps conter
    let mut prev_time = 0.0;
    let mut curr_time;
    let mut time_diff;
    let mut counter = 0.0;

    // render loop
    // -----------
    while !window.should_close() {

        // fps counter
        curr_time = glfw.get_time();
        time_diff = curr_time - prev_time;
        counter += 1.0;
        if time_diff >= 1.0 {
            let fps = (1.0 / time_diff) * counter;
            let fps = format!("{:.2}", fps);
            let new_title = String::from("SCOP - FPS: ") + &fps[..];
            window.set_title(&new_title[..]);
            prev_time = curr_time;
            counter = 0.0;
        }

        // events
        // -----
        process_events(&mut window, &events);

        // render
        // ------
        unsafe {
            // clear screen
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // bind textures on corresponding texture units
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);
            
            // activate shader
            our_shader.use_program();
            
            // create transformations
            // NOTE: cgmath requires axis vectors to be normalized!
            let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0.0, -5.));
            let projection: Matrix4<f32> = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            // retrieve the matrix uniform locations
            let model_loc = gl::GetUniformLocation(our_shader.id, c_str!("model").as_ptr());
            let view_loc = gl::GetUniformLocation(our_shader.id, c_str!("view").as_ptr());
            // pass them to the shaders (3 different ways)
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);
            // note: currently we set the projection matrix each frame, but since the projection matrix rarely changes it's often best practice to set it outside the main loop only once.
            our_shader.setMat4(c_str!("projection"), &projection);

            // render box
            gl::BindVertexArray(vao);

            let angle = 40.0 as f32 * glfw.get_time() as f32;
            let rotation = Matrix4::from_axis_angle(vec3(0.0, 1.0, 0.0), Deg(angle)) * model;
            // let model = Matrix4::from_axis_angle(vec3(0.0, 1.0, 0.0), Deg(angle));
            our_shader.setMat4(c_str!("model"), &rotation);

            // gl::DrawArrays(gl::TRIANGLES, 0, 72);
            gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
    }
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}
