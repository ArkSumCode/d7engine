use crate::prelude::*;
use nalgebra_glm::TMat4;

// holds transform data in the form of a matrix
// and the x, y and z values
// used in shaders
pub struct Transform {
    matrix: TMat4<f32>,
    x: f32, y: f32, z: f32
}

impl Transform {
    pub fn new() -> Transform {
        Transform {matrix: mvp::identity(), x: 0.0, y: 0.0, z: 0.0}
    }

    // set the x, y and z position
    // on the screen
    // set the structs values and the matrix values
    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        let id = mvp::identity();
        self.x = x;
        self.y = y;
        self.z = z;
        self.matrix = mvp::translate(&id, x, y, z);
    }

    // set the x position 
    pub fn set_x(&mut self, val: f32) {
        let id = mvp::identity();
        self.x = val;
        self.matrix = mvp::translate(&id, val, self.y, self.z);
    }

    // set the y position 
    pub fn set_y(&mut self, val: f32) {
        let id = mvp::identity();
        self.y = val;
        self.matrix = mvp::translate(&id, self.x, val, self.z);
    }

    // set the z position 
    pub fn set_z(&mut self, val: f32) {
        let id = mvp::identity();
        self.z = val;
        self.matrix = mvp::translate(&id, self.x, self.y, val);
    }

    // add x, y and z to the current position
    // adds the structs values and the matrix values
    pub fn add(&mut self, x: f32, y: f32, z: f32) {
        self.x += x;
        self.y += y;
        self.z += z;
        self.matrix = mvp::translate(&self.matrix, x, y, z);
    }

    // add to the x position 
    pub fn add_x(&mut self, val: f32) {
        self.x += val;
        self.matrix = mvp::translate(&self.matrix, val, 0.0, 0.0);
    }

    // add to the y position 
    pub fn add_y(&mut self, val: f32) {
        self.y += val;
        self.matrix = mvp::translate(&self.matrix, 0.0, val, 0.0);
    }

    // add to the z position 
    pub fn add_z(&mut self, val: f32) {
        self.z += val;
        self.matrix = mvp::translate(&self.matrix, 0.0, 0.0, val);
    }

    // returns the current model (object) matrix
    pub fn matrix(&self) -> TMat4<f32> {
        self.matrix
    }

    // returns the position values of the matrix
    pub fn pos(&self) -> (f32, f32, f32) {
        (
            self.matrix[(4, 1)], // x
            self.matrix[(4, 2)], // y
            self.matrix[(4, 3)], // z
        )
    }
}