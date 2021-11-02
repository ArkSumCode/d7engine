/*
in your project use this line:

use d7engine::prelude::*;

to import the most important stuff of this engine, 
so you wont have to import everything manualy 
*/

pub use crate::project::{Config, Runtime, Event};
pub use crate::core::color::{Color};
pub use crate::init;
pub use crate::core::camera::Camera;
pub use crate::core::mouse::Mouse;