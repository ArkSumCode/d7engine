use crate::prelude::*;
use nalgebra_glm::{vec3, TMat4};

// returns an othogonal projection matrix
pub fn ortho(window: &Window) -> TMat4<f32> {
    nalgebra_glm::ortho(0.0, window.width() as f32, window.height() as f32,  0.0, -1.0, 1.0)
}

// translates a matrix 
pub fn translate(src: &TMat4<f32>, x: f32, y: f32, z: f32) -> TMat4<f32> {
    nalgebra_glm::translate(src, &vec3(x, y, z))
}

// returns a standard identity matrix
pub fn identity() -> TMat4<f32> {
    nalgebra_glm::identity()
}

// scale a matrix
pub fn scale(src: &TMat4<f32>, x: f32, y: f32, z: f32) -> TMat4<f32> {
    nalgebra_glm::scale(src, &vec3(x, y, z))
}

// rotate a matrix
// where xyz is the axis
pub fn rotate(src: &TMat4<f32>, angle: f32, x: f32, y: f32, z: f32) -> TMat4<f32> {
    let axis = vec3(x, y, z);
    let axis = nalgebra_glm::normalize(&axis);
    nalgebra_glm::rotate(src, angle, &axis)
}