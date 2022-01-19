#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::c_void;
use std::path::Path;

use cgmath::{vec2, vec3};
use gl;
use image;
use image::DynamicImage::*;
use image::GenericImage;
use tobj;

use crate::mesh;
use mesh::{ Mesh, Texture, Vertex };
use crate::shader;
use shader::Shader;

#[derive(Debug)]
#[derive(Default)]
pub struct Model {
    /*  Model Data */
    pub meshes: Vec<Mesh>,
    pub textures_loaded: Vec<Texture>,   // stores all the textures loaded so far, optimization to make sure textures aren't loaded more than once.
    directory: String,
}

impl Model {
    /// constructor, expects a filepath to a 3D model.
    pub fn new(path: &str) -> Model {
        let mut model = Model::default();
        model.loadModel(path);
        model
    }

    pub fn Draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            unsafe { mesh.Draw(shader); }
        }
    }

    // loads a model from file and stores the resulting meshes in the meshes vector.
    fn loadModel(&mut self, path: &str) {
        let path = Path::new(path);

        // retrieve the directory path of the filepath
        self.directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
        let obj = tobj::load_obj(path);

        
        let (models, materials) = obj.unwrap();
        for model in models {
          let mesh = &model.mesh;
          let num_vertices = mesh.positions.len() / 3;
          
          // data to fill
          let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
          let indices: Vec<u32> = mesh.indices.clone();
          
          let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
          println!("hello {:?}", t);
          for i in 0..num_vertices {
                vertices.push(Vertex {
                    Position:  vec3(p[i*3], p[i*3+1], p[i*3+2])
                })
            }

            self.meshes.push(Mesh::new(vertices, indices, Texture {id: 0, type_: "t".to_string(), path: "r".to_string()}));
        }

    }

}

