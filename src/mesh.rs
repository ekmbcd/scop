/*
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;

use cgmath::{ Vector3, Vector2 };
use cgmath::prelude::*;
use gl;

use crate::shader;
use shader::Shader;

// NOTE: without repr(C) the compiler may reorder the fields or use different padding/alignment than C.
// Depending on how you pass the data to OpenGL, this may be bad. In this case it's not strictly
// necessary though because of the `offset!` macro used below in setupMesh()
#[derive(Debug)]
#[repr(C)]
pub struct Vertex {
    // position
    pub Position: Vector3<f32>
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            Position: Vector3::zero()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Texture {
    pub id: u32,
    pub type_: String,
    pub path: String,
}

#[derive(Debug)]
pub struct Mesh {
    /*  Mesh Data  */
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub texture: Texture,
    // Vertex Array Object: contains pointers to VBOs
    pub VAO: u32,

    /*  Render data  */
    // Vertex Buffer Object: array of all the vertices
    VBO: u32,
    // Element Buffer Object: array with indices of vertices to draw for each triangle
    EBO: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, texture: Texture) -> Mesh {
        let mut mesh = Mesh {
            vertices, indices, texture,
            VAO: 0, VBO: 0, EBO: 0
        };

        // now that we have all the required data, set the vertex buffers and its attribute pointers.
        unsafe { mesh.setupMesh() }
        mesh
    }

    /// render the mesh
    pub unsafe fn Draw(&self, shader: &Shader) {
        //     gl::ActiveTexture(gl::TEXTURE0 as u32); // active proper texture unit before binding
        //     // retrieve texture number (the N in diffuse_textureN)
        //     let name = &texture.type_;
        //     // now set the sampler to the correct texture unit
        //     let sampler = CString::new(format!("{}{}", name, number)).unwrap();
        //     gl::Uniform1i(gl::GetUniformLocation(shader.ID, sampler.as_ptr()), i as i32);
        //     // and finally bind the texture
        //     gl::BindTexture(gl::TEXTURE_2D, texture.id);
        // }

        // draw mesh
        gl::BindVertexArray(self.VAO);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        gl::BindVertexArray(0);

        // always good practice to set everything back to defaults once configured.
        // gl::ActiveTexture(gl::TEXTURE0);
    }

    unsafe fn setupMesh(&mut self) {
        // create buffers/arrays
        gl::GenVertexArrays(1, &mut self.VAO);
        gl::GenBuffers(1, &mut self.VBO);
        gl::GenBuffers(1, &mut self.EBO);

        gl::BindVertexArray(self.VAO);
        // load data into vertex buffers
        gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);
        // A great thing about structs with repr(C) is that their memory layout is sequential for all its items.
        // The effect is that we can simply pass a pointer to the struct and it translates perfectly to a glm::vec3/2 array which
        // again translates to 3/2 floats which translates to a byte array.
        let size = (self.vertices.len() * size_of::<Vertex>()) as isize;
        let data = &self.vertices[0] as *const Vertex as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
        let size = (self.indices.len() * size_of::<u32>()) as isize;
        let data = &self.indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        // set the vertex attribute pointers
        let size = size_of::<Vertex>() as i32;
        // vertex Positions
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, Position) as *const c_void);

        gl::BindVertexArray(0);
    }
}
*/