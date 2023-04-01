/**  
implementation of linear interpolation
moves a value smoothly between 2 points
*/
pub fn lerp(from: f32, to: f32, step: f32) -> f32 {
    from * (1.0 - step) + (to * step)
}

/**
implementation of linear transformation
map a value from one range to another
take a value a as f32 
and a range from from_min to from_max
and a range from to_min to to_max
return the value as f32
*/
pub fn lint(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    (value - from_min) / (from_max - from_min) * (to_max - to_min) + to_min
}

#[cfg(test)]
mod tests {
    use super::*;

    // create a test case for lerp
    #[test]
    fn test_lerp() {
        let end = 1.0;
        let step = 0.5;

        assert_eq!(0.5, lerp(0.0, end, step));
        assert_eq!(0.75, lerp(0.5, end, step));
        assert_eq!(0.875, lerp(0.75, end, step));
        assert_ne!(0.0, lerp(0.0, end, step));
    }

    // create a test case for lint
    #[test]
    fn test_lint() {
        assert_eq!(15.0, lint(0.5, 0.0, 1.0, 10.0, 20.0));
        assert_eq!(-5.0, lint(50.0, 0.0, 100.0, -10.0, 0.0));
        assert_eq!(0.0, lint(0.0, 0.0, 1.0, 0.0, 1.0));
        assert_eq!(1.0, lint(1.0, 0.0, 1.0, 0.0, 1.0));
        assert_eq!(5.0, lint(-50.0, -100.0, 0.0, 0.0, 10.0));
    }
}