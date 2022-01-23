use std::time::Instant;

/*
structure for keeping track of performance
it holds the timestamp of the last frame and the current fps
*/
#[derive(Copy, Clone)]
pub struct Performance {
    last_frame: Instant,
    fps: f32, 
    delta: f32,
}

impl Performance {
    // create a new performance object, init timestamp and fps initialize
    pub fn new() -> Performance {
        let last_frame = Instant::now();
        let fps = 0.0;
        let delta = 0.0;
        Performance { last_frame, fps, delta }
    }

    // calculates fps and returns the delta time
    pub fn frame(&mut self) {
        let elapsed = self.last_frame.elapsed();
        self.last_frame =  Instant::now();
        self.fps = (1_000_000_000 / elapsed.as_nanos()) as f32;
        self.delta = 1.0 / self.fps;
    }

    // returns the current fps
    pub fn fps(&self) -> f32 {
        self.fps
    }

    // returns the current delta
    pub fn delta(&self) -> f32 {
        self.delta
    }
}
