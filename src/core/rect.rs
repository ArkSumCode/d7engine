use crate::core::color::Color;
use crate::core::transform;
use crate::core::camera;

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
    pub fn create_shader_buffer(&mut self, camera: &camera::Camera) {
        self.shader_buffer = crate::shader::create_default_shader_buffer(self.vertices(camera));
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
    fn vertices(&self, camera: &camera::Camera) -> Vec<f32> {
        let ct = transform::CanvasTransform::new(&self.transform, camera);
        let bot_y = ct.y - ct.height;
        let right_x = ct.x + ct.width;

        vec![
            ct.x,  ct.y,  0.0,  self.color.r, self.color.g, self.color.b, // top left
            right_x,           ct.y,  0.0,  self.color.r, self.color.g, self.color.b, // top right
            right_x,           bot_y,             0.0,  self.color.r, self.color.g, self.color.b, // bot right
            ct.x,  bot_y,             0.0,  self.color.r, self.color.g, self.color.b, // bot left
        ]
    }
}