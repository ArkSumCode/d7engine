d7engine
A project by Markus Dick
d7engine is a homemade games engine for fun.

Basic setup:

pub struct Config {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub background_color: color::Color,
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
    fn load(&mut self, camera: &mut Camera);

    // inputs is called every frame before draw, handle the inputs from sdl
    fn inputs(&mut self, event: Event);

    // draw is called every frame
    fn draw(&mut self, delta: f32, camera: &mut Camera, mouse: &mouse::Mouse);
}


init()