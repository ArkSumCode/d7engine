use std::time::Instant;
use crate::core::color::Color;
use crate::core::window::Window;
use crate::core::mouse::Mouse;

/*
used as argument in the main init function
in the project make a struct that is Config
and pass it over to the engine

holds standard information usefull for SDL2
*/

pub struct Config {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub background_color: Color,
}

impl Default for Config {
    // returns a default configuration
    fn default() -> Config {
        Config {
            title: format!("d7engine {}", env!("CARGO_PKG_VERSION")),
            width: 1270,
            height: 700,
            background_color: Color::grey(44),
        }
    }
}

/*
used as argument in the main init function
in the project make a struct that implements Runtime
and pass it over to the engine

this is your main struct then startpoint of your project 
and holds a lot of opengl code
*/
pub trait Runtime {
    // load is called before the actual gameloop
    fn load(&mut self);

    // update is called every frame
    fn update(&mut self, draw: &Draw);
}

/*
holds important components for the draw functions,
like the shaderprograms, camera, events etc.
*/
pub struct Draw {
    pub performance: Performance,
    pub window: Window,
    pub mouse: Mouse,
    pub keys: Vec<String>,
}



/*
structure for keeping track of performance
it holds the timestamp of the last frame and the current fps
*/
#[derive(Clone)]
pub struct Performance {
    last_frame: Instant,
    fps: f32, 
    delta: f32,
}

impl Performance {
    // create a new performance object, init timestamp and fps initialize
    pub fn new() -> Performance {
        let last_frame = Instant::now();
        let fps = 0.0;
        let delta = 0.0;
        Performance { last_frame, fps, delta }
    }

    // calculates fps and returns the delta time
    pub fn frame(&mut self) {
        let elapsed = self.last_frame.elapsed();
        self.last_frame = Instant::now();
        self.fps = 1_000_000_000.0 / elapsed.as_nanos() as f32;
        self.delta = 1.0 / self.fps;
    }

    // returns the current fps
    pub fn fps(&self) -> f32 {
        self.fps
    }

    // returns the current delta
    pub fn delta(&self) -> f32 {
        self.delta
    }
}
