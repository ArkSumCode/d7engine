use crate::core::*;
use crate::program;

/*
struct that can draw a image/texture to the window
it holds the shader buffer, a shader texture buffer
and its transform in the world
*/
pub struct Texture {
    shader_buffer: gl::types::GLuint,
    shader_texture_buffer: gl::types::GLuint,
    transform: transform::Transform,
    color: Option<color::Color>,
}

impl Texture {
    // creates a new texture object with uninitialies buffers
    pub fn new(transform: transform::Transform) -> Texture {
        let shader_buffer = 0;
        let shader_texture_buffer = 0;
        Texture {shader_buffer, shader_texture_buffer, transform, color: None}
    }

    pub fn colored(transform: transform::Transform, color: &color::Color) -> Texture {
        let shader_buffer = 0;
        let shader_texture_buffer = 0;
        Texture {shader_buffer, shader_texture_buffer, transform, color: Some(color::Color::copy(color))}
    }

    // create the shader buffer, for that we need the image
    pub fn create_shader_buffer(&mut self, image: &image::RgbaImage) {
        let (shader_texture_buffer, shader_buffer) = crate::shader::create_texture_shader_buffer(
            self.vertices(), 
            image
        );
        self.shader_texture_buffer = shader_texture_buffer;
        self.shader_buffer = shader_buffer;
    } 

    // draw the texture to the screen
    pub fn draw(&self, program: &program::Program, camera: &camera::Camera) {
        let pos = program.uniform_location("pos");
        let dim = program.uniform_location("dim");
        let cam = program.uniform_location("cam");

        // programs like font can use the uniform color
        // use the colored constructor to get one
        let col = if let Some(_) = &self.color {
            Some(program.uniform_location("color"))
        } else {
            None
        };
       
        program.active();

        unsafe {
            // set values for the uniforms in the shader
            gl::Uniform2f(cam, camera.width, camera.height);
            gl::Uniform2f(pos, self.transform.x, self.transform.y);
            gl::Uniform2f(dim, self.transform.width, self.transform.height);

            if let Some(color) = &self.color {
                if let Some(c) = col {
                    gl::Uniform4f(c, color.r as f32, color.g as f32, color.b as f32, color.a as f32);
                }
            }
           
            gl::BindTexture(gl::TEXTURE_2D, self.shader_texture_buffer);
            gl::BindVertexArray(self.shader_buffer);

            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }

    // get the vertices passed to the program
    fn vertices(&self) -> Vec<f32> {
        vec![
            0.0, 1.0, // top left
            1.0, 1.0, // top right
            1.0, 0.0, // bot right
            0.0, 0.0, // bot left
        ]
    }

    // set the position of the texture
    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.transform.set_pos(x, y);
    }
}

// get data from an image file
pub fn image_data(path: &str) -> Result<image::RgbaImage, String> {
    if let Ok(data) = image::open(path) {
        // need to flip because opengl starts bottom left
        let flipped = data.flipv();
        return Ok(flipped.to_rgba8())
    }
  
    Err(format!("could not open image '{}'", path))
}

// crop an image out of another image
pub fn crop_image(image: &mut image::RgbImage, x: i32, y: i32, width: i32, height: i32) -> image::RgbImage {
    let img = image::imageops::crop(image, x as u32, y as u32, width as u32, height as u32);
    img.to_image()
}