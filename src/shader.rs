use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use gl;
use gl::types::*;

use crate::math::{Matrix4, Vector4};
// use cgmath::prelude::*;

pub struct Shader {
    pub id: u32,
}

/// NOTE: mixture of `shader_s.h` and `shader_m.h` (the latter just contains
/// a few more setters for uniforms)
impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let mut shader = Shader { id: 0 };
        // 1. retrieve the vertex/fragment source code from filesystem
        let mut v_shader_file = File::open(vertex_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", vertex_path));
        let mut f_shader_file = File::open(fragment_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragment_path));
        let mut vertex_code = String::new();
        let mut fragment_code = String::new();
        v_shader_file
            .read_to_string(&mut vertex_code)
            .expect("Failed to read vertex shader");
        f_shader_file
            .read_to_string(&mut fragment_code)
            .expect("Failed to read fragment shader");

        let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
        let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX");
            // fragment Shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT");
            // shader Program
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id;
        }

        shader
    }

    /// activate the shader
    /// ------------------------------------------------------------------------
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id)
    }

    /// utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn setBool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setInt(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setFloat(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    // pub unsafe fn setVector4(&self, name: &CStr, value: &Vector4) {
    //     gl::Uniform3fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, value.as_ptr());
    // }
    /// ------------------------------------------------------------------------
    pub unsafe fn setVec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setMat4(&self, name: &CStr, mat: &Matrix4) {
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, gl::FALSE, mat.as_ptr());
    }

    /// utility function for checking shader compilation/linking errors.
    /// ------------------------------------------------------------------------
    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut infoLog = vec!(0; 1024);
        infoLog.set_len(1024 - 1); // subtract 1 to skip the trailing null character

        // check shader compilation
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                // if compilation failed, get log
                gl::GetShaderInfoLog(
                    shader, 
                    1024, 
                    ptr::null_mut(), 
                    infoLog.as_mut_ptr() as *mut GLchar
                );
                println!(
                    "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n",
                    type_,
                    str::from_utf8(&infoLog).unwrap()
                );
                // TODO: manage error (?)
                panic!();
            }
        // check program linking
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    shader, 
                    1024, 
                    ptr::null_mut(), 
                    infoLog.as_mut_ptr() as *mut GLchar
                );
                println!(
                    "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n",
                    type_,
                    str::from_utf8(&infoLog).unwrap()
                );
                // TODO: manage error (?)
                panic!();
            }
        }
    }
}
