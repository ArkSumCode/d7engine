use gl::types::*;
use crate::prelude::*;

pub mod rect;
pub mod texture;
pub mod text;

/*
a buffer holds the information
used for the attributes of the vertex shader source code

it has an id generated by opengl that points to the graphics card
the target is a value like gl::ARRAY_BUFFER
*/
pub struct Buffer {
    pub id: GLuint,
    target: GLuint,
    usage: GLuint,
}

impl Buffer {
    // create a Buffer Object
    // opengl will generate the id
    pub fn new(target: GLuint, usage: GLuint) -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self { id, target, usage }
    }

    // set the buffer to active, 
    // only 1 buffer can be active at a time
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }

    // give the buffer its data
    // this data is an array of vertices
    // this data describes the attributes of the vertex shader attributes
    pub fn set_data<DataType>(&self, data: &Vec<DataType>) {
        self.bind();
        let size = data.len() * std::mem::size_of::<DataType>();
        let data = data.as_ptr();

        unsafe {
            gl::BufferData(
                self.target,
                size as gl::types::GLsizeiptr,
                data as *const gl::types::GLvoid,
                self.usage
            );
        }
    }
}

impl Default for Buffer {
    // get an empty buffer
    fn default() -> Self {
        Self {
            id: 0,
            target: 0,
            usage: 0,
        }
    }
}

// delete the data from the graphics card
// when the buffer dropps
impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

/*
A VertexArray is all the data in a single draw call
this can include multiple buffers

it has an id that gets generated by opengl and points to the graphics card
*/
pub struct VertexArray {
    pub id: GLuint,
}

impl VertexArray {
    // generate the id 
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self { id }
    }

    // set the Vertex Array to active 
    // there can only one vertex array be active at a time
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
}

impl Default for VertexArray {
    // create an empty vertex array
    fn default() -> Self {
        Self {
            id: 0,
        }
    }
}

// delete the vertex array on the graphics card 
// when VertexArray gets dropped
impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}


// a Texture Buffer
// holds information from textures
// we need a seperate location on the grapics card
pub struct TextureBuffer {
    pub id: GLuint,
}

impl TextureBuffer {
    // generate the id 
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Self { id }
    }

    // set the texture buffer to active 
    // there can only one texture buffer be active at a time
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    // give the buffer its data
    // this data is an image
    pub fn set_data(&self, image: &image::RgbaImage) {
        self.bind();
        let (width, height) = image.dimensions();

        unsafe {
            // pixelate the image when scaling down
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            // pixelate the image when scaling up
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            // fill in empty space around the texture
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

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
        }
    }
}

impl Default for TextureBuffer {
    // an empty TextureBuffer
    fn default() -> Self {
        Self {
            id: 0,
        }
    }
}

// delete the texture buffer on the graphics card 
// when VertexArray gets dropped
impl Drop for TextureBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}

// Implement this Trait on shader objects
// so we can use them in the component system
pub trait Object {
    fn add(&mut self, component_data: &ComponentData);
    fn load(&mut self) -> Result<(), String>;
    fn reload(&mut self);
    fn remove(&mut self, i: usize);
    fn draw(&mut self, draw: &Draw, camera: &Transform, model_transform: &Transform) -> Result<(), String>;
    fn set_state(&mut self, object_state: ObjectState);
}

// describes the object state
// used so that we dont reload unnecessarily
pub enum ObjectState {
    Ok,
    Reload
}


// a type that describes the array of
// 8 floats which is 4 times 2 floats uv data
// 2 for every corner of the rectangle
pub type TextureCoordinate = [f32; 8];