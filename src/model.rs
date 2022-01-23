// use cgmath::Matrix4;
use crate::math::Matrix4;

pub fn generate_model_matrix(vertices: &Vec<f32>) -> Matrix4{
    let mut i = 0;
    let (mut max_x, mut max_y, mut max_z) = (f32::MIN, f32::MIN, f32::MIN);
    let (mut min_x, mut min_y, mut min_z) = (f32::MAX, f32::MAX, f32::MAX);

    while i < vertices.len() {
        // X
        if vertices[i] > max_x {
            max_x = vertices[i];
        }
        if vertices[i] < min_x {
            min_x = vertices[i];
        }
        // Y
        if vertices[i + 1] > max_y {
            max_y = vertices[i + 1];
        }
        if vertices[i + 1] < min_y {
            min_y = vertices[i + 1];
        }
        // Z
        if vertices[i + 2] > max_z {
            max_z = vertices[i + 2];
        }
        if vertices[i + 2] < min_z {
            min_z = vertices[i + 2];
        }

        i+=3;
    }

    let mut scale_vec = Vec::new();
    scale_vec.push((max_x - min_x) / 2.0);
    scale_vec.push((max_y - min_y) / 2.0);
    scale_vec.push((max_z - min_z) / 2.0);

    // find the longest side of the object
    let abs_max = scale_vec.iter()
        .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
        .expect("fail in abs_max");

    let scale_matrix = Matrix4::from_scale(1.0 / (*abs_max));
    // let myscale_matrix = math::Matrix4::from_scale(1.0 / (*abs_max));
    
    println!("CG trans {:?}", scale_matrix);
    // println!("MY trans {:?}", myscale_matrix);
    // used to center the object
    // let translation = cgmath::Vector3::new(-(max_x + min_x) / 2.0, -(max_y + min_y) / 2.0, -(max_z + min_z) / 2.0);
    
    // let translation_matrix = Matrix4::from_translation(translation);
    let translation_matrix = Matrix4::from_translation(
        -(max_x + min_x) / 2.0, 
        -(max_y + min_y) / 2.0, 
        -(max_z + min_z) / 2.0
    );

    scale_matrix * translation_matrix
}
