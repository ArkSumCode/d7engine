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

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    } 
}