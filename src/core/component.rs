use crate::*;
use object::Object;

/*
A d7engine component 
can be used throughout your program
so you can put these together in an 
Vec<Component>
*/
pub struct Component {
    pub transform: Transform,
    component_data: ComponentData,
    object: Box<dyn object::Object>,
}

impl Component {
    // create a new rect component
    pub fn rect() -> Result<Self, String> {
        // create the data that is used to create 
        // the transform buffer in the shader
        let component_data = ComponentData::default();

        let mut rect = object::rect::Rect::new();
        rect.add(&component_data);
        rect.load()?;

        let component = Self {
            object: Box::new(rect),
            component_data, 
            transform: Transform::default(),
        };

        Ok(component)
    }

    // create a new circle component
    pub fn circle() -> Result<Self, String> {
        // create the data that is used to create 
        // the transform buffer in the shader
        let component_data = ComponentData::default();

        let mut circle = object::circle::Circle::new();
        circle.add(&component_data);
        circle.load()?;

        let component = Self {
            object: Box::new(circle),
            component_data, 
            transform: Transform::default(),
        };

        Ok(component)
    }

    // create a new texture component
    pub fn texture(image: &Image) -> Result<Self, String> {
        // create the data that is used to create 
        // the transform buffer in the shader
        let mut component_data = ComponentData::default();
        component_data.dim = (image.width() as f32, image.height() as f32);

        let mut texture = object::texture::Texture::new(image);
        texture.add(&component_data);
        texture.load()?;

        let component = Self {
            object: Box::new(texture),
            component_data, 
            transform: Transform::default(),
        };

        Ok(component)
    }

    // create a new text component
    pub fn text(text: &str, font: &Font, font_size: usize, color: &Color) -> Result<Self, String> {
        // create the text as rgba image
        let image = font.snapshot(text, font_size as f32)?;
        let image = Image::from(image);

        // create the data that is used to create 
        // the transform buffer in the shader
        let mut component_data = ComponentData::default();
        component_data.dim.0 = image.width() as f32;
        component_data.dim.1 = image.height() as f32;
        component_data.color = color.clone();

        let mut text = object::text::Text::new(&image)?;
        text.add(&component_data);
        text.load()?;

        let component = Self {
            object: Box::new(text),
            component_data, 
            transform: Transform::default(),
        };

        Ok(component)
    }

    // draw the component to the screen
    pub fn draw(&mut self, draw: &Draw, camera: &Transform) -> Result<(), String> {
        self.object.draw(draw, camera, &self.transform)?;
        Ok(())
    } 

    // set the width and the height of the component
    pub fn set_dim(&mut self, width: f32, height: f32) {
        self.component_data.dim.0 = width;
        self.component_data.dim.1 = height;
        self.object.set(0, &self.component_data);
        self.object.set_state(object::ObjectState::Reload);
    }

    // get the width and the height of the component
    pub fn dim(&self) -> (f32, f32) {
        self.component_data.dim
    }

    // set the width of the component
    pub fn set_width(&mut self, width: f32) {
        self.component_data.dim.0 = width;
        self.object.set(0, &self.component_data);
        self.object.set_state(object::ObjectState::Reload);
    }

    // get the width of the component
    pub fn width(&self) -> f32 {
        self.component_data.dim.0
    }

    // set the height of the component
    pub fn set_height(&mut self, height: f32) {
        self.component_data.dim.1 = height;
        self.object.set(0, &self.component_data);
        self.object.set_state(object::ObjectState::Reload);
    }

    // get the height of the component
    pub fn height(&self) -> f32 {
        self.component_data.dim.1
    }

    // set the color of the component
    pub fn set_color(&mut self, color: &Color) {
        self.component_data.color = color.clone();
        self.object.set(0, &self.component_data);
        self.object.set_state(object::ObjectState::Reload);
    }

    // get the color of the component
    pub fn color(&self) -> Color {
        self.component_data.color
    }

    // set the opacity of the component
    pub fn set_opacity(&mut self, opacity: f32) {
        self.component_data.opacity = opacity;
        self.object.set(0, &self.component_data);
        self.object.set_state(object::ObjectState::Reload);
    }

    // get the opacity of the component
    pub fn opacity(&self) -> f32 {
        self.component_data.opacity
    }

    // set the offset of the component
    // the offset os mainly used for 
    // better positioning of rotation
    // or when using instanced drawing
    pub fn set_offset(&mut self, x_offset: f32, y_offset: f32) {
        self.component_data.offset = (x_offset, y_offset);
        self.object.set(0, &self.component_data);
        self.object.set_state(object::ObjectState::Reload);
    }

    // get the offset data of the component
    pub fn offset(&self) -> (f32, f32) {
        self.component_data.offset
    }

    // set the texture coordinate data of the component
    pub fn set_texcoord(&mut self, texcoord: object::TextureCoordinate) {
        self.component_data.texcoord = texcoord;
        self.object.set(0, &self.component_data);
        self.object.set_state(object::ObjectState::Reload);
    }

    // get the texture coordinate data of the component
    pub fn texcoord(&self) -> object::TextureCoordinate {
        self.component_data.texcoord
    }
}

/*
the default trait is used to create empty
data
we need that because we load data afterwards
*/
pub trait Default {
    fn default() -> Self;
}

/*
holds data that is used to insert into transformation 
buffer of the shader
*/
#[derive(Clone, Copy)]
pub struct ComponentData {
    pub color: Color,
    pub dim: (f32, f32),
    pub opacity: f32,
    pub offset: (f32, f32),
    pub texcoord: object::TextureCoordinate,
}

impl Default for ComponentData {
    // create empty component data
    fn default() -> Self {
        Self {
            color: Color::default(),
            dim: (0.0, 0.0),
            opacity: 1.0,
            offset: (0.0, 0.0),
            texcoord: [   
                0.0, 0.0,
                0.0, 1.0,
                1.0, 1.0,            
                1.0, 0.0,
            ],
        }
    }
}

// implement collision on 
// both component types
impl Collision for Component {
    fn collides(&self, x: f32, y: f32) -> bool {
        let (tx, ty, _) = self.transform.pos();
        let (x_offset, y_offset) = self.offset();
        let (width, height) = self.dim();
        collision::point_in_rect(x, y, tx + x_offset, ty + y_offset, width, height)
    }
}

use std::collections::HashMap;

// helps to store components in your struct
pub struct ComponentContainer {
    map: HashMap<String, Component>,
}

impl ComponentContainer {
    // standard Hashmap function
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }
    // standard Hashmap function
    pub fn insert(&mut self, key: &str, value: Component) {
        self.map.insert(key.to_string(), value);
    }
    // standard Hashmap function
    pub fn get(&self, key: &str) -> Option<&Component> {
        self.map.get(key)
    }
    // standard Hashmap function
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Component> {
        self.map.get_mut(key)
    }
    // adds drawing for loop for cleaner user code
    // draws all components in map
    pub fn draw(&mut self, draw: &Draw, camera: &Transform) -> Result<(), String> {
        for (_, component) in &mut self.map {
            component.draw(draw, camera)?;
        }

        Ok(())
    }
}
