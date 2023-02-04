//! d7engine dokumentation

pub mod core;

use sdl2::surface::Surface;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

use crate::core::mouse;

pub use gl;
pub use nalgebra_glm;
pub use std::ffi::CString;
pub use std::f32::consts::PI;
pub use crate::core::shader::Shader;
pub use crate::core::project::{Config, Runtime, Draw, Performance};
pub use crate::core::color::{Color};
pub use crate::core::mouse::{MouseWheelState, Mouse};
pub use std::collections::HashMap;
pub use crate::core::resource::{font::Font, tilemap, image::Image, tilemap::TileMap};
pub use crate::core::{seed, seed::Seed};
pub use crate::core::math::{mvp, collision, collision::Collision, pathfinding, interpolation::lerp};
pub use crate::core::window::Window;
pub use crate::core::math::transform::Transform;
pub use crate::core::{file, file::installation::Installation};
pub use std::path::{PathBuf, Path};
pub use crate::core::shader::program::Program;
pub use crate::core::shader::object;
pub use crate::core::component::{Default, Component, ComponentData, InstancedComponent};
pub use crate::core::component::{ComponentContainer};

/*
entry function for every project
supply the config and runtime structs

init sdl and opengl and run the gameloop
*/
pub fn init(config: crate::core::project::Config, runtime: &mut impl crate::core::project::Runtime) {
    // init sdl and the video subsystem
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    // opengl settings
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    // version
    gl_attr.set_context_version(3, 3);
    // double buffering
    gl_attr.set_double_buffer(true);

    // create the window using opengl and make it resizable
    let mut window = video_subsystem
        .window(&config.title, config.width, config.height)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // create an opengl context
    let _gl_context = window.gl_create_context().unwrap();

    // set the window icon
    if let Ok(window_icon) = Surface::load_bmp("icon.bmp") {
        window.set_icon(window_icon);
    }

    // tell opengl where the video subsystem is on the memeory
    let _gl = gl::load_with(
        |ptr| video_subsystem.gl_get_proc_address(ptr) as *const _
    );

    // set vsync
    video_subsystem.gl_set_swap_interval(1).unwrap();

    // set the viewport to a the initial values
    set_viewport(config.width as i32, config.height as i32);

    // event_pump holds all user input events like key or mouse button clicks
    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        // set the default background color
        let color = config.background_color;
        gl::ClearColor(color.r, color.g, color.b, 1.0);
        // enable alpha drawing
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    // create the window struct with width and height
    let mut win = core::window::Window::new(config.width as i32, config.height as i32);

    // create the performance object
    let mut performance = Performance::new();

    // call the projects load funtion
    runtime.load();
  
    'main: loop {
        let mut mws = crate::core::mouse::MouseWheelState::None;
      
        // handling of events
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit{..} => break 'main,
                _ => {}
            }

            // resize the viewport after resizing the window
            if let sdl2::event::Event::Window { win_event, .. } = event {
                if let sdl2::event::WindowEvent::Resized(width, height) = win_event {
                    // create the window struct with width and height
                    win = core::window::Window::new(width as i32, height as i32);
                    set_viewport(width, height);
                }
            }
           
            // handle the mouse wheel, check if y greater or less than 0 
            if let sdl2::event::Event::MouseWheel {y, ..} = event {
                mws = if y < 0 {
                    crate::core::mouse::MouseWheelState::Down
                } else {
                    crate::core::mouse::MouseWheelState::Up
                };
            }
        }

        // create a new mouse struct thats holds the data for our draw struct
        let mouse_state = event_pump.mouse_state();
        let mouse = mouse::Mouse::new(
            mouse_state.x(), 
            mouse_state.y(), 
            mouse_state.left(), 
            mouse_state.right(), 
            mws
        );

        // Create a set of pressed Keys.
        let hashset_keys: HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        // Create a vec of Strings to 
        // pass to draw functions
        let mut keys = vec![];
        for key in hashset_keys {
            keys.push(key.to_string());
        }

        // create the draw struct 
        // that will be passed to draw functions
        let draw = crate::core::project::Draw {
            performance: performance.clone(),
            window: win,
            mouse: mouse,
            keys: keys,
        };
     
        unsafe {
            // clear the screen
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // call the projects draw method
        runtime.draw(&draw);
        
        // sdl will change the window its draing to
        window.gl_swap_window();

        // performance tick
        performance.frame();
    }

}

/*
always set the viewport to be a square so 
rects on different resolutions are the same ratio
*/
fn set_viewport(width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}
