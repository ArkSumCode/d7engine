use crate::prelude::*;

/*
generic rectangle struct
it holds its shaderbuffer, its transform(position,dimensio) and its color
*/
pub struct Rect {
    shader_buffer: gl::types::GLuint,
    color: Color,
}

impl Rect {
    // create a new rectangle, setting the shader buffer to uninitalised for now
    pub fn new(color: Color) -> Rect {
        let shader_buffer = 0;
        Rect {shader_buffer, color}
    }

    // create the shaderbuffer and the default shader
    pub fn create_shader_buffer(&mut self) {
        self.shader_buffer = crate::shader::create_default_shader_buffer(vec![
            0.0,  0.0,  0.0, 0.0
        ]);
    } 

    // call this function every frame to display the rectangle
    pub fn draw(&self, program: &program::Program) {
        let col = program.uniform_location("color");
       
        program.active();

        unsafe {
            gl::Uniform4f(col, self.color.r as f32, self.color.g as f32, self.color.b as f32, self.color.a as f32);

            gl::BindVertexArray(self.shader_buffer);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }

    // draw only the borders of the rectangle
    pub fn draw_borders(&self, program: &program::Program) {
        let col = program.uniform_location("color");
       
        program.active();

        unsafe {
            gl::Uniform4f(col, self.color.r as f32, self.color.g as f32, self.color.b as f32, self.color.a as f32);

            gl::BindVertexArray(self.shader_buffer);
            gl::DrawArrays(gl::LINE_STRIP, 0, 4);
        }
    }
}