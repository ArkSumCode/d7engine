use crate::core::shader::object::{Object, ObjectState};
use crate::*;
use crate::core::shader::object::{circle::Circle, rect::Rect, text::Text, texture::Texture};
use crate::core::color::Color;
use crate::core::resource::font::Font;
use crate::core::math::{collision, collision::Collision};
use crate::core::shader::object::TextureCoordinate;

/// The api to draw to the screen
/// 
/// create a new rect in load
/// ```rust
/// let mut rect = Shader::rect().unwrap();
/// rect.set_color(&Color::grey(44));
/// rect.transform.set(50.0, 50.0, 0.0);
/// rect.set_dim(100.0, 75.0);
/// ```
/// 
/// draw the rect in update
/// ```rust
/// rect.draw(&draw, &camera).unwrap();
/// ```
pub struct Shader {
    pub transform: Transform,
    object_data: ObjectData,
    object: Box<dyn Object>,
}

impl Shader {
    // create a new rect Shader
    pub fn rect() -> Result<Self, String> {
        // create the data that is used to create 
        // the transform buffer in the shader
        let object_data = ObjectData::default();

        let mut rect = Rect::new();
        rect.add(&object_data);
        rect.load()?;

        let component = Self {
            object: Box::new(rect),
            object_data, 
            transform: Transform::default(),
        };

        Ok(component)
    }

    // create a new circle Shader
    pub fn circle() -> Result<Self, String> {
        // create the data that is used to create 
        // the transform buffer in the shader
        let object_data = ObjectData::default();

        let mut circle = Circle::new();
        circle.add(&object_data);
        circle.load()?;

        let component = Self {
            object: Box::new(circle),
            object_data, 
            transform: Transform::default(),
        };

        Ok(component)
    }

    // create a new texture Shader
    pub fn texture(image: &Image) -> Result<Self, String> {
        // create the data that is used to create 
        // the transform buffer in the shader
        let mut object_data = ObjectData::default();
        object_data.dim = (image.width() as f32, image.height() as f32);

        let mut texture = Texture::new(image);
        texture.add(&object_data);
        texture.load()?;

        let component = Self {
            object: Box::new(texture),
            object_data, 
            transform: Transform::default(),
        };

        Ok(component)
    }

    // create a new text Shader
    pub fn text(text: &str, font: &Font, font_size: usize, color: &Color) -> Result<Self, String> {
        // create the text as rgba image
        let image = font.snapshot(text, font_size as f32)?;
        let image = Image::from(image);

        // create the data that is used to create 
        // the transform buffer in the shader
        let mut object_data = ObjectData::default();
        object_data.dim.0 = image.width() as f32;
        object_data.dim.1 = image.height() as f32;
        object_data.color = color.clone();

        let mut text = Text::new(&image)?;
        text.add(&object_data);
        text.load()?;

        let component = Self {
            object: Box::new(text),
            object_data, 
            transform: Transform::default(),
        };

        Ok(component)
    }

    // draw the Shader to the screen
    pub fn draw(&mut self, draw: &Draw, camera: &Transform) -> Result<(), String> {
        self.object.draw(draw, camera, &self.transform)?;
        Ok(())
    } 

    // set the width and the height of the Shader
    pub fn set_dim(&mut self, width: f32, height: f32) {
        self.object_data.dim.0 = width;
        self.object_data.dim.1 = height;
        self.object.set(0, &self.object_data);
        self.object.set_state(ObjectState::Reload);
    }

    // get the width and the height of the Shader
    pub fn dim(&self) -> (f32, f32) {
        self.object_data.dim
    }

    // set the width of the Shader
    pub fn set_width(&mut self, width: f32) {
        self.object_data.dim.0 = width;
        self.object.set(0, &self.object_data);
        self.object.set_state(ObjectState::Reload);
    }

    // get the width of the Shader
    pub fn width(&self) -> f32 {
        self.object_data.dim.0
    }

    // set the height of the Shader
    pub fn set_height(&mut self, height: f32) {
        self.object_data.dim.1 = height;
        self.object.set(0, &self.object_data);
        self.object.set_state(ObjectState::Reload);
    }

    // get the height of the Shader
    pub fn height(&self) -> f32 {
        self.object_data.dim.1
    }

    // set the color of the Shader
    pub fn set_color(&mut self, color: &Color) {
        self.object_data.color = color.clone();
        self.object.set(0, &self.object_data);
        self.object.set_state(ObjectState::Reload);
    }

    // get the color of the Shader
    pub fn color(&self) -> Color {
        self.object_data.color
    }

    // set the opacity of the Shader
    pub fn set_opacity(&mut self, opacity: f32) {
        self.object_data.opacity = opacity;
        self.object.set(0, &self.object_data);
        self.object.set_state(ObjectState::Reload);
    }

    // get the opacity of the Shader
    pub fn opacity(&self) -> f32 {
        self.object_data.opacity
    }

    // set the offset of the Shader
    // the offset os mainly used for 
    // better positioning of rotation
    // or when using instanced drawing
    pub fn set_offset(&mut self, x_offset: f32, y_offset: f32) {
        self.object_data.offset = (x_offset, y_offset);
        self.object.set(0, &self.object_data);
        self.object.set_state(ObjectState::Reload);
    }

    // get the offset data of the Shader
    pub fn offset(&self) -> (f32, f32) {
        self.object_data.offset
    }

    // set the texture coordinate data of the Shader
    pub fn set_texcoord(&mut self, texcoord: TextureCoordinate) {
        self.object_data.texcoord = texcoord;
        self.object.set(0, &self.object_data);
        self.object.set_state(ObjectState::Reload);
    }

    // get the texture coordinate data of the Shader
    pub fn texcoord(&self) -> TextureCoordinate {
        self.object_data.texcoord
    }
}

// implement collision on 
// both Shader types
impl Collision for Shader {
    fn collides(&self, x: f32, y: f32) -> bool {
        let (tx, ty, _) = self.transform.pos();
        let (x_offset, y_offset) = self.offset();
        let (width, height) = self.dim();
        collision::point_in_rect(x, y, tx + x_offset, ty + y_offset, width, height)
    }
}