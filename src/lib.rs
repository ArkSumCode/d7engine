pub mod shader;
pub mod program;
pub mod core;
pub mod prelude;

use std::ffi::CString;
use sdl2::surface::Surface;
use crate::core::event::Event;

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
    gl_attr.set_context_version(4, 5);
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
        |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
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

    // create the default shaders
    let default_shaders = program::load().unwrap();

    // call the projects load funtion
    runtime.load();

    // create the performance object
    let mut performance = crate::core::performance::Performance::new();

    // create the mouse structure
    use crate::core::mouse;
   
  
    'main: loop {
        let mut special_inputs = vec![];
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

            /*
            transform sdl event into our own format
            so we dont have to include sdl in the project
            */
            use sdl2::keyboard::Keycode;
           
            // handle special keys
            let mut project_event = match event {
                sdl2::event::Event::KeyDown{keycode: Some(Keycode::W), repeat: false, ..} => Event::KeyUp,                  // w
                sdl2::event::Event::KeyDown{keycode: Some(Keycode::A), repeat: false, ..} => Event::KeyLeft,                // a
                sdl2::event::Event::KeyDown{keycode: Some(Keycode::D), repeat: false, ..} => Event::KeyRight,               // s
                sdl2::event::Event::KeyDown{keycode: Some(Keycode::S), repeat: false, ..} => Event::KeyDown,                // d
                sdl2::event::Event::KeyDown{keycode: Some(Keycode::Escape), repeat: false, ..} => Event::Escape,            // esc
                _ => Event::None,
            };

            // handle the mouse wheel, check if y greater or less than 0 
            if let sdl2::event::Event::MouseWheel {y, ..} = event {
                mws = if y < 0 {
                    crate::core::mouse::MouseWheelState::Down
                } else {
                    crate::core::mouse::MouseWheelState::Up
                };
            }

            special_inputs.push(project_event);
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
     
        unsafe {
            // clear the screen
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let draw = crate::core::project::Draw {
            shaders: &default_shaders,
            performance,
            window: win,
            mouse: mouse,
        };

        // call the projects draw method
        runtime.draw(&draw);
        
        // sdl will change the window its draing to
        window.gl_swap_window();

        // performance tick
        performance.frame();
    }

}

/*
create a c string of a certain length of whitespaces
mainly used to get opengl errors
*/
fn create_whitespace_cstring(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
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
