use crate::texture::Texture;
use crate::shapes::{Transform, rect::Rect};
use crate::color::Color;
use crate::core::seed::Seed;
use std::cell::RefCell;

pub mod layer;
pub mod map;

use layer::{Layer, LayerName};

/*
a chunk is a tilemap, 
with different layers
*/
pub struct Chunk {
    background: Vec<Rect>,
    layers: [Layer; 3],
}

impl Chunk {
    // create a new empty chunk
    pub fn new(x: i32, y: i32, rows: u32, cols: u32, seed: &RefCell<Seed>) -> Chunk {
        let layers = layer::empty_layers(false);

        // creating and setting the default background
        let mut background = vec![];
        for tile_coord_y in 0..rows {
            for tile_coord_x in 0..cols {
                let transform = map_coords_to_transform(x, y, rows,cols,tile_coord_x, tile_coord_y, 0.05, 0.05);
                let color = match seed.borrow_mut().next_bool() {
                    true => Color::from(27, 125, 60), // greenish
                    false => Color::from(112, 59, 42), // brownish
                };
                let rect = Rect::new(transform, color);
                background.push(rect);
            }
        }
     

        Chunk{background,layers}
    }

    // add a layern to the chunk
    pub fn add_layer(&mut self, name: LayerName, textures: Vec<Texture>) {
        let active = true;

        // get the id so we can access array
        let id = match name {
            LayerName::BACK => 0,
            LayerName::MIDDLE => 1,
            LayerName::FRONT => 2,
        };

        let layer = Layer{textures, active};
        self.layers[id] = layer;
    }

    // doing stuff at runtime, like creating shader programs
    pub fn load(&mut self) {
        for rect in &mut self.background {
            rect.create_shader_buffer();
        }
    }

    // draw the background to the screen
    pub fn draw_background(&self) {
        for rect in &self.background {
            rect.draw();
        }
    }

    // draw the layers to the screen
    pub fn draw_layers(&self) {
        /*
        go throug all layers
        by their respective order 
        and draw them if they are active
        */
        for i in 0..3 {
            if self.layers[i].active {
                self.layers[i].draw();
            }
        }
    }
}

/* 
transforming x and y coords into a real transform
width and height is real height of one tile
*/
fn map_coords_to_transform
(
    chunk_x: i32, 
    chunk_y: i32, 
    rows: u32, 
    cols: u32, 
    x: u32, 
    y: u32, 
    width: f32, 
    height: f32
) -> Transform {
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