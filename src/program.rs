use crate::shader::Shader;

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
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        unsafe { gl::LinkProgram(program_id); }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = crate::create_whitespace_cstring(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        Ok(Program {id: program_id})

    }

    // get the programs id
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    // tell opengl that we want to use this program for the next drawing operation
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

/*
we want to impl Drop, 
so we can free the memory on the graphics card, 
once we dont need the program anymore
*/
impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}