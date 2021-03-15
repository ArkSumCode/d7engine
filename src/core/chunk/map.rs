use crate::core::chunk::Chunk;
use crate::program::Program;
use crate::core::seed::Seed;
use std::cell::RefCell;

/*
a map holds all chunks,
the default and texture shader programs
*/
pub struct Map {
    chunks: Vec<Chunk>,
    programs: Vec<Program>,
}

impl Map {

    // create a new empty map
    pub fn new() -> Map {
        let mut chunks = vec![];
        let seed = RefCell::new(Seed::from_str("meineinselhat2bergemitdemtiefenweitenmeer"));
        
        for y in -2..3 {
            for x in -2..2 {
                
                let chunk = Chunk::new(x, y, 10, 10, &seed);
                chunks.push(chunk);
            }
        }

        let programs = vec![];
        Map{chunks, programs}
    }

    // doing stuff at runtime, like creating shader programs
    pub fn load(&mut self) {
        // create the used shaders
        self.programs = vec![
            Program::default().unwrap(),
            Program::texture().unwrap(),
        ];

        // each chunk needs to create ist shader buffers
        for chunk in &mut self.chunks {
            chunk.load();
        }
    }

    // draw the map to the screen
    pub fn draw(&self) {
        for chunk in &self.chunks {
            self.programs[0].set_used(); // default shaders
            chunk.draw_background();

            self.programs[1].set_used(); // texture shaders
            chunk.draw_layers();
        }
    }
}