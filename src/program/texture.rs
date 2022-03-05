use crate::prelude::*;

/*
struct that can draw a image/texture to the window
it holds the shader buffer, a shader texture buffer
and its transform in the world
*/
pub struct Texture {
    pub transform: Transform,
    shader_buffer: gl::types::GLuint,
    shader_texture_buffer: gl::types::GLuint,
    width: u32,
    height: u32,
}

impl Texture {
    // creates a new texture object
    pub fn new(image: &Image) -> Texture {
        let image = image.to_rgba_image();
        // create the shader buffer, for that we need the image and the vertices
        let (shader_texture_buffer, shader_buffer) = crate::shader::create_texture_shader_buffer(
            Texture::vertices(&image), 
            &image,
        );

        // create the default transform 
        let transform = Transform::new();

        Texture {
            shader_buffer, shader_texture_buffer, 
            transform, width: image.width(), height: image.height()
        }
    }

    // draws the texture to the screen
    pub fn draw(&self, draw: &Draw, camera: &Transform) {
        let program = draw.shaders.get("texture").unwrap();
        program.active();

        // get all location references of the uniforms
        let projection_location = program.uniform_location("projection");
        let view_location = program.uniform_location("view");
        let model_location = program.uniform_location("model");

        // create the mvp (model view projection) matrixes
        let projection = mvp::ortho(&draw.window);
        let view = camera.matrix();
        let model = self.transform.matrix();

        unsafe {
            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection.as_ptr());
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model.as_ptr());

            gl::BindTexture(gl::TEXTURE_2D, self.shader_texture_buffer);
            gl::BindVertexArray(self.shader_buffer);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }

    // get the vertices passed to the program
    // first 3 values position, second two are the texture coords
    fn vertices(image: &image::RgbaImage) -> Vec<f32> {
        let height = image.height() as f32;
        let width = image.width() as f32;
        vec![
            0.0,   height, 0.0, 0.0, 0.0, // top left
            width, height, 0.0, 1.0, 0.0, // top right
            width,    0.0, 0.0, 1.0, 1.0, // bot right
            0.0,      0.0, 0.0, 0.0, 1.0, // bot left
        ]
    }

    // returns the width of the rgba image
    pub fn width(&self) -> u32 {
        self.width
    } 

    // returns the height of the rgba image
    pub fn height(&self) -> u32 {
        self.height
    }
    
    // returns if a point collides with the 
    // texture in a 2d space
    pub fn collides(&self, x: f32, y: f32) -> bool {
        let (x2, y2, _) = self.transform.pos();
        collision::point_in_rect(x, y, x2, y2, self.width() as f32, self.height() as f32)
    }
}