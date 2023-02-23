use crate::*;
use object::{VertexArray, Buffer, ObjectState, Object};

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330
    layout (location = 0) in vec2 position;
    layout (location = 1) in vec4 color;
    layout (location = 2) in vec2 offset;
    layout (location = 3) in vec2 scale;
    
    uniform mat4 projection;
    uniform mat4 view;
    uniform mat4 model;

    out vec4 oColor;
    out vec2 oPosition;

    void main() {
        vec2 scale_position = position * scale;
        vec2 offset_position = scale_position + offset;
        gl_Position = projection * view * model * vec4(offset_position, 0.0, 1.0);
        oColor = color;
        oPosition = position;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330
    in vec4 oColor;
    in vec2 oPosition;

    out vec4 color;

    void main() {
        float dist = distance(oPosition, vec2(0.5));
        float delta = fwidth(dist);
        float alpha = smoothstep(0.49-delta, 0.49, dist);
        color = mix(oColor, vec4(0.0), alpha);
    }
"#;

/*
A circle is an instanced 
implementation of a the shaders above
it holds all the data that is used for drawing circles to 
the screen

when circle gets dropped the shader also gets 
deletet from the graphics card
*/
type TransformData = [f32; 8];

pub struct Circle {
    program: Program,
    vertex_array: VertexArray,
    model_buffer: Buffer, // the buffer needs to stay alive
    transform_buffer: Buffer, // the buffer needs to stay alive
    transform_data: Vec<TransformData>,
    state: ObjectState,
}

impl Circle {
    // creates an empty Circle
    pub fn new() -> Self {
        Self {
            program: Program::default(),
            vertex_array: VertexArray::default(),
            model_buffer: Buffer::default(),
            transform_buffer: Buffer::default(),
            transform_data: vec![],
            state: ObjectState::Ok,
        }
    }
}

impl Object for Circle {
    // add an new Circle to the transform data
    fn add(&mut self, component_data: &ComponentData) {
        let color = component_data.color;
        let opacity = component_data.opacity;
        let (offset_x, offset_y) = component_data.offset;
        let (width, height) = component_data.dim;

        let transform_data: TransformData = [
            color.r, color.g, color.b, opacity, offset_x, offset_y, width, height, 
        ];

        self.transform_data.push(transform_data);
    }

    /// set the component data
    /// for a specific element 'i' of the transform data vector
    fn set(&mut self, i: usize, component_data: &ComponentData) {
        let color = component_data.color;
        let opacity = component_data.opacity;
        let (offset_x, offset_y) = component_data.offset;
        let (width, height) = component_data.dim;

        let transform_data: TransformData = [
            color.r, color.g, color.b, opacity, offset_x, offset_y, width, height, 
        ];

        self.transform_data[i] = transform_data;
    }

    // removes a circle from 
    // the transform data
    fn remove(&mut self, i: usize) {
        self.transform_data.remove(i);
    }

    // create shaders and buffers
    fn load(&mut self) -> Result<(), String> {
        let model_data: [f32; 4*2] = [
            1.0,  0.0,      // top right 0
            0.0,  0.0,      // top left 1
            0.0,  1.0,      // bottom left 2 
            1.0,  1.0,      // bottom right 3
        ];

        let transform_data = self.transform_data.concat();

        unsafe {
            // create the shaderprogram
            let vertex_shader = Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            self.program = Program::new(&vertex_shader, &fragment_shader)?;

            // create a new buffer for our vertex array (model + transform data)
            self.vertex_array = VertexArray::new();
            self.vertex_array.bind(); 
            
            // create a new buffer for our model data
            self.model_buffer = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
            self.model_buffer.set_data(&model_data.to_vec());
            // and create the attributes in the vertex shader
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 8, 0 as *const _); // position
            gl::EnableVertexAttribArray(0);
          
            // create a new buffer for our transform data
            self.transform_buffer = Buffer::new(gl::ARRAY_BUFFER, gl::DYNAMIC_DRAW);
            self.transform_buffer.set_data(&transform_data);
            // and create the attributes in the vertex shader
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 32, 0 as *const _); // color
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 32, 16 as *const _); // offset
            gl::VertexAttribPointer(3, 2, gl::FLOAT, gl::FALSE, 32, 24 as *const _); // scale
            gl::VertexAttribDivisor(1, 1);
            gl::VertexAttribDivisor(2, 1);
            gl::VertexAttribDivisor(3, 1);
            gl::EnableVertexAttribArray(1);
            gl::EnableVertexAttribArray(2);
            gl::EnableVertexAttribArray(3);
        }

        self.state = ObjectState::Ok;
        Ok(())
    }

    // resets the transformation data
    fn reload(&mut self) {
        let transform_data = self.transform_data.concat();
        self.transform_buffer.set_data(&transform_data);
        self.state = ObjectState::Ok;
    }

    // draw the circle to the screen
    fn draw(&mut self, draw: &Draw, camera: &Transform, model_transform: &Transform) -> Result<(), String> {
        // reset the transformation data if needed
        match self.state {
            ObjectState::Reload => self.reload(),
            ObjectState::Ok => (),
        }

        // create the mvp (model view projection) matrixes
        let projection = mvp::ortho(&draw.window);
        let view = camera.matrix();
        let model = model_transform.matrix();

        unsafe {
            // bind the programm and vertex array before sending
            // uniform and drawing
            self.program.bind();
            self.vertex_array.bind();
           
            // set the model view matrices
            let projection_location = self.program.get_uniform_location("projection")?;
            let view_location = self.program.get_uniform_location("view")?;
            let model_location = self.program.get_uniform_location("model")?;
            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection.as_ptr());
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model.as_ptr());
            gl::DrawArraysInstanced(gl::TRIANGLE_FAN, 0, 4, self.transform_data.len() as i32);
        }

        Ok(())
    }

    // set the state of the Object
    fn set_state(&mut self, object_state: ObjectState) {
        self.state = object_state;
    }
}