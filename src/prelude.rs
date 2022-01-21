/*
in your project use this line:

use d7engine::prelude::*;

to import the most important stuff of this engine, 
so you wont have to import everything manualy 
*/

pub use crate::project::{Config, Runtime, Event, Draw};
pub use crate::core::color::{Color};
pub use crate::init;
pub use crate::core::mouse::Mouse;
pub use crate::program::text::Text;
pub use crate::program;
pub use crate::program::Program;
pub use std::collections::HashMap;
pub use crate::program::rect::Rect;
pub use crate::core::resource::{font::Font, image::Image};
pub use crate::program::texture::Texture;
pub use crate::core::file;
pub use crate::core::seed::{Seed,Roll};
pub use crate::core::math::model_view_projection;
pub use crate::core::window::Window;
pub use crate::core::math::transform::Transform;