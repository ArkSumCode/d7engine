use crate::core::color::Color;
use crate::core::shader::object::TextureCoordinate;

/// add an Instance to a [InstancedShader](InstancedShader)
/// ```rust
/// let mut isd = InstancedShader::rect().unwrap();
/// let mut od = ObjectData::default();
/// od.dim = (128.0, 128.0);
/// od.offset = (450.0, 210.0);
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