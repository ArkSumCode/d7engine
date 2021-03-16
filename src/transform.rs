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

