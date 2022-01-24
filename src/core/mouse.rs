// defines the mouse wheel to be 
// in one of 3 positions
// up, down and nothing 
#[derive(Clone)]
pub enum MouseWheelState {
    Up,
    Down,
    None,
}

/*
a little structure to hold the data
from the mouse it comes from sdl2 and is set in lib.rs
*/
pub struct Mouse {
    x: i32,
    y: i32,
    left: bool,
    right: bool,
    mws: MouseWheelState,
}

impl Mouse {
    // create a new mouse structures
    pub fn new(x: i32, y: i32, left: bool, right: bool, mws: MouseWheelState) -> Mouse {
        Mouse {x, y, left, right, mws}
    }

    // returns the x and y position of the mouse realtive to top left
    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    } 

    // returns true if left button is down
    pub fn left(&self) -> bool {
        self.left
    }

     // returns true if right button is down
    pub fn right(&self) -> bool {
        self.right
    }

    // returns the mouse wheel state
    pub fn mws(&self) -> MouseWheelState {
        self.mws.clone()
    }
}

