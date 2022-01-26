use std::path::Path;
use std::os::raw::c_void;
use image;
use image::GenericImage;
use image::DynamicImage::*;

// TODO: add error management
pub unsafe fn load_texture(path: &str) -> u32 {
	let mut texture_id = 0;

	// load and create a texture
	// -------------------------
	gl::GenTextures(1, &mut texture_id);
	let img = image::open(&Path::new(path))
		.expect("Texture failed to load");

	// flip image vertically
    let img = img.flipv();
	
	// get image format
	let format = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
	};

	let data = img.raw_pixels();

	// all upcoming GL_TEXTURE_2D operations now have effect on this texture object
	gl::BindTexture(gl::TEXTURE_2D, texture_id);
	// create texture
	gl::TexImage2D(
		gl::TEXTURE_2D, 
		0, 
		format as i32, 
		img.width() as i32, 
		img.height() as i32,
		0, 
		format, 
		gl::UNSIGNED_BYTE, 
		&data[0] as *const u8 as *const c_void
	);

	// generate mipMap
	gl::GenerateMipmap(gl::TEXTURE_2D);

	// set the texture wrapping parameters
	// set texture wrapping to gl::REPEAT (default wrapping method)
	gl::TexParameteri(
		gl::TEXTURE_2D, 
		gl::TEXTURE_WRAP_S, 
		gl::REPEAT as i32
	);
	gl::TexParameteri(
		gl::TEXTURE_2D, 
		gl::TEXTURE_WRAP_T, 
		gl::REPEAT as i32
	);
	// set texture filtering parameters
	gl::TexParameteri(
		gl::TEXTURE_2D, 
		gl::TEXTURE_MIN_FILTER, 
		gl::LINEAR_MIPMAP_LINEAR as i32
	);   
	gl::TexParameteri(
		gl::TEXTURE_2D, 
		gl::TEXTURE_MAG_FILTER, 
		gl::LINEAR as i32
	);

	texture_id
}
