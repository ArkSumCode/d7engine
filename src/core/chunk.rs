use crate::core::texture::Texture;
use crate::transform::Transform;
use crate::program::Program;
use crate::core::color::Color;
use crate::core::seed::Seed;
use crate::core::rect::Rect;
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
                let transform = Transform::from_map_coords(x, y, rows,cols,tile_coord_x, tile_coord_y, 0.2, 0.2);
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

    // draw the chunk, needs the shaders created in map 
    pub fn draw(&self, default_program: &Program, texture_program: &Program) {
        // draw the background to the screen
        default_program.set_used();
        for rect in &self.background {
            rect.draw();
        }

        /*
        go throug all layers
        by their respective order 
        and draw them if they are active
        */
        texture_program.set_used();
        for i in 0..3 {
            if self.layers[i].active {
                self.layers[i].draw();
            }
        }
    } 
}

