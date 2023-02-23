use std::collections::HashMap;
use crate::core::color::Color;
use crate::core::shader::object::TextureCoordinate;
use crate::core::shader::shader::Shader;
use crate::*;

/// add an Instance to a [InstancedShader](InstancedShader)
/// ```rust
/// let mut isd = InstancedShader::rect().unwrap();
/// let mut attr = Attributes::default();
/// attr.dim = (128.0, 128.0);
/// attr.offset = (450.0, 210.0);
/// 
/// ```
#[derive(Clone, Copy)]
pub struct ObjectData {
    pub color: Color,
    pub dim: (f32, f32),
    pub opacity: f32,
    pub offset: (f32, f32),
    pub texcoord: TextureCoordinate,
}

impl Default for ObjectData {
    // create empty ObjectData
    fn default() -> Self {
        Self {
            color: Color::default(),
            dim: (0.0, 0.0),
            opacity: 1.0,
            offset: (0.0, 0.0),
            texcoord: [   
                0.0, 0.0,
                0.0, 1.0,
                1.0, 1.0,            
                1.0, 0.0,
            ],
        }
    }
}



// helps to store Shader in your struct
pub struct ShaderContainer {
    map: HashMap<String, Shader>,
}

impl ShaderContainer {
    // standard Hashmap function
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    // standard Hashmap function
    pub fn insert(&mut self, key: &str, value: Shader) {
        self.map.insert(key.to_string(), value);
    }

    // standard Hashmap function
    pub fn get(&self, key: &str) -> Option<&Shader> {
        self.map.get(key)
    }

    // standard Hashmap function
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Shader> {
        self.map.get_mut(key)
    }

    // adds drawing for loop for cleaner user code
    // draws all Shaders in map
    pub fn draw(&mut self, draw: &Draw, camera: &Transform) -> Result<(), String> {
        for (_, shader) in &mut self.map {
            shader.draw(draw, camera)?;
        }

        Ok(())
    }
}
