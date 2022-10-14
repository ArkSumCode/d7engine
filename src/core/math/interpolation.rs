/**  
implementation of linear interpolation
moves a value smoothly between 2 points
*/
pub fn lerp(start: f32, end: f32, step: f32) -> f32 {
    start * (1.0 - step) + (end * step)
}