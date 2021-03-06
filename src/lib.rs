pub mod shader;
pub mod program;
pub mod project;
pub mod color;
pub mod shapes;

use std::ffi::CString;

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
        runtime.draw();
        
        // sdl will change the window its draing to
        window.gl_swap_window();
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