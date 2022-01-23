use std::{path::Path, io, fs::File};
use cgmath::Matrix4;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufRead::lines(io::BufReader::new(file)))
}

pub unsafe fn load_model(path: &str) -> (Vec<f32>, Vec<u32>) {
    let path = Path::new(path);

    let obj = tobj::load_obj(path);

    
    let (models, materials) = obj.unwrap();
    // for model in models {
        let model = &models[0];
        let mesh = &model.mesh;
        let num_vertices = mesh.positions.len() / 3;
        
        // data to fill
        // let mut vertices: Vec<Vector3<f32>> = Vec::with_capacity(num_vertices);
        let indices: Vec<u32> = mesh.indices.clone();
        
        let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
        println!("positions {}", p.len());
        println!("indices {:?} {}", indices, indices.len());

        let mut mypos: Vec<f32> = Vec::new();
        let mut my_indices: Vec<u32> = Vec::new();

        if let Ok(lines) = read_lines(path) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    if ip.starts_with("v ") {
                        ip.split(" ")
                            .for_each(|x| {
                                if let Ok(lmao) = x.parse::<f32>() {
                                    mypos.push(lmao);
                                }
                            });
                        // println!("> {}", ip);
                    } else if ip.starts_with("f ") {
                        let mut polygon = Vec::new();
                        ip.split(" ")
                            .for_each(|x| {
                                let test = x.split("/").next().expect("obj is wrong");
                                if let Ok(lmao) = test.parse::<u32>() {
                                    polygon.push(lmao);
                                }
                            });
                        my_indices.append(&mut to_triangles(polygon));
                        // println!("> {}", ip);
                    }
                }
            }
        }
        println!("myindices {} {:?}",  my_indices.len(), my_indices);


        // let mut m = 2;
        // for n in 0..p.len() {
        // 	if p[n] != mypos[n] {
        // 		println!("{} - {} {}", n, p[n], mypos[n])
        // 	}
        // }
        // println!("mypositions {}",  mypos.len());

        let vertices: [f32; 24] = [
                        // positions         // colors
                        0.5,  0.5, 0.0,  0.0, 0.0, 1.0,  // top right
                        0.5, -0.5, 0.0,  1.0, 0.0, 0.0,  // bottom right
                     -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  // bottom left
                     -0.5,  0.5, 0.0,  1.0, 0.0, 0.0   // top left
                ];
                let indices = [ // note that we start from 0!
                        0, 1, 3,  // first Triangle
                        1, 2, 3   // second Triangle
                ];


    //   for i in 0..num_vertices {
    //         vertices.push(Vertex {
    //             Position:  vec3(p[i*3], p[i*3+1], p[i*3+2])
    //         })
    //     }

    //     self.meshes.push(Mesh::new(vertices, indices, Texture {id: 0, type_: "t".to_string(), path: "r".to_string()}));
    // }
    // (mesh.positions.clone(), mesh.indices.clone())
    let scale_matrix = scale_vertices(&mypos);
    println!("{:?}", scale_matrix);
    (mypos, my_indices)
}

fn to_triangles(polygon: Vec<u32>) -> Vec<u32> {
    let mut out = Vec::<u32>::new();

    let mut len = polygon.len();

    if len < 3 {
        panic!("obj file is wrong!")
    }

    while len >= 3 {
        out.push(polygon[0] - 1);
        out.push(polygon[polygon.len() - len + 1] - 1);
        out.push(polygon[polygon.len() - len + 2] - 1);

        len -= 1;
    }
    println!("{:?} - {:?}", polygon, out);
    out
}

fn center_vertices(vertices: &Vec<f32>) -> Vec<f32>{
    let mut i = 0;
    let (mut max_x, mut max_y, mut max_z) = (f32::MIN, f32::MIN, f32::MIN);
    let (mut min_x, mut min_y, mut min_z) = (f32::MAX, f32::MAX, f32::MAX);

    while i < vertices.len() {
        if vertices[i] > max_x {
            max_x = vertices[i];
        }
        if vertices[i] < min_x {
            min_x = vertices[i];
        }

        if vertices[i + 1] > max_y {
            max_y = vertices[i + 1];
        }
        if vertices[i + 1] < min_y {
            min_y = vertices[i + 1];
        }

        if vertices[i + 2] > max_z {
            max_z = vertices[i + 2];
        }
        if vertices[i + 2] < min_z {
            min_z = vertices[i + 2];
        }

        i+=3;
    }

    let mut translation = Vec::new();
    translation.push((max_x - min_x) / 2.0);
    translation.push((max_y - min_y) / 2.0);
    translation.push((max_z - min_z) / 2.0);
    
    translation
}

fn scale_vertices(vertices: &Vec<f32>) -> Matrix4<f32>{
    let abs_max = vertices.iter()
        .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
        .expect("fail in abs_max");

    println!("MAX = {}", abs_max);
    
    Matrix4::from_scale(1.0 / (*abs_max))
}