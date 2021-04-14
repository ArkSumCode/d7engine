use crate::core::color::Color;
use crate::core::transform;
use crate::core::camera;

/*
generic triangle struct
it holds its shaderbuffer, its transform(position,dimensio) and its color
*/
pub struct Triangle {
    shader_buffer: gl::types::GLuint,
    transform: transform::Transform,
    color: Color,
}

impl Triangle {
    // create a new triangle, setting the shader buffer to uninitalised for now
    pub fn new(transform: transform::Transform, color: Color) -> Triangle {
        let shader_buffer = 0;
        Triangle {shader_buffer, transform, color}
    }

    // create the shaderbuffer for this shape and the default shader
    pub fn create_shader_buffer(&mut self, camera: &camera::Camera) {
        self.shader_buffer = crate::shader::create_default_shader_buffer(self.vertices(camera));
    } 

    // call this function every frame to display the triangle
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.shader_buffer);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    // calculate the vertices of the triangle, used for creating the shader buffer
    fn vertices(&self, camera: &camera::Camera) -> Vec<f32> {
        let ct = transform::CanvasTransform::new(&self.transform, camera);
        let half_width = ct.width / 2.0;
        let bot_y = ct.y - ct.height;
        vec![
            ct.x + half_width,            ct.y,  0.0,  self.color.r, self.color.g, self.color.b, // top middle
            ct.x + ct.width,  bot_y,             0.0,  self.color.r, self.color.g, self.color.b, // bot right
            ct.x,             bot_y,             0.0,  self.color.r, self.color.g, self.color.b, // bot left
        ]
    }
}