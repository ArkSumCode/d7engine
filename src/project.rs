use crate::prelude::*;

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

impl Config {
    // returns a default configuration
    pub fn default() -> Config {
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

    // inputs is called every frame before draw, handle the inputs from sdl
    fn inputs(&mut self, event: Event);

    // draw is called every frame
    fn draw(&mut self, draw: &Draw);
}

/*
definition of all the user events that can happen
this will get past into runtime inputs so the project 
doesnt have to include sdl
return here when unsure what events are available
*/

pub enum Event {
    KeyLeft,
    KeyDown,
    KeyUp,
    KeyRight,
    WheelUp,
    WheelDown,
    MouseLeft,
    MouseRight,
    None,
}

/*
holds important components for the draw functions,
like the shaderprograms, camera etc.
*/
pub struct Draw<'a> {
    pub shaders: &'a HashMap<String, Program>,
    pub performance: crate::Performance,
    pub window: Window,
    pub mouse: Mouse,
}