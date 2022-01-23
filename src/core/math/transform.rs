use crate::prelude::*;
use nalgebra_glm::TMat4;

// holds current transform data
// the view (camera matrix) and model (object matrix) 
// and the projection mat (which is the same for all objects) 
// will be uniforms in shaders
pub struct Transform {
    model: TMat4<f32>,
}

impl Transform {
    pub fn new() -> Transform {
        let model = mvp::identity();
        Transform {model}
    }

    // set the x, y and z position
    // on the screen
    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        let id = mvp::identity();
        self.model = mvp::translate(&id, x, y, z);
    }

    // returns the current model (object) matrix
    pub fn model(&self) -> TMat4<f32> {
        self.model
    }

    // returns an empty view (camera) matrix
    pub fn view() -> TMat4<f32> {
        mvp::identity()
    } 
}