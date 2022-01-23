use crate::prelude::*;

/*
generic rectangle struct
it holds its shaderbuffer, its transform(position,dimensio) and its color
*/
pub struct Rect {
    shader_buffer: gl::types::GLuint,
    color: Color,
    transform: Transform,
    width: f32,
    height: f32,
}

impl Rect {
    // create a new rectangle, setting the shader buffer to uninitalised for now
    pub fn new(width: f32, height: f32, color: &Color) -> Rect {
        // create the shaderbuffer and the default shader
        let shader_buffer = crate::shader::create_default_shader_buffer(Rect::vertices(width, height));
        // create the default transform
        let transform = Transform::new();
        Rect {shader_buffer, color: *color, width, height, transform}
    }

    // call this function every frame to display the rectangle
    pub fn draw(&self, draw: &Draw, camera: &Transform) {
        let program = draw.shaders.get("rect").unwrap();
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

            gl::BindVertexArray(self.shader_buffer);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }

    // returns the width of the rect
    pub fn width(&self) -> f32 {
        self.width
    }

    // returns the height of the rect
    pub fn height(&self) -> f32 {
        self.height
    }

    // returns the color of the rect
    pub fn color(&self) -> Color {
        self.color
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
    // first 3 values position
    fn vertices(width: f32, height: f32) -> Vec<f32> {
        vec![
            0.0,   height, 0.0, // top left
            width, height, 0.0, // top right
            width,    0.0, 0.0, // bot right
            0.0,      0.0, 0.0, // bot left
        ]
    }
}