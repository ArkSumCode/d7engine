/*
camera is the screen 
it holds the current screen width and height
pass it as uniform to the shaders,
the shaders can then calc the right vertices
zoom is how big units appear on the screen
*/
pub struct Camera {
    pub width: i32,
    pub height: i32,
}

impl Camera {
    // create a new camera 
    pub fn new(width: i32, height: i32) -> Camera {
        Camera{width, height}
    }

    // refresh the cameras width and height, like after resizing sdl window
    pub fn set_dim(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }
}