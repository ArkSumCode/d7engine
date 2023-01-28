/**  
implementation of linear interpolation
moves a value smoothly between 2 points
*/
pub fn lerp(start: f32, end: f32, step: f32) -> f32 {
    start * (1.0 - step) + (end * step)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_lerp() {
        let end = 1.0;
        let step = 0.5;

        assert_eq!(0.5, lerp(0.0, end, step));
        assert_eq!(0.75, lerp(0.5, end, step));
        assert_eq!(0.875, lerp(0.75, end, step));
        assert_ne!(0.0, lerp(0.0, end, step));
    }
}