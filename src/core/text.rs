use crate::prelude::*;

/*
Text holds the text of the Text
its transform
and the vector of all the chars textures
*/
pub struct Text {
    texture: Texture,
}

impl Text {
    // creates a new text, using the font a position(transform), the text, the font size and color
    pub fn new(font: &Font, transform: Transform, text: &str, font_size: u32, color: &Color) -> Result<Text, String> {
        let img = font.snapshot(text, font_size as f32)?;

        // the textures transform
        let transform = Transform{
            x: transform.x,
            y: transform.y,
            width: img.width() as f32,
            height: img.height() as f32,
        };

        let mut texture = Texture::colored(transform, color);
        texture.create_shader_buffer(&img);

        Ok(Text{texture})
    }

    // draw the text' rendered image
    pub fn draw(&self, draw: &Draw) {
        self.texture.draw_font(&draw);
    }
}