pub mod shader;
pub mod program;
pub mod project;
pub mod core;

use std::ffi::CString;
use std::time::Instant;

/*
entry function for every project
supply the config and runtime structs

init sdl and opengl and run the gameloop
*/
pub fn init(config: &impl project::Config, runtime: &mut impl project::Runtime) {
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
    let window = video_subsystem
        .window(&config.title(), config.width(), config.height())
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // create an opengl context
    let _gl_context = window.gl_create_context().unwrap();

    // tell opengl where the video subsystem is on the memeory
    let _gl = gl::load_with(
        |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    );

    // set vsync
    video_subsystem.gl_set_swap_interval(1).unwrap();

    // set the viewport to a the initial values
    set_viewport(config.width() as i32, config.height() as i32);

    // event_pump holds all user input events like key or mouse button clicks
    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        // set the default background color
        let color = config.background_color();
        gl::ClearColor(color.r, color.g, color.b, 1.0);
        // enable alpha drawing
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    // create the windows camera
    let mut camera = core::camera::Camera::new(config.width() as i32, config.height() as i32);

    // call the projects load funtion
    runtime.load(&mut camera);

    // create the performance object
    let mut performance = Performance::new();
    let mut delta = 0.0;

    // create the mouse structure
    use crate::core::mouse;
    let mut mouse = mouse::Mouse::new();
  
    'main: loop {
        let mouse_state = event_pump.mouse_state();

        // handling of events
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit{..} => break 'main,
                _ => {}
            }

            // resize the viewport after resizing the window
            if let sdl2::event::Event::Window { win_event, .. } = event {
                if let sdl2::event::WindowEvent::Resized(width, height) = win_event {
                    // change the camers values and set the viewport
                    camera.set_dim(width, height);
                    set_viewport(width, height);
                }
            }

            /*
            transform sdl event into our own format
            so we dont have to include sdl in the project
            */
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;
         
            let mut project_event = match event {
                Event::KeyDown{keycode: Some(Keycode::W), repeat: false, ..} => project::Event::KeyUp,                  // w
                Event::KeyDown{keycode: Some(Keycode::A), repeat: false, ..} => project::Event::KeyLeft,                // a
                Event::KeyDown{keycode: Some(Keycode::D), repeat: false, ..} => project::Event::KeyRight,               // s
                Event::KeyDown{keycode: Some(Keycode::S), repeat: false, ..} => project::Event::KeyDown,                // d
                _ => project::Event::None,
            };

            // handle the mouse wheel, check if y greater or less than 0 
            if let Event::MouseWheel {y, ..} = event {
                project_event = if y < 0 {
                    project::Event::WheelDown
                } else {
                    project::Event::WheelUp
                };
            }

            runtime.inputs(project_event);
        }

        // handle left mouse button click
        if mouse_state.left() {
            runtime.inputs(project::Event::MouseLeft);
        }
        // handle right mouse button click
        if mouse_state.right() {
            runtime.inputs(project::Event::MouseRight);
        }

        // update the mouse position
        mouse.set_pos(mouse_state.x(), mouse_state.y());
     
        unsafe {
            // clear the screen
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // call the projects draw method
        runtime.draw(delta, &mut camera, &mouse);
        
        // sdl will change the window its draing to
        window.gl_swap_window();

        // performance tick
        delta = performance.frame();
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
structure for keeping track of performance
it holds the timestamp of the last frame and the current fps
*/
struct Performance {
    last_frame: Instant,
    fps: f32, 
}

impl Performance {
    // create a new performance object, init timestamp and fps initialize
    pub fn new() -> Performance {
        let last_frame = Instant::now();
        let fps = 0.0;
        Performance { last_frame, fps }
    }

    // calculates fps and returns the delta time
    pub fn frame(&mut self) -> f32 {
        let elapsed = self.last_frame.elapsed();
        self.last_frame =  Instant::now();
        self.fps = (1_000_000_000 / elapsed.as_nanos()) as f32;
        1.0 / self.fps
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
