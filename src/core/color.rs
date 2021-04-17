/*
Colors in opengl are 3 values in range of 0.0 to 1.0
*/
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