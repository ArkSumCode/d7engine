use crate::core::*;

/*
struct that can draw a image/texture to the window
it holds the shader buffer, a shader texture buffer
and its transform in the world
*/
pub struct Texture {
    shader_buffer: gl::types::GLuint,
    shader_texture_buffer: gl::types::GLuint,
    transform: transform::Transform,
}

impl Texture {
    // creates a new texture object with uninitialies buffers
    pub fn new(transform: transform::Transform) -> Texture {
        let shader_buffer = 0;
        let shader_texture_buffer = 0;
        Texture {shader_buffer, shader_texture_buffer, transform}
    }

    // create the shader buffer, for that we need the image
    pub fn create_shader_buffer(&mut self, image: &image::RgbImage, camera: &camera::Camera) {
        let (shader_texture_buffer, shader_buffer) = crate::shader::create_texture_shader_buffer(
            self.vertices(camera), 
            image
        );
        self.shader_texture_buffer = shader_texture_buffer;
        self.shader_buffer = shader_buffer;
    } 

    // create a shader buffer with a pos, texture and color attribute 
    pub fn create_colored_shader_buffer(&mut self, image: &image::RgbImage, camera: &camera::Camera, color: &color::Color) {
        let (shader_texture_buffer, shader_buffer) = crate::shader::create_colored_texture_shader_buffer(
            self.colored_vertices(camera, color), 
            image
        );
        self.shader_texture_buffer = shader_texture_buffer;
        self.shader_buffer = shader_buffer;
    }

    // draw the texture to the screen
    pub fn draw(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.shader_texture_buffer);
            gl::BindVertexArray(self.shader_buffer);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }

    // get the vertices passed to the program
    fn vertices(&self, camera: &camera::Camera) -> Vec<f32> {
        let ct = transform::CanvasTransform::new(&self.transform, camera);
        let bot_y = ct.y - ct.height;
        let right_x = ct.x + ct.width;

        vec![
            ct.x,     ct.y,    0.0,  0.0, 1.0, // top left
            right_x,  ct.y,    0.0,  1.0, 1.0, // top right
            right_x,  bot_y,   0.0,  1.0, 0.0, // bot right
            ct.x,     bot_y,   0.0,  0.0, 0.0, // bot left
        ]
    }

    // get the vertices for the program with pos vec3 textCoord vec2 and color vec3
    fn colored_vertices(&self, camera: &camera::Camera, color: &color::Color) -> Vec<f32> {
        let ct = transform::CanvasTransform::new(&self.transform, camera);
        let bot_y = ct.y - ct.height;
        let right_x = ct.x + ct.width;

        vec![
            ct.x,     ct.y,    0.0,  0.0, 1.0, color.r, color.g, color.b, // top left
            right_x,  ct.y,    0.0,  1.0, 1.0, color.r, color.g, color.b, // top right
            right_x,  bot_y,   0.0,  1.0, 0.0, color.r, color.g, color.b, // bot right
            ct.x,     bot_y,   0.0,  0.0, 0.0, color.r, color.g, color.b, // bot left
        ]
    }

}

// get data from an image file
pub fn image_data(path: &str) -> Result<image::RgbImage, String> {
    if let Ok(data) = image::open(path) {
        // need to flip because opengl starts bottom left
        let flipped = data.flipv();
        return Ok(flipped.to_rgb8())
    }
  
    Err(format!("could not open image '{}'", path))
}

// crop an image out of another image
pub fn crop_image(image: &mut image::RgbImage, x: i32, y: i32, width: i32, height: i32) -> image::RgbImage {
    let img = image::imageops::crop(image, x as u32, y as u32, width as u32, height as u32);
    img.to_image()
}