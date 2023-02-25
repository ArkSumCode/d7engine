use crate::core::shader::object::{Object, ObjectState};
use crate::core::*;
use crate::core::shader::object::{circle::Circle, rect::Rect, text::Text, texture::Texture};
use crate::core::color::Color;
use crate::core::resource::font::Font;
use crate::core::math::collision;
use crate::core::shader::object::TextureCoordinate;

// this enum will 
// help in not rendering something because you forgot the
// load method
enum InstancedComponentState {
    NotLoaded,
    Ok,
}

/*
while the Component is used for drawing single 
elements to the screen 
the instanced component is used to 
draw a single element multiple times at once 
with different transformations
*/
pub struct InstancedShader {
    pub transform: Transform,
    object_data: Vec<ObjectData>,
    object: Box<dyn Object>,
    state: InstancedComponentState,
} 

impl InstancedShader {
    // create a new rect InstancedShader
    pub fn rect() -> Result<Self, String> {
        let rect = Rect::new();

        let component = Self {
            object: Box::new(rect),
            object_data: vec![], 
            transform: Transform::default(),
            state: InstancedComponentState::NotLoaded,
        };

        Ok(component)
    }

    // create a new circle InstancedShader
    pub fn circle() -> Result<Self, String> {
        let circle = Circle::new();

        let component = Self {
            object: Box::new(circle),
            object_data: vec![], 
            transform: Transform::default(),
            state: InstancedComponentState::NotLoaded,
        };

        Ok(component)
    }

    // create a new texture InstancedShader
    pub fn texture(image: &Image) -> Result<Self, String> {
        let texture = Texture::new(image);

        let component = Self {
            object: Box::new(texture),
            object_data: vec![], 
            transform: Transform::default(),
            state: InstancedComponentState::NotLoaded,
        };

        Ok(component)
    }

    // create a new text InstancedShader
    pub fn text(text: &str, font: &Font, font_size: i32) -> Result<Self, String> {
        // create the text as rgba image
        let image = font.snapshot(text, font_size as f32)?;
        let image = Image::from(image);

        let text = Text::new(&image)?;

        let component = Self {
            object: Box::new(text),
            object_data: vec![], 
            transform: Transform::default(),
            state: InstancedComponentState::NotLoaded,
        };

        Ok(component)
    }

    // add a new Component Data
    // this will create a new instance within the object 
    // of the InstancedShader
    pub fn add(&mut self, object_data: &ObjectData) {
        self.object_data.push(object_data.clone());
        self.object.add(object_data);
        self.object.set_state(ObjectState::Reload);
    }

    // remove a Component Data
    // this will remove an instance within the object
    // of the InstancedShader
    pub fn remove(&mut self, i: usize) -> Result<(), String> {
        self.index_oob(i)?;
        self.object_data.remove(i);
        self.object.remove(i);
        self.object.set_state(ObjectState::Reload);
        Ok(())
    }

    // loads the object with the model data
    // call this after adding all the transform/component data
    pub fn load(&mut self) -> Result<(), String> {
        self.object.load()?;
        self.state = InstancedComponentState::Ok;
        Ok(())
    }

    // draw the InstancedShader to the screen
    pub fn draw(&mut self, draw: &Draw, camera: &Transform) -> Result<(), String> {
        match self.state {
            InstancedComponentState::NotLoaded => return Err("Cannot render without creating the model data. Please call load on the InstancedComponent.".to_string()),
            _ => (),
        }

        self.object.draw(draw, camera, &self.transform)?;
        Ok(())
    } 

    // set the width and the height of a transform data i of the InstancedShader
    pub fn set_dim(&mut self, i: usize, width: f32, height: f32) -> Result<(), String> {
        self.index_oob(i)?;
        self.object_data[i].dim.0 = width;
        self.object_data[i].dim.1 = height;
        self.object.set(i, &self.object_data[i]);
        self.object.set_state(ObjectState::Reload);
        Ok(())
    }

    // get the width and the height of a transform data i the InstancedShader
    pub fn dim(&self, i: usize) -> Result<(f32, f32), String> {
        self.index_oob(i)?;
        Ok(self.object_data[i].dim)
    }

    // set the width of a transform data i of the InstancedShader
    pub fn set_width(&mut self, i: usize, width: f32) -> Result<(), String> {
        self.index_oob(i)?;
        self.object_data[i].dim.0 = width;
        self.object.set(i, &self.object_data[i]);
        self.object.set_state(ObjectState::Reload);
        Ok(())
    }

