use crate::color;

/*
used as argument in the main init function
in the project make a struct that implements Config 
and pass it over to the engine

holds standard information usefull for SDL2
*/
pub trait Config {
    // the title at the top of the window
    fn title(&self) -> String;
    
    // the width of the window at the start of the program
    fn width(&self) -> u32;

    // the height of the window at the start of the program
    fn height(&self) -> u32;

    // the default background and clear color of the window
    fn background_color(&self) -> color::Color;
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
    fn draw(&mut self, delta: f32);
}