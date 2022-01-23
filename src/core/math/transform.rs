use crate::prelude::*;
use nalgebra_glm::TMat4;

// holds transform data in the form of a matrix
// used in shaders
pub struct Transform {
    matrix: TMat4<f32>,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {matrix: mvp::identity()}
    }

    // set the x, y and z position
    // on the screen
    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        let id = mvp::identity();
        self.matrix = mvp::translate(&id, x, y, z);
    }

    // add x, y and z to the current position
    pub fn add(&mut self, x: f32, y: f32, z: f32) {
        self.matrix = mvp::translate(&self.matrix, x, y, z);
    }

    // returns the current model (object) matrix
    pub fn matrix(&self) -> TMat4<f32> {
        self.matrix
    }
}