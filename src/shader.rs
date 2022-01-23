use std::ffi::CStr;

/*
Shader is reponsible for loading the actual shader file 
which can be a vertex or a fragment shader,
and then adding it to your graphics card,

opengl will create an id, we need to store
*/
pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    // creating a vertex shader, source is already the file as a string (sourcecode)
    pub fn from_vertex(source: &CStr) -> Result<Shader, String> {
        Shader::from(source, gl::VERTEX_SHADER)
    }

    // creating a fragment shader, source is already the file as a string (sourcecode)
    pub fn from_fragment(source: &CStr) -> Result<Shader, String> {
        Shader::from(source, gl::FRAGMENT_SHADER)
    }

    // return the shaders id
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    /*
    compiling a shader, source is already the file as a string
    kind is the type of the shader which can ether be a 
    vertex or a fragment shader.
    */
    fn from(source: &CStr, kind: gl::types::GLuint) -> Result<Shader, String> {
        // get a new id
        let id = unsafe { gl::CreateShader(kind) };

        unsafe {
            /*
            we pass the shader id, the sourcecode of the shader
            ShaderSource wants the shader type, the number of shaders, the shader sourcecode, and its length
            length can be a null pointer, at this point i dont know why
            then we want opengl to compile the shader
            */
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        // check the compile status and write it to "success"
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;

            unsafe {
                // get the error length
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            // create a c string, which opengl will write the error message to
            let error = crate::create_whitespace_cstring(len as usize);

            unsafe {
                // write the error message to "error"
                gl::GetShaderInfoLog(
                    id, 
                    len, 
                    std::ptr::null_mut(), 
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            // cstring to rust String
            let msg = error.to_string_lossy().into_owned();
            let msg = format!("shaders: {}", msg);
            return Err(msg);
        }
        
        Ok(Shader{id})
    }
}

/*
impl Drop, so the memory is freed on the graphics card, 
once the shader is not in use anymore
*/
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

/*
creating the default shader buffer
used in triangle and rect shapes
*/
pub fn create_default_shader_buffer(vertices: Vec<f32>) -> gl::types::GLuint {
    let mut buffer: gl::types::GLuint = 0;
    let mut array: gl::types::GLuint = 0;
    unsafe {
        // create unused integers from opengl we can write tp
        gl::GenBuffers(1, &mut buffer);
        gl::GenVertexArrays(1, &mut array);
        /*
        tell opengl that at point "buffer" we want to pass vertices 
        gl::ARRAY_BUFFER tells opengl that the data are vertices and not
        for example a texture
        then tell opengl the position of "array"
        */
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        gl::BindVertexArray(array);

        // describe the buffers data 
        let size = vertices.len() * std::mem::size_of::<f32>();
        let data = vertices.as_ptr();
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size as gl::types::GLsizeiptr,
            data as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        let attr_position_coord = 0;
        let size_vec3 = 3 * std::mem::size_of::<f32>();
        let out_vec_3 = 3;

        /*
        explaining each attribute to opengl
        vertexattribpointer (
            location in the vertex shader eg. 0,1,2,3 etc,
            specify the output vector (max is vec4 = 4),
            type of one value in one vertex eg 0.0, 0.1, 0.4 etc..,
            we have float so set to false (integers are true),
            size of one vertex position plus color are 6 floats,
            offset data eg when position is at 0 color is 3 values further,
        )
        */

        gl::VertexAttribPointer(
            attr_position_coord, 
            out_vec_3,
            gl::FLOAT,
            gl::FALSE,
            size_vec3 as gl::types::GLint,
            0 as *const gl::types::GLvoid,
        );

         // enable the attributes
         gl::EnableVertexAttribArray(attr_position_coord);

        // clear bindings
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    array
}

/*
creates the texture shader infos and returns the
id of the texture buffer and the vertex array buffer 
(texture, array)
*/
pub fn create_texture_shader_buffer(vertices: Vec<f32>, image: &image::RgbaImage) -> (gl::types::GLuint, gl::types::GLuint) {
    let (width, height) = image.dimensions();
    let mut texture: gl::types::GLuint = 0;
    let mut buffer: gl::types::GLuint = 0;
    let mut array: gl::types::GLuint = 0;

    unsafe {
        // create unused integers from opengl we can write tp
        gl::GenTextures(1, &mut texture);
        gl::GenBuffers(1, &mut buffer);
        gl::GenVertexArrays(1, &mut array);
        /*
        tell opengl that at point "buffer" we want to pass vertices 
        gl::ARRAY_BUFFER tells opengl that the data are vertices and not
        for example a texture
        then tell opengl the position of "array"
        */
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        gl::BindVertexArray(array);

        // pixelate the image when scaling down
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        // pixelate the image when scaling up
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        // structure of the texture, with width height and texture data
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            image.as_ptr() as *const gl::types::GLvoid,
        );
        // generate the neccessasary mipmap textures, (needed when texture is very near or far away)
        gl::GenerateMipmap(gl::TEXTURE_2D);
   
        // describe the buffers data 
        let size = vertices.len() * std::mem::size_of::<f32>();
        let data = vertices.as_ptr();
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size as gl::types::GLsizeiptr,
            data as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        let attr_position_coord = 0;
        let size_vec3 = 3 * std::mem::size_of::<f32>();
        let out_vec_3 = 3;

        let attr_texture_coord = 1;
        let size_vec2 = 2 * std::mem::size_of::<f32>();
        let out_vec_2 = 2;

        let size_row = size_vec2 + size_vec3;

         /*
        explaining each attribute to opengl
        vertexattribpointer (
            location in the vertex shader eg. 0,1,2,3 etc,
            specify the output vector (max is vec4 = 4),
            type of one value in one vertex eg 0.0, 0.1, 0.4 etc..,
            we have float so set to false (integers are true),
            size of one vertex position plus color are 6 floats,
            offset data eg when position is at 0 color is 3 values further,
        )
        */

        gl::VertexAttribPointer(
            attr_position_coord, 
            out_vec_3,
            gl::FLOAT,
            gl::FALSE,
            size_row as gl::types::GLint,
            0 as *const gl::types::GLvoid,
        );
        
        gl::VertexAttribPointer(
            attr_texture_coord, 
            out_vec_2,
            gl::FLOAT,
            gl::FALSE,
            size_row as gl::types::GLint,
            size_vec3 as *const gl::types::GLvoid,
        ); 

        // enable the attributes
        gl::EnableVertexAttribArray(attr_position_coord);
        gl::EnableVertexAttribArray(attr_texture_coord);

        // clear bindings
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    (texture, array)
}