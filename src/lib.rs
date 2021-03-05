pub mod shader;
pub mod program;

use std::ffi::CString;

const WIN_WIDTH: u32 = 1280;
const WIN_HEIGHT: u32 = 720;

pub fn init() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    gl_attr.set_double_buffer(true);

    let window = video_subsystem
        .window("Game", WIN_WIDTH, WIN_HEIGHT)
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
        gl::Viewport(0, 0, WIN_WIDTH as i32, WIN_HEIGHT as i32);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }
  
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
        
        window.gl_swap_window();
    }

}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
