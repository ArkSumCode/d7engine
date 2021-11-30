/*
shapes have a transform struct,
that stores its position and dimensions
x and y represents the top left point of a shape
x+width, y+height is the bottom right point of the shape
*/
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Transform {
    // returns a Transform with zero width and height
    pub fn location(x: f32, y: f32) -> Transform {
        Transform {x, y, width: 0.0, height: 0.0}
    }

    // returns a transform shorter notation than writing it yourself
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Transform {
        Transform {x, y, width, height}
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    } 
}