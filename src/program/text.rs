use crate::prelude::*;

/*
Text holds the text of the Text
its transform
and the vector of all the chars textures
*/
pub struct Text {
    shader_buffer: gl::types::GLuint,
    shader_texture_buffer: gl::types::GLuint,
    transform: Transform,
    color: Color,
    width: u32,
    height: u32,
}

impl Text {
    // creates a new text, using the font a position(transform), the text, the font size and color
    pub fn new(font: &Font, text: &str, font_size: u32, color: &Color) -> Result<Text, String> {
        // create the text as rgba image
        let image = font.snapshot(text, font_size as f32)?;

        // create the shader buffer, for that we need the image and the vertices
        let (shader_texture_buffer, shader_buffer) = crate::shader::create_texture_shader_buffer(
            Text::vertices(&image), 
            &image
        );

        // create the default transform
        let transform = Transform::new();
  
        Ok(Text {shader_buffer, shader_texture_buffer, transform, color: *color, width: image.width(), height: image.height()})
    }

    // draw the text' rendered image
    pub fn draw(&self, draw: &Draw, camera: &Transform) {
        let program = draw.shaders.get("text").unwrap();
        program.active();

        // get all location references of the uniforms
        let projection_location = program.uniform_location("projection");
        let view_location = program.uniform_location("view");
        let model_location = program.uniform_location("model");
        let color_location = program.uniform_location("color");

        // create the mvp (model view projection) matrixes
        let projection = mvp::ortho(&draw.window);
        let view = camera.matrix();
        let model = self.transform.matrix();

        unsafe {
            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection.as_ptr());
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model.as_ptr());
            gl::Uniform3f(color_location, self.color.r, self.color.g, self.color.b);

            gl::BindTexture(gl::TEXTURE_2D, self.shader_texture_buffer);
            gl::BindVertexArray(self.shader_buffer);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }

    // sets the x, y, z position 
    // on the screen
    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.transform.set(x, y, z);
    }

    // sets the color of the text
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    // get the vertices passed to the program
    // first 3 values position, second two are the texture coords
    fn vertices(image: &image::RgbaImage) -> Vec<f32> {
        let height = image.height() as f32;
        let width = image.width() as f32;
        vec![
            0.0,   height, 0.0, 0.0, 1.0, // top left
            width, height, 0.0, 1.0, 1.0, // top right
            width,    0.0, 0.0, 1.0, 0.0, // bot right
            0.0,      0.0, 0.0, 0.0, 0.0, // bot left
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
}