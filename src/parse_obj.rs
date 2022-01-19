use std::{path::Path, io, fs::File};

use cgmath::Vector3;


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
		let mut myIndices: Vec<i32> = Vec::new();

		if let Ok(lines) = read_lines("resources/objects/teapot/teapot2.obj") {
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
					}
				}
			}
		}

		let mut m = 2;
		for n in 0..p.len() {
			if p[n] != mypos[n] {
				println!("{} - {} {}", n, p[n], mypos[n])
			}
		}
		println!("mypositions {}",  mypos.len());

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
	(mesh.positions.clone(), mesh.indices.clone())
	// (mypos, mesh.indices.clone())
}