pub mod shader;
pub mod program;
pub mod project;
pub mod transform;
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

    // event_pump holds all user input events like key or mouse button clicks
    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        // set the viewport and set the default background color
        gl::Viewport(0, 0, config.width() as i32, config.height() as i32);
        let color = config.background_color();
        gl::ClearColor(color.r, color.g, color.b, 1.0);
    }

    // call the projects load funtion
    runtime.load();

    // create the performance object
    let mut performance = Performance::new();
    let mut delta = 0.0;
  
    'main: loop {
        // handling of events
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit{..} => break 'main,
                _ => {}
            }

            // resize the viewport after resizing the window
            if let sdl2::event::Event::Window { win_event, .. } = event {
                if let sdl2::event::WindowEvent::Resized(width, height) = win_event {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                }
            }
        }

     
        unsafe {
            // clear the screen
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // call the projects draw method
        runtime.draw(delta);
        
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