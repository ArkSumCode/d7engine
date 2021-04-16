use crate::core::camera;

/*
shapes have a transform struct,
that stores its position and dimensions
x and y represents the top left point of a shape
x+width, y+height is the bottom right point of the shape
*/
pub struct Transform {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Transform {
    // returns a Transform with zero width and height
    pub fn location(x: i32, y: i32) -> Transform {
        Transform {x, y, width: 0, height: 0}
    }
}

/*
Canvas transform is transform in the coordinate system
of the opengl canvas
*/
pub struct CanvasTransform {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl CanvasTransform {
    // transfrom to canvastransform method
    pub fn new(transform: &Transform, camera: &camera::Camera) -> CanvasTransform {
        let x = 2.0 * transform.x as f32 / camera.width as f32 - 1.0;
        let y = 1.0 - 2.0 * transform.y as f32 / camera.height as f32;

        let width = 2.0 * transform.width as f32 / camera.width as f32;
        let height = 2.0 * transform.height as f32 / camera.height as f32;

        CanvasTransform {x, y, width, height}
    }
}