use crate::*;

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
            title: String::from("d7engine"),
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

    // draw is called every frame
    fn draw(&mut self, draw: &Draw);
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