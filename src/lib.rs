pub mod shader;
pub mod program;
pub mod api;

use std::ffi::CString;

pub fn init(config: &impl api::Config, runtime: &mut impl api::Runtime) {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    gl_attr.set_double_buffer(true);

    let window = video_subsystem
        .window(&config.title(), config.width(), config.height())
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let _gl = gl::load_with(
        |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    );

    // vsync
    video_subsystem.gl_set_swap_interval(1).unwrap();

    unsafe {
        gl::Viewport(0, 0, config.width() as i32, config.height() as i32);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    runtime.load();
  
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit{..} => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        runtime.draw();
        
        window.gl_swap_window();
    }

}

fn create_whitespace_cstring(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
