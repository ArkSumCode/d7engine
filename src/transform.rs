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
    /* 
    transforming x and y coords into a real transform
    width and height is real height of one tile
    */
    pub fn from_map_coords(
        chunk_x: i32, 
        chunk_y: i32, 
        rows: u32, 
        cols: u32, 
        x: u32, 
        y: u32, 
        width: f32, 
        height: f32
    ) -> Transform 
    {
        let chunk_width = cols as f32 * width;
        let chunk_height = rows as f32 * height;
        let chunk_x = chunk_x as f32 * chunk_width;
        let chunk_y = chunk_y as f32 * chunk_height;
        let tile_x = x as f32 * width;
        let tile_y = y as f32 * height;
        let x = chunk_x + tile_x;
        let y = chunk_y - tile_y;
        Transform{x, y, width, height}
    }
}


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
    pub zoom: f32,
}

impl Camera {
    // create a new camera 
    pub fn new(width: i32, height: i32) -> Camera {
        Camera{width, height, zoom: 0.5}
    }

    // refresh the cameras width and height, like after resizing sdl window
    pub fn set_dim(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    /* 
    zoom in when applying positiv offeset, and out with negative offset
    has to respect zoom boundaries
    */
    pub fn zoom(&mut self, offset: f32) {
        self.zoom = self.zoom + offset;
        if self.zoom < 0.1 {
            self.zoom = 0.1;
        } else if 1.0 < self.zoom {
            self.zoom = 1.0;
        }
    }
}