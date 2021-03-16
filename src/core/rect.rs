use crate::core::color::Color;
use crate::transform::Transform;

/*
generic rectangle struct
it holds its shaderbuffer, its transform(position,dimensio) and its color
*/
pub struct Rect {
    shader_buffer: gl::types::GLuint,
    transform: Transform,
    color: Color,
}

impl Rect {
    // create a new rectangle, setting the shader buffer to uninitalised for now
    pub fn new(transform: Transform, color: Color) -> Rect {
        let shader_buffer = 0;
        Rect {shader_buffer, transform, color}
    }

    // create the shaderbuffer and the default shader
    pub fn create_shader_buffer(&mut self) {
        self.shader_buffer = crate::shader::create_default_shader_buffer(self.vertices());
    } 

    // call this function every frame to display the rectangle
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.shader_buffer);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }

    // draw only the borders of the rectangle
    pub fn draw_borders(&self) {
        unsafe {
            gl::BindVertexArray(self.shader_buffer);
            gl::DrawArrays(gl::LINE_STRIP, 0, 4);
        }
    }

    // calculate the vertices of the rectangle, used for creating the shader buffer
    fn vertices(&self) -> Vec<f32> {
        let bot_y = self.transform.y - self.transform.height;
        let right_x = self.transform.x + self.transform.width;

        vec![
            self.transform.x,  self.transform.y,  0.0,  self.color.r, self.color.g, self.color.b, // top left
            right_x,           self.transform.y,  0.0,  self.color.r, self.color.g, self.color.b, // top right
            right_x,           bot_y,             0.0,  self.color.r, self.color.g, self.color.b, // bot right
            self.transform.x,  bot_y,             0.0,  self.color.r, self.color.g, self.color.b, // bot left
        ]
    }
}