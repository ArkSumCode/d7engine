use crate::prelude::*;
use nalgebra_glm::TMat4;

// holds current transform data
// the view (camera matrix) and model (object matrix) 
// and the projection mat (which is the same for all objects) 
// will be uniforms in shaders
pub struct Transform {
    view: TMat4<f32>,
    model: TMat4<f32>,
}

impl Transform {
    pub fn new() -> Transform {
        let view = model_view_projection::identity();
        let model = model_view_projection::identity();
        Transform {view, model}
    }

    // set the x, y and z position
    // on the screen
    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        let id = model_view_projection::identity();
        self.model = model_view_projection::translate(&id, x, y, z);
    }

    // returns the current model (object) matrix
    pub fn model(&self) -> TMat4<f32> {
        self.model
    }

    // returns the current view (camera) matrix
    pub fn view(&self) -> TMat4<f32> {
        self.view
    } 
}