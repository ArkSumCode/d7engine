// this struct holds the information 
// of the actuall window (e.g width height)
// we don't want to allow to write to width and
// height

#[derive(Copy,Clone)]
pub struct Window {
    width: i32,
    height: i32,
}

impl Window {
    // create a new window struct
    pub fn new(width: i32, height: i32) -> Window {
        Window {width, height}
    }

    // returns the width of the window
    pub fn width(&self) -> i32 {
        self.width
    }

    // returns the height of the window
    pub fn height(&self) -> i32 {
        self.height
    }
}