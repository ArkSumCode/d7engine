use crate::core::color::Color;
use crate::core::transform;
use crate::core::camera;
use crate::program;

/*
generic rectangle struct
it holds its shaderbuffer, its transform(position,dimensio) and its color
*/
pub struct Rect {
    shader_buffer: gl::types::GLuint,
    transform: transform::Transform,
    color: Color,
}

impl Rect {
    // create a new rectangle, setting the shader buffer to uninitalised for now
    pub fn new(transform: transform::Transform, color: Color) -> Rect {
        let shader_buffer = 0;
        Rect {shader_buffer, transform, color}
    }

    // create the shaderbuffer and the default shader
    pub fn create_shader_buffer(&mut self) {
        self.shader_buffer = crate::shader::create_default_shader_buffer(vec![
            0.0,  0.0,  0.0, 0.0
        ]);
    } 

    // call this function every frame to display the rectangle
    pub fn draw(&self, program: &program::Program, camera: &camera::Camera) {
        let pos = program.uniform_location("pos");
        let dim = program.uniform_location("dim");
        let cam = program.uniform_location("cam");
        let col = program.uniform_location("color");
       
        program.active();

        unsafe {
            gl::Uniform2f(cam, camera.width as f32, camera.height as f32);
            gl::Uniform2f(pos, self.transform.x as f32, self.transform.y as f32);
            gl::Uniform2f(dim, self.transform.width as f32, self.transform.height as f32);
            gl::Uniform4f(col, self.color.r as f32, self.color.g as f32, self.color.b as f32, self.color.a as f32);

            gl::BindVertexArray(self.shader_buffer);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }

    // draw only the borders of the rectangle
    pub fn draw_borders(&self, program: &program::Program, camera: &camera::Camera) {
        let pos = program.uniform_location("pos");
        let dim = program.uniform_location("dim");
        let cam = program.uniform_location("cam");
        let col = program.uniform_location("color");
       
        program.active();

        unsafe {
            gl::Uniform2f(cam, camera.width as f32, camera.height as f32);
            gl::Uniform2f(pos, self.transform.x as f32, self.transform.y as f32);
            gl::Uniform2f(dim, self.transform.width as f32, self.transform.height as f32);
            gl::Uniform4f(col, self.color.r as f32, self.color.g as f32, self.color.b as f32, self.color.a as f32);

            gl::BindVertexArray(self.shader_buffer);
            gl::DrawArrays(gl::LINE_STRIP, 0, 4);
        }
    }

    // set the position of the rect
    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.transform.set_pos(x, y);
    }
}