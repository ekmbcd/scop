use std::{path::Path, io, fs::File};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufRead::lines(io::BufReader::new(file)))
}

pub unsafe fn load_model(path: &str) -> (Vec<f32>, Vec<u32>) {
    let path = Path::new(path);

    let mut positions: Vec<f32> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(obj_string) = line {
                // vertex line (3 floats)
                if obj_string.starts_with("v ") {
                    obj_string.split(" ")
                        .for_each(|x| {
                            if let Ok(y) = x.parse::<f32>() {
                                positions.push(y);
                            }
                        });
                // face line (at least 3 ints, may have '/' used for normals (we ignore them))
                } else if obj_string.starts_with("f ") {
                    let mut polygon = Vec::new();
                    obj_string.split(" ")
                        .for_each(|x| {
                            let test = x.split("/").next().expect("obj is wrong");
                            if let Ok(y) = test.parse::<u32>() {
                                polygon.push(y);
                            }
                        });
                    indices.append(&mut to_triangles(polygon));
                // ignored lines
                } else if obj_string.starts_with("#") || 
                    obj_string.starts_with("vt ") ||
                    obj_string.starts_with("vn ") ||
                    obj_string.starts_with("usemtl ") ||
                    obj_string.starts_with("s ") ||
                    obj_string.starts_with("mtllib ") ||
                    obj_string.starts_with("o ") {
                        // skip
                } else {
                    panic!("Wrong obj file");
                }

            }
        }
    }
    (positions, indices)
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
    // println!("{:?} - {:?}", polygon, out);
    out
}
