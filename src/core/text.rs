use crate::core::*;
use crate::program;

/*
Text holds the text of the Text
its transform
and the vector of all the chars textures
*/
pub struct Text {
    text: String,
    transform: transform::Transform,
    textures: Vec<texture::Texture>,
}

impl Text {
    // create a new Text with the transform and the text of the Text
    pub fn new(transform: transform::Transform, text: &str) -> Text {
        Text{textures: vec![], text: text.to_string(), transform}
    }

    // create all the textures and all transforms of the textures, camera font and color therfore needed
    pub fn load(&mut self, font: &mut font::Font, color: &color::Color) {
        let mut cursor = 0;

        for d in self.text.chars() {
            if d == ' ' {
                cursor += 1;
                continue;
            }

            if let Ok(img) = font.char(d) {
             
                // the textures transform
                let transform = transform::Transform{
                    x: self.transform.x + cursor as f32,
                    y: self.transform.y,
                    width: 150.0,
                    height: 150.0,
                };

                let mut texture = texture::Texture::colored(transform, color);
                texture.create_shader_buffer(&img);
                self.textures.push(texture);
                cursor += 1;
            }
        }
    }

    // draw the text aka all the chars textures
    pub fn draw(&self, program: &program::Program, camera: &camera::Camera) {
        for img in &self.textures {
            img.draw(program, camera);
        }
    }

    // set the position and all the texts textures position
    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.transform.set_pos(x, y);

        let mut cursor = 0;
        let mut i = 0;
        for d in self.text.chars() {
            if d == ' ' {
                cursor += 1;
                continue;
            }

            self.textures[i].set_pos(self.transform.x + cursor as f32, self.transform.y);

            i += 1;
            cursor += 1;
        }
    }
}