    // get the width of a transform data i of the InstancedShader
    pub fn width(&self, i: usize) -> Result<f32, String> {
        self.index_oob(i)?;
        Ok(self.object_data[i].dim.0)
    }

    // set the height of a transform data i of the InstancedShader
    pub fn set_height(&mut self, i: usize, height: f32) -> Result<(), String> {
        self.index_oob(i)?;
        self.object_data[i].dim.1 = height;
        self.object.set(i, &self.object_data[i]);
        self.object.set_state(ObjectState::Reload);
        Ok(())
    }

    // get the height of a transform data i of the InstancedShader
    pub fn height(&self, i: usize) -> Result<f32, String> {
        self.index_oob(i)?;
        Ok(self.object_data[i].dim.1)
    }

    // set the color of a transform data i of the InstancedShader
    pub fn set_color(&mut self, i: usize, color: &Color) -> Result<(), String> {
        self.index_oob(i)?;
        self.object_data[i].color = color.clone();
        self.object.set(i, &self.object_data[i]);
        self.object.set_state(ObjectState::Reload);
        Ok(())
    }

    // get the color of a transform data i of the InstancedShader
    pub fn color(&self, i: usize) -> Result<Color, String> {
        self.index_oob(i)?;
        Ok(self.object_data[i].color)
    }

    // set the opacity of a transform data i of the InstancedShader
    pub fn set_opacity(&mut self, i: usize, opacity: f32) -> Result<(), String> {
        self.index_oob(i)?;
        self.object_data[i].opacity = opacity;
        self.object.set(i, &self.object_data[i]);
        self.object.set_state(ObjectState::Reload);
        Ok(())
    }

    // get the opacity of a transform data i of the InstancedShader
    pub fn opacity(&self, i: usize) -> Result<f32, String> {
        self.index_oob(i)?;
        Ok(self.object_data[i].opacity)
    }

    // set the offset of a transform data i of the InstancedShader
    // the offset os mainly used for 
    // better positioning of rotation
    // or when using instanced drawing
    pub fn set_offset(&mut self, i: usize, x_offset: f32, y_offset: f32) -> Result<(), String> {
        self.index_oob(i)?;
        self.object_data[i].offset = (x_offset, y_offset);
        self.object.set(i, &self.object_data[i]);
        self.object.set_state(ObjectState::Reload);
        Ok(())
    }

    // get the offset of transform data i of the InstancedShader
    pub fn offset(&self, i: usize) -> Result<(f32, f32), String> {
        self.index_oob(i)?;
        Ok(self.object_data[i].offset)
    }

      // set the texture coordinate of transform data i of the InstancedShader
      pub fn set_texcoord(&mut self, i: usize, texcoord: TextureCoordinate) -> Result<(), String> {
        self.index_oob(i)?;
        self.object_data[i].texcoord = texcoord;
        self.object.set(i, &self.object_data[i]);
        self.object.set_state(ObjectState::Reload);
        Ok(())
    }

    // get the texture coordinate of transform data i of the InstancedShader
    pub fn texcoord(&self, i: usize) -> Result<TextureCoordinate, String> {
        self.index_oob(i)?;
        Ok(self.object_data[i].texcoord)
    }

    // collision for an instance
    pub fn instance_collides(&self, i: usize, x: f32, y: f32) -> Result<bool, String> {
        let (tx, ty, _) = self.transform.pos();
        let (x_offset, y_offset) = self.offset(i)?;
        let (width, height) = self.dim(i)?;
        let collides = collision::point_in_rect(x, y, tx + x_offset, ty + y_offset, width, height);
        Ok(collides)
    }

    // get the instance that collides,
    // if one collides
    pub fn collides(&self, x: f32, y: f32) -> Result<Option<usize>, String> {
        for i in 0..self.object_data.len() {
            let collides = self.instance_collides(i, x, y)?;
            if collides {
                return Ok(Some(i));
            }
        }

        Ok(None)
    }

    // checks if a item is in the 
    // InstancedShader data vector
    fn index_oob(&self, i: usize) -> Result<(), String> {
        if i >= self.object_data.len() {
            Err(format!("Component Data with index '{}' not found.", i))
        } else {
            Ok(())
        }
    }
}



