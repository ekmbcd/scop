extern crate glfw;
use self::glfw::Context;

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

mod macros;
mod model;
mod parse_obj;
mod texture;
mod window;

mod shader;
use shader::Shader;

mod matrix;
use matrix::Matrix4;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

// const MODEL_PATH: &str = "resources/objects/redcube/cube.obj";
// const MODEL_PATH: &str = "resources/objects/redcube/cube2.obj";
// const MODEL_PATH: &str = "resources/objects/statue/statue.obj";
const MODEL_PATH: &str = "resources/objects/42/42.obj";
// const MODEL_PATH: &str = "resources/objects/teapot/teapot.obj";

fn main() {

    let path;
    if let Some(argument) = std::env::args().nth(1) {
        path = argument;
    } else {
        path = String::from(MODEL_PATH);
    }

    let (mut glfw, mut window, events) = 
        window::create_window(SCR_WIDTH, SCR_HEIGHT);

    let (our_shader, vbo, vao, texture, indices, model) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // build and compile our shader program
        // ------------------------------------
        let our_shader = Shader::new(
            "src/shaders/shader.vs",
            "src/shaders/shader.fs");

        let (vertices, indices) = parse_obj::load_model(&path);
            
        let model = model::generate_model_matrix(&vertices);

        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        // vao: vertex array object
        gl::GenVertexArrays(1, &mut vao);
        // vbo: vertex buffer (coordinates)
        gl::GenBuffers(1, &mut vbo);
        // element buffer (faces)
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &indices[0] as *const u32 as *const c_void,    
            gl::STATIC_DRAW
        );
        
        // stride is the "jump" between vertices in the vbo
        let stride = 3 * mem::size_of::<GLfloat>() as GLsizei;
        // position attribute
        gl::VertexAttribPointer(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            stride, 
            ptr::null()
        );
        gl::EnableVertexAttribArray(0);
        
        let texture = texture::load_texture("resources/textures/ponies.jpg");
        
        // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
        // -------------------------------------------------------------------------------------------
        our_shader.use_program();
        our_shader.set_int(c_str!("texture1"), 0);

        // enable face culling to increase performances
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);  

        (our_shader, vbo, vao, texture, indices, model)
    };

    // needed for fps conter
    let mut prev_time = 0.0;
    let mut curr_time;
    let mut counter = 0.0;

    //projection matrix
    let mut projection = Matrix4::perspective(
        45.0, 
        SCR_WIDTH as f32 / SCR_HEIGHT as f32, 
        0.1, 
        100.0
    );

    //matrix used to rotate the object
    let mut transformation = Matrix4::identity();

    //used to detect if mouse button is pressed
    let mut mouse_pressed = false;

    // mouse position
    let mut last_x = 0.0;
    let mut last_y = 0.0;

    //used to mix textures
    let mut delta_mix: f32 = 0.01;
    let mut texture_mix = 1.0;

    // used as fov (in degrees)
    let mut zoom = 45.0;

    // render loop
    // -----------
    while !window.should_close() {

        // fps counter
        curr_time = glfw.get_time();
        let time_diff = curr_time - prev_time;
        counter += 1.0;
        if time_diff >= 1.0 {
            let fps = (1.0 / time_diff) * counter;
            let fps = format!("{:.2}", fps);
            let new_title = String::from("SCOP - FPS: ") + &fps[..];
            window.set_title(&new_title[..]);
            prev_time = curr_time;
            counter = 0.0;
        }

        // smoothly switch textures
        texture_mix += delta_mix;
        if texture_mix > 1.0 {
            texture_mix = 1.0;
        }
        else if texture_mix < 0.0 {
            texture_mix = 0.0;
        }

        // events
        // -----
        window::process_events(
            &events, 
            &mut mouse_pressed, 
            &mut last_x, 
            &mut last_y, 
            &mut transformation,
            &mut projection,
            &mut window,
            &mut zoom,
            &mut delta_mix
        );

        // render
        // ------
        unsafe {
            // clear screen
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // bind textures on corresponding texture units
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            
            // activate shader
            // our_shader.use_program();
            
            // create transformations
            let view = Matrix4::from_translation(0.0, 0.0, -5.);
            
            // render box
            gl::BindVertexArray(vao);

            // only rotate when mouse button is not pressed
            if !mouse_pressed {
                transformation = transformation * Matrix4::from_angle_y(0.02);
            }

            // set uniforms for shaders
            our_shader.set_mat4(c_str!("model"), &model);
            our_shader.set_mat4(c_str!("transformation"), &transformation);
            our_shader.set_mat4(c_str!("projection"), &projection);
            our_shader.set_mat4(c_str!("view"), &view);
			our_shader.set_float(c_str!("textureMix"), texture_mix);

            // draw drame
            gl::DrawElements(
                gl::TRIANGLES, 
                indices.len() as i32, 
                gl::UNSIGNED_INT, 
                ptr::null()
            );
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
