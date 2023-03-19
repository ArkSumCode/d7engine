// defines the mouse wheel to be 
// in one of 3 positions
// up, down and nothing 
#[derive(Clone, PartialEq, Debug)]
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
    pub x: f32,
    pub y: f32,
    left: bool,
    right: bool,
    mws: MouseWheelState,
}

impl Mouse {
    // create a new mouse structures
    pub fn new(x: f32, y: f32, left: bool, right: bool, mws: MouseWheelState) -> Mouse {
        Mouse {x, y, left, right, mws}
    }

    // returns the x and y position of the mouse realtive to top left
    pub fn pos(&self) -> (f32, f32) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse() {
        let mouse = Mouse::new(100.0, 250.0, false, true, MouseWheelState::None);
        assert_eq!(mouse.pos(), (100.0, 250.0));
        assert_eq!(mouse.left(), false);
        assert_eq!(mouse.right(), true);
        assert_eq!(mouse.mws(), MouseWheelState::None);
    }
}