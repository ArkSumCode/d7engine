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

            return Err(msg);
        }
        
        Ok(Shader{id})
    }
}

/*
we want to impl Drop, 
so we can free the memory on the graphics card, 
once we dont need the shader anymore
*/
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}