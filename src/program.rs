use crate::shader::Shader;
use std::ffi::CString;

/*
a program is more or less the render pipline
it holds the vertex and fragment shaders

we need to store the id of the program,
mainly for droping it after we dont need it anymore
and calling set_used when we want this render program
for the next drawing operations
*/
pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    /*
    the default render pipline
    every vertex has a position and color, they will not be modiefied in the shader
    */
    pub fn rect() -> Result<Program, String> {
        let vertex_source = CString::new(include_str!("shaders/rect.vert")).unwrap();
        let vertex_shader = Shader::from_vertex(&vertex_source)?;
    
        let fragment_source = CString::new(include_str!("shaders/rect.frag")).unwrap();
        let fragment_shader = Shader::from_fragment(&fragment_source)?;
    
        Program::from_shaders(&[vertex_shader, fragment_shader])
    }

    /*
    the texture render pipline
    every vertex has a position and a texture coordinate, 
    they will not be modified in the shader
    */
    pub fn texture() -> Result<Program, String> {
        let vertex_source = CString::new(include_str!("shaders/texture.vert")).unwrap();
        let vertex_shader = Shader::from_vertex(&vertex_source)?;
    
        let fragment_source = CString::new(include_str!("shaders/texture.frag")).unwrap();
        let fragment_shader = Shader::from_fragment(&fragment_source)?;
    

        Program::from_shaders(&[vertex_shader, fragment_shader])
    }

    /*
    create a program, attach the shaders use like: 

        let vertex_source = CString::new(include_str!("shaders/texture.vert")).unwrap();
        let vertex_shader = Shader::from_vertex(&vertex_source)?;
    
        let fragment_source = CString::new(include_str!("shaders/texture.frag")).unwrap();
        let fragment_shader = Shader::from_fragment(&fragment_source)?;

        Program::from_shaders(&[vertex_shader, fragment_shader])

    and link the program
    */
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        // opengl creates an id
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            // attach every shader to the program
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        // link the program
        unsafe { gl::LinkProgram(program_id); }

        // write linking status into "success"
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            // write the error messages length to "len"
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            // create a whitespace c string
            let error = crate::create_whitespace_cstring(len as usize);

            unsafe {
                // write the error message to "error"
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            // c string to rust String conversion
            let msg = error.to_string_lossy().into_owned();
            return Err(msg);
        }

        for shader in shaders {
            // clean up memmory after linking
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        Ok(Program {id: program_id})
    }

    // get the programs id
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    // tell opengl that we want to use this program for the next drawing operation
    pub fn active(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /* 
    get the position of a uniform in the vertex shader
    used when passing data to the shader
    */
    pub fn uniform_location(&self, name: &str) -> gl::types::GLint {
        // create a c string from parameter and transform it to a pointer
        let name = CString::new(name).expect("cannot create c-string");
        let name = name.as_bytes_with_nul().as_ptr() as *const i8;

        let loc = unsafe {
            gl::GetUniformLocation(self.id, name)
        };

        loc
    }
}

/*
impl Drop, so the memory is freed on the graphics card, 
once the program is not in use anymore
*/
impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}