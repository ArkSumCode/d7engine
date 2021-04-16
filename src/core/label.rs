use crate::core::*;

/*
Label holds the text of the label
its transform
and the vector of all the chars textures
*/
pub struct Label {
    text: String,
    transform: transform::Transform,
    textures: Vec<texture::Texture>,
}

impl Label {
    // create a new label with the transform and the text of the label
    pub fn new(transform: transform::Transform, text: String) -> Label {
        Label{textures: vec![], text, transform}
    }

    // create all the textures and all transforms of the textures, camera font and color therfore needed
    pub fn load(&mut self, camera: &camera::Camera, font: &font::Font, color: &color::Color) {
        let mut x = 0;

        for d in self.text.chars() {
            if d == ' ' {
                x += 1;
                continue;
            }

            if let Some(img) = font.get(&d.to_string()) {
             
                // the textures transform
                let transform = transform::Transform{
                    x: self.transform.x + x * font.spacing(),
                    y: self.transform.y,
                    width: font.dimension(),
                    height: font.dimension(),
                };
    
                let mut texture = texture::Texture::new(transform);
                texture.create_colored_shader_buffer(&img, camera, &color);
                self.textures.push(texture);
                x += 1;
            }
        }
    }

    // draw the label aka all the chars textures
    pub fn draw(&self) {
        for img in &self.textures {
            img.draw();
        }
    }
}