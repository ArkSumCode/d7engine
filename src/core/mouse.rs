/*
a little structure to hold the data
from the mouse it comes from sdl2 and is set in lib.rs
*/
#[derive(Clone)]
pub struct Mouse {
    x: i32,
    y: i32,
}

impl Mouse {
    // create a new mouse structures
    pub fn new() -> Mouse {
        Mouse {
            x: 0,
            y: 0,
        }
    }

    // returns the x and y position of the mouse realtive to top left
    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    } 

    // set the position ot the mouse relative to top left
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

