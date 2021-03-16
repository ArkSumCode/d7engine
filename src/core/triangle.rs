use crate::core::color::Color;
use crate::transform::Transform;

/*
generic triangle struct
it holds its shaderbuffer, its transform(position,dimensio) and its color
*/
pub struct Triangle {
    shader_buffer: gl::types::GLuint,
    transform: Transform,
    color: Color,
}

impl Triangle {
    // create a new triangle, setting the shader buffer to uninitalised for now
    pub fn new(transform: Transform, color: Color) -> Triangle {
        let shader_buffer = 0;
        Triangle {shader_buffer, transform, color}
    }

    // create the shaderbuffer for this shape and the default shader
    pub fn create_shader_buffer(&mut self) {
        self.shader_buffer = crate::shader::create_default_shader_buffer(self.vertices());
    } 

    // call this function every frame to display the triangle
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.shader_buffer);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    // calculate the vertices of the triangle, used for creating the shader buffer
    fn vertices(&self) -> Vec<f32> {
        let half_width = self.transform.width / 2.0;
        let bot_y = self.transform.y - self.transform.height;
        vec![
            self.transform.x + half_width,            self.transform.y,  0.0,  self.color.r, self.color.g, self.color.b, // top middle
            self.transform.x + self.transform.width,  bot_y,             0.0,  self.color.r, self.color.g, self.color.b, // bot right
            self.transform.x,                         bot_y,             0.0,  self.color.r, self.color.g, self.color.b, // bot left
        ]
    }
}