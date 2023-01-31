/*
Colors in opengl are 3 values in range of 0.0 to 1.0
*/
#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {
    // create a color from the 0 to 255 format
    pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
        let r = red as f32 / 255.0;
        let g = green as f32 / 255.0;
        let b = blue as f32 / 255.0;
        Color {r, g, b, a: 1.0}
    }

    // create a Color that has all values the same like white black or some grey
    pub fn grey(value: u8) -> Color {
        let v = value as f32 / 255.0;
        Color {r: v, g: v, b: v, a: 1.0}
    } 
}

impl Default for Color {
    // returns an empty Color struct
    fn default() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let default = Color::default();
        assert_eq!(default.r, 0.0);
        assert_eq!(default.g, 0.0);
        assert_eq!(default.b, 0.0);
        assert_eq!(default.a, 0.0);

        let some_grey = Color::grey(77);
        let assert_color = 77.0 / 255.0;
        assert_eq!(some_grey.r, assert_color);
        assert_eq!(some_grey.g, assert_color);
        assert_eq!(some_grey.b, assert_color);
        assert_eq!(some_grey.a, 1.0);

        let red = Color::rgb(255, 0, 0);
        assert_eq!(red.r, 1.0);
        assert_eq!(red.g, 0.0);
        assert_eq!(red.b, 0.0);
        assert_eq!(red.a, 1.0);
    }
}