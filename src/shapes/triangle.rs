use crate::color::Color;
use crate::shapes::Transform;

pub struct Triangle {
    vao: gl::types::GLuint,
    transform: Transform,
    color: Color,
}

impl Triangle {
    pub fn new(transform: Transform, color: Color) -> Triangle {
        let vao = 0;
        Triangle {vao, transform, color}
    }

    pub fn create_shader_buffer(&mut self) {
        self.vao = crate::shader::create_default_shader_buffer(self.vertices());
    } 

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    fn vertices(&self) -> Vec<f32> {
        let half_width = self.transform.width / 2.0;
        let bot_y = self.transform.y + self.transform.height;
        vec![
            self.transform.x + half_width,            self.transform.y,  0.0,  self.color.r, self.color.g, self.color.b, // top middle
            self.transform.x + self.transform.width,  bot_y,             0.0,  self.color.r, self.color.g, self.color.b, // bot right
            self.transform.x,                         bot_y,             0.0,  self.color.r, self.color.g, self.color.b, // bot left
        ]
    }
}