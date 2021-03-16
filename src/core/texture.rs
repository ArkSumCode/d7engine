use crate::transform::Transform;

/*
struct that can draw a image/texture to the window
it holds the shader buffer, a shader texture buffer
and its transform in the world
*/
pub struct Texture {
    shader_buffer: gl::types::GLuint,
    shader_texture_buffer: gl::types::GLuint,
    transform: Transform,
}

impl Texture {
    // creates a new texture object with uninitialies buffers
    pub fn new(transform: Transform) -> Texture {
        let shader_buffer = 0;
        let shader_texture_buffer = 0;
        Texture {shader_buffer, shader_texture_buffer, transform}
    }

    // create the shader buffer, for that we need the image
    pub fn create_shader_buffer(&mut self, image: &image::RgbImage) {
        let (shader_texture_buffer, shader_buffer) = crate::shader::create_texture_shader_buffer(
            self.vertices(), 
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
    fn vertices(&self) -> Vec<f32> {
        let bot_y = self.transform.y - self.transform.height;
        let right_x = self.transform.x + self.transform.width;

        vec![
            self.transform.x,  self.transform.y,  0.0,  0.0, 1.0, // top left
            right_x,           self.transform.y,  0.0,  1.0, 1.0, // top right
            right_x,           bot_y,             0.0,  1.0, 0.0, // bot right
            self.transform.x,  bot_y,             0.0,  0.0, 0.0, // bot left
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