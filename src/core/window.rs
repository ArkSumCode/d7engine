// this struct holds the information 
// of the actuall window (e.g width height)
// we don't want to allow to write to width and
// height

#[derive(Copy,Clone)]
pub struct Window {
    pub width: f32,
    pub height: f32,
}

impl Window {
    // create a new window struct
    pub fn new(width: f32, height: f32) -> Window {
        Window {width, height}
    }
}