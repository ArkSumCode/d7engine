use crate::prelude::*;

/*
A d7engine component 
can be used throughout your program
so you can put these together in an 
Vec<Component>
*/
pub trait Component {
    fn load(&mut self) -> Result<(), String>;
    fn draw(&self, draw: &Draw, camera: &Transform) -> Result<(), String>;
}

/*
the default trait is used to create empty
data
we need that because we load data afterwards
*/
pub trait Default {
    fn default() -> Self;
}