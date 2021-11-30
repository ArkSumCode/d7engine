/*
camera is the screen 
it holds the current screen width and height
pass it as uniform to the shaders,
the shaders can then calc the right vertices
zoom is how big units appear on the screen
*/
#[derive(Clone)]
pub struct Camera {
    pub width: f32,
    pub height: f32,
}

impl Camera {
    // create a new camera 
    pub fn new(width: f32, height: f32) -> Camera {
        Camera{width, height}
    }

    // refresh the cameras width and height, like after resizing sdl window
    pub fn set_dim(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

   
}