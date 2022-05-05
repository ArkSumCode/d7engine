use crate::prelude::*;
use nalgebra_glm::Mat4;

// holds transform data in the form of a matrix
// and the x, y and z values
// used in shaders
#[derive(Copy, Clone)]
pub struct Transform {
    object: Mat4,
    rotation: Mat4,
    scale: Mat4,
    x: f32, y: f32, z: f32
}

impl Transform {
    pub fn new() -> Transform {
        let object = mvp::identity();
        let scale = mvp::identity();
        let rotation = mvp::identity();
        Transform {object, rotation, scale, x: 0.0, y: 0.0, z: 0.0}
    }

    // set the x, y and z position
    // on the screen
    // set the structs values and the matrix values
    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        let id = mvp::identity();
        self.x = x;
        self.y = y;
        self.z = z;
        self.object = mvp::translate(&id, x, y, z);
    }

    // set the x position 
    pub fn set_x(&mut self, val: f32) {
        let id = mvp::identity();
        self.x = val;
        self.object = mvp::translate(&id, val, self.y, self.z);
    }

    // set the y position 
    pub fn set_y(&mut self, val: f32) {
        let id = mvp::identity();
        self.y = val;
        self.object = mvp::translate(&id, self.x, val, self.z);
    }

    // set the z position 
    pub fn set_z(&mut self, val: f32) {
        let id = mvp::identity();
        self.z = val;
        self.object = mvp::translate(&id, self.x, self.y, val);
    }

    // add x, y and z to the current position
    // adds the structs values and the matrix values
    pub fn add(&mut self, x: f32, y: f32, z: f32) {
        self.x += x;
        self.y += y;
        self.z += z;
        self.object = mvp::translate(&self.object, x, y, z);
    }

    // add to the x position 
    pub fn add_x(&mut self, val: f32) {
        self.x += val;
        self.object = mvp::translate(&self.object, val, 0.0, 0.0);
    }

    // add to the y position 
    pub fn add_y(&mut self, val: f32) {
        self.y += val;
        self.object = mvp::translate(&self.object, 0.0, val, 0.0);
    }

    // add to the z position 
    pub fn add_z(&mut self, val: f32) {
        self.z += val;
        self.object = mvp::translate(&self.object, 0.0, 0.0, val);
    }

    // returns the current model (object) matrix
    pub fn matrix(&self) -> Mat4 {
        self.object * self.rotation * self.scale
    }

    // returns the position values of the matrix
    pub fn pos(&self) -> (f32, f32, f32) {
        (
            self.object[(0, 3)], // x
            self.object[(1, 3)], // y
            self.object[(2, 3)], // z
        )
    }

    // add to rotate of the transform
    pub fn set_rotation(&mut self, angle: f32, x: f32, y: f32, z: f32) {
        let id = mvp::identity();
        self.rotation = mvp::rotate(&id, angle, x, y, z);
    }

    // add to rotate of the transform
    pub fn add_rotation(&mut self, angle: f32, x: f32, y: f32, z: f32) {
        self.rotation = mvp::rotate(&self.rotation, angle, x, y, z);
    }

    // set the scale of the transform
    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        let id = mvp::identity();
        self.scale = mvp::scale(&id, x, y, z);
    }

    // add to the scale of the transform
    pub fn add_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = mvp::scale(&self.scale, x, y, z);
    }
}