use crate::prelude::*;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330
    layout (location = 0) in vec2 position;
    layout (location = 1) in vec2 texcoord;
    layout (location = 2) in vec2 offset;
    layout (location = 3) in vec2 scale;
    
    uniform mat4 projection;
    uniform mat4 view;
    uniform mat4 model;

    out vec2 oTexCoord;

    void main() {
        vec2 scale_position = position * scale;
        vec2 offset_position = scale_position + offset;
        gl_Position = projection * view * model * vec4(offset_position, 0.0, 1.0);
        oTexCoord = texcoord;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330

    uniform sampler2D sampler;

    in vec2 oTexCoord;

    out vec4 color;

    void main() {
        color = texture(sampler, oTexCoord);
    }
"#;

type TransformData = [f32; 4];

pub struct Texture {
    pub transform: Transform,
    program: Program,
    vertex_array: VertexArray,
    model_buffer: Buffer, // the buffer needs to stay alive
    texture_buffer: TextureBuffer, // the buffer needs to stay alive
    transform_buffer: Buffer, // the buffer needs to stay alive
    transform_data: Vec<TransformData>,
    image_data: Image,
}

impl Texture {
    pub fn instanced(image: &Image) -> Self {
        Self {
            transform: Transform::new(),
            program: Program::default(),
            vertex_array: VertexArray::default(),
            model_buffer: Buffer::default(),
            texture_buffer: TextureBuffer::default(),
            transform_buffer: Buffer::default(),
            transform_data: vec![],
            image_data: image.clone(),
        }
    }

    // add an new Texture to the transform data
    pub fn new(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let transform_data: TransformData = [
            x, y, width, height, 
        ];

        self.transform_data.push(transform_data);
    }
}

impl Component for Texture {
    fn load(&mut self) -> Result<(), String> {
        let model_data: [f32; 4*4] = [
            1.0,  0.0, 1.0, 1.0,    // top right 0
            0.0,  0.0, 0.0, 1.0,    // top left 1
            0.0,  1.0, 0.0, 0.0,    // bottom left 2 
            1.0,  1.0, 1.0, 0.0,    // bottom right 3
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
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 16, 0 as *const _); // position
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 16, 8 as *const _); // texcoord
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);

            // create the texture buffer out of the image
            self.texture_buffer = TextureBuffer::new();
            self.texture_buffer.set_data(&self.image_data.to_rgba_image());
          
            // create a new buffer for our transform data
            self.transform_buffer = Buffer::new(gl::ARRAY_BUFFER, gl::DYNAMIC_DRAW);
            self.transform_buffer.set_data(&transform_data);
            // and create the attributes in the vertex shader
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 16, 0 as *const _); // offset
            gl::VertexAttribPointer(3, 2, gl::FLOAT, gl::FALSE, 16, 8 as *const _); // scale
            gl::VertexAttribDivisor(2, 1);
            gl::VertexAttribDivisor(3, 1);
            gl::EnableVertexAttribArray(2);
            gl::EnableVertexAttribArray(3);
        }

        Ok(())
    }

    fn draw(&self, draw: &Draw, camera: &Transform) -> Result<(), String> {
        // create the mvp (model view projection) matrixes
        let projection = mvp::ortho(&draw.window);
        let view = camera.matrix();
        let model = self.transform.matrix();

        unsafe {
            // bind the programm and vertex array before sending
            // uniform and drawing
            // we also need to bind the texture buffer
            self.program.bind();
            self.vertex_array.bind();
            self.texture_buffer.bind();
        
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
}