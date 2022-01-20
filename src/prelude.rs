/*
in your project use this line:

use d7engine::prelude::*;

to import the most important stuff of this engine, 
so you wont have to import everything manualy 
*/

pub use crate::project::{Config, Runtime, Event, Draw};
pub use crate::core::color::{Color};
pub use crate::init;
pub use crate::core::camera::Camera;
pub use crate::core::mouse::Mouse;
pub use crate::core::text::Text;
pub use crate::program;
pub use crate::program::Program;
pub use std::collections::HashMap;
pub use crate::core::transform::Transform;
pub use crate::core::rect::Rect;
pub use crate::core::font::Font;
pub use crate::core::texture::Texture;
pub use crate::core::texture::image_data;
pub use crate::core::file;
pub use crate::core::seed::{Seed,Roll};