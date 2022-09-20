use crate::prelude::*;

/*
A d7engine component 
can be used throughout your program
so you can put these together in an 
Vec<Component>
*/
pub struct Component {
    pub transform: Transform,
    component_data: ComponentData,
    object: Box<dyn Object>,
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

        let component = Component {
            object: Box::new(rect),
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
        component_data.width = image.width() as f32;
        component_data.height = image.height() as f32;

        let mut texture = object::texture::Texture::new(image);
        texture.add(&component_data);
        texture.load()?;

        let component = Component {
            object: Box::new(texture),
            component_data, 
            transform: Transform::default(),
        };

        Ok(component)
    }

    // create a new text component
    pub fn text(text: &str, font: &Font, font_size: i32, color: &Color) -> Result<Self, String> {
        // create the text as rgba image
        let image = font.snapshot(text, font_size as f32)?;
        let image = Image::from(image);

        // create the data that is used to create 
        // the transform buffer in the shader
        let mut component_data = ComponentData::default();
        component_data.width = image.width() as f32;
        component_data.height = image.height() as f32;
        component_data.color = color.clone();

        let mut text = object::text::Text::new(&image)?;
        text.add(&component_data);
        text.load()?;

        let component = Component {
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
    pub fn set(&mut self, width: f32, height: f32) {
        self.component_data.width = width;
        self.component_data.height = height;
        self.object.remove(0);
        self.object.add(&self.component_data);
        self.object.set_state(ObjectState::RELOAD);
    }

    // get the width and the height of the component
    pub fn get(&self) -> (f32, f32) {
        (self.component_data.width, self.component_data.height)
    }

    // set the width of the component
    pub fn set_width(&mut self, width: f32) {
        self.component_data.width = width;
        self.object.remove(0);
        self.object.add(&self.component_data);
        self.object.set_state(ObjectState::RELOAD);
    }

    // get the width of the component
    pub fn width(&self) -> f32 {
        self.component_data.width
    }

    // set the height of the component
    pub fn set_height(&mut self, height: f32) {
        self.component_data.height = height;
        self.object.remove(0);
        self.object.add(&self.component_data);
        self.object.set_state(ObjectState::RELOAD);
    }

    // get the height of the component
    pub fn height(&self) -> f32 {
        self.component_data.height
    }

    // set the color of the component
    pub fn set_color(&mut self, color: &Color) {
        self.component_data.color = color.clone();
        self.object.remove(0);
        self.object.add(&self.component_data);
        self.object.set_state(ObjectState::RELOAD);
    }

    // get the color of the component
    pub fn color(&self) -> Color {
        self.component_data.color
    }

    // set the opacity of the component
    pub fn set_opacity(&mut self, opacity: f32) {
        self.component_data.opacity = opacity;
        self.object.remove(0);
        self.object.add(&self.component_data);
        self.object.set_state(ObjectState::RELOAD);
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
        self.object.remove(0);
        self.object.add(&self.component_data);
        self.object.set_state(ObjectState::RELOAD);
    }

    // get the offset data of the component
    pub fn offset(&self) -> (f32, f32) {
        self.component_data.offset
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
pub struct ComponentData {
    pub color: Color,
    pub width: f32,
    pub height: f32,
    pub opacity: f32,
    pub offset: (f32, f32),
}

impl Default for ComponentData {
    // create empty component data
    fn default() -> Self {
        Self {
            color: Color::default(),
            width: 0.0,
            height: 0.0,
            opacity: 1.0,
            offset: (0.0, 0.0),
        }
    }
}