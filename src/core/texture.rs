use crate::prelude::*;

/*
struct that can draw a image/texture to the window
it holds the shader buffer, a shader texture buffer
and its transform in the world
*/
pub struct Texture {
    shader_buffer: gl::types::GLuint,
    shader_texture_buffer: gl::types::GLuint,
    transform: Transform,
    color: Option<Color>,
}

impl Texture {
    // creates a new texture object with uninitialies buffers
    pub fn new(transform: Transform) -> Texture {
        let shader_buffer = 0;
        let shader_texture_buffer = 0;
        Texture {shader_buffer, shader_texture_buffer, transform, color: None}
    }

    pub fn colored(transform: Transform, color: &Color) -> Texture {
        let shader_buffer = 0;
        let shader_texture_buffer = 0;
        Texture {shader_buffer, shader_texture_buffer, transform, color: Some(Color::copy(color))}
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

    // draws the texture with a specific program
    pub fn draw_texture(&self, _draw: &Draw, program: &Program) {
        program.active();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.shader_texture_buffer);
            gl::BindVertexArray(self.shader_buffer);

            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }

    // draw the font to the screen
    pub fn draw_font(&self, draw: &Draw) {
        let program = draw.shaders.get("font").unwrap();
        self.draw_texture(draw, &program);
    }

    // draw the texture to the screen, this is only for better 
    // use of the method
    pub fn draw(&self, draw: &Draw) {
        let program = draw.shaders.get("texture").unwrap();
        self.draw_texture(draw, &program);
    }

    // get the vertices passed to the program
    // first 3 values position, second two are the texture coords
    fn vertices(&self) -> Vec<f32> {
        vec![
            -0.5,  0.5, 0.0, 0.0, 1.0, // top left
             0.5,  0.5, 0.0, 1.0, 1.0, // top right
             0.5, -0.5, 0.0, 1.0, 0.0, // bot right
            -0.5, -0.5, 0.0, 0.0, 0.0, // bot left
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