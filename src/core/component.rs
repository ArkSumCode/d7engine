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
        self.object.remove(0);
        self.object.add(&self.component_data);
        self.object.set_state(object::ObjectState::Reload);
    }

    // get the width and the height of the component
    pub fn dim(&self) -> (f32, f32) {
        self.component_data.dim
    }

    // set the width of the component
    pub fn set_width(&mut self, width: f32) {
        self.component_data.dim.0 = width;
        self.object.remove(0);
        self.object.add(&self.component_data);
        self.object.set_state(object::ObjectState::Reload);
    }

    // get the width of the component
    pub fn width(&self) -> f32 {
        self.component_data.dim.0
    }

    // set the height of the component
    pub fn set_height(&mut self, height: f32) {
        self.component_data.dim.1 = height;
        self.object.remove(0);
        self.object.add(&self.component_data);
        self.object.set_state(object::ObjectState::Reload);
    }

    // get the height of the component
    pub fn height(&self) -> f32 {
        self.component_data.dim.1
    }

    // set the color of the component
    pub fn set_color(&mut self, color: &Color) {
        self.component_data.color = color.clone();
        self.object.remove(0);
        self.object.add(&self.component_data);
        self.object.set_state(object::ObjectState::Reload);
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
        self.object.remove(0);
        self.object.add(&self.component_data);
        self.object.set_state(object::ObjectState::Reload);
    }

    // get the offset data of the component
    pub fn offset(&self) -> (f32, f32) {
        self.component_data.offset
    }

    // set the texture coordinate data of the component
    pub fn set_texcoord(&mut self, texcoord: object::TextureCoordinate) {
        self.component_data.texcoord = texcoord;
        self.object.remove(0);
        self.object.add(&self.component_data);
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

/*
while the Component is used for drawing single 
elements to the screen 
the instanced component is used to 
draw a single element multiple times at once 
with different transformations
*/
pub struct InstancedComponent {
    pub transform: Transform,
    component_data: Vec<ComponentData>,
    object: Box<dyn object::Object>,
    state: InstancedComponentState,
} 

impl InstancedComponent {
    // create a new rect component
    pub fn rect() -> Result<Self, String> {
        let rect = object::rect::Rect::new();

        let component = Self {
            object: Box::new(rect),
            component_data: vec![], 
            transform: Transform::default(),
            state: InstancedComponentState::NotLoaded,
        };

        Ok(component)
    }

    // create a new texture component
    pub fn texture(image: &Image) -> Result<Self, String> {
        let texture = object::texture::Texture::new(image);

        let component = Self {
            object: Box::new(texture),
            component_data: vec![], 
            transform: Transform::default(),
            state: InstancedComponentState::NotLoaded,
        };

        Ok(component)
    }

    // create a new text component
    pub fn text(text: &str, font: &Font, font_size: i32) -> Result<Self, String> {
        // create the text as rgba image
        let image = font.snapshot(text, font_size as f32)?;
        let image = Image::from(image);

        let text = object::text::Text::new(&image)?;

        let component = Self {
            object: Box::new(text),
            component_data: vec![], 
            transform: Transform::default(),
            state: InstancedComponentState::NotLoaded,
        };

        Ok(component)
    }

    // add a new Component Data
    // this will create a new instance within the object 
    // of the InstancedComponent
    pub fn add(&mut self, component_data: &ComponentData) {
        self.component_data.push(component_data.clone());
        self.object.add(component_data);
        self.object.set_state(object::ObjectState::Reload);
    }

    // remove a Component Data
    // this will remove an instance within the object
    // of the InstancedComponent
    pub fn remove(&mut self, i: usize) -> Result<(), String> {
        self.index_oob(i)?;
        self.component_data.remove(i);
        self.object.remove(i);
        self.object.set_state(object::ObjectState::Reload);
        Ok(())
    }

    // loads the object with the model data
    // call this after adding all the transform/component data
    pub fn load(&mut self) -> Result<(), String> {
        self.object.load()?;
        self.state = InstancedComponentState::Ok;
        Ok(())
    }

    // draw the component to the screen
    pub fn draw(&mut self, draw: &Draw, camera: &Transform) -> Result<(), String> {
        match self.state {
            InstancedComponentState::NotLoaded => return Err("Cannot render without creating the model data. Please call load on the InstancedComponent.".to_string()),
            _ => (),
        }

        self.object.draw(draw, camera, &self.transform)?;
        Ok(())
    } 

    // set the width and the height of a transform data i of the component
    pub fn set_dim(&mut self, i: usize, width: f32, height: f32) -> Result<(), String> {
        self.index_oob(i)?;
        self.component_data[i].dim.0 = width;
        self.component_data[i].dim.1 = height;
        self.object.remove(i);
        self.object.add(&self.component_data[i]);
        self.object.set_state(object::ObjectState::Reload);
        Ok(())
    }

    // get the width and the height of a transform data i the component
    pub fn dim(&self, i: usize) -> Result<(f32, f32), String> {
        self.index_oob(i)?;
        Ok(self.component_data[i].dim)
    }

    // set the width of a transform data i of the component
    pub fn set_width(&mut self, i: usize, width: f32) -> Result<(), String> {
        self.index_oob(i)?;
        self.component_data[i].dim.0 = width;
        self.object.remove(i);
        self.object.add(&self.component_data[i]);
        self.object.set_state(object::ObjectState::Reload);
        Ok(())
    }

    // get the width of a transform data i of the component
    pub fn width(&self, i: usize) -> Result<f32, String> {
        self.index_oob(i)?;
        Ok(self.component_data[i].dim.0)
    }

    // set the height of a transform data i of the component
    pub fn set_height(&mut self, i: usize, height: f32) -> Result<(), String> {
        self.index_oob(i)?;
        self.component_data[i].dim.1 = height;
        self.object.remove(i);
        self.object.add(&self.component_data[i]);
        self.object.set_state(object::ObjectState::Reload);
        Ok(())
    }

    // get the height of a transform data i of the component
    pub fn height(&self, i: usize) -> Result<f32, String> {
        self.index_oob(i)?;
        Ok(self.component_data[i].dim.1)
    }

    // set the color of a transform data i of the component
    pub fn set_color(&mut self, i: usize, color: &Color) -> Result<(), String> {
        self.index_oob(i)?;
        self.component_data[i].color = color.clone();
        self.object.remove(i);
        self.object.add(&self.component_data[i]);
        self.object.set_state(object::ObjectState::Reload);
        Ok(())
    }

    // get the color of a transform data i of the component
    pub fn color(&self, i: usize) -> Result<Color, String> {
        self.index_oob(i)?;
        Ok(self.component_data[i].color)
    }

    // set the opacity of a transform data i of the component
    pub fn set_opacity(&mut self, i: usize, opacity: f32) -> Result<(), String> {
        self.index_oob(i)?;
        self.component_data[i].opacity = opacity;
        self.object.remove(i);
        self.object.add(&self.component_data[i]);
        self.object.set_state(object::ObjectState::Reload);
        Ok(())
    }

    // get the opacity of a transform data i of the component
    pub fn opacity(&self, i: usize) -> Result<f32, String> {
        self.index_oob(i)?;
        Ok(self.component_data[i].opacity)
    }

    // set the offset of a transform data i of the component
    // the offset os mainly used for 
    // better positioning of rotation
    // or when using instanced drawing
    pub fn set_offset(&mut self, i: usize, x_offset: f32, y_offset: f32) -> Result<(), String> {
        self.index_oob(i)?;
        self.component_data[i].offset = (x_offset, y_offset);
        self.object.remove(i);
        self.object.add(&self.component_data[i]);
        self.object.set_state(object::ObjectState::Reload);
        Ok(())
    }

    // get the offset of transform data i of the component
    pub fn offset(&self, i: usize) -> Result<(f32, f32), String> {
        self.index_oob(i)?;
        Ok(self.component_data[i].offset)
    }

      // set the texture coordinate of transform data i of the component
      pub fn set_texcoord(&mut self, i: usize, texcoord: object::TextureCoordinate) -> Result<(), String> {
        self.index_oob(i)?;
        self.component_data[i].texcoord = texcoord;
        self.object.remove(i);
        self.object.add(&self.component_data[i]);
        self.object.set_state(object::ObjectState::Reload);
        Ok(())
    }

    // get the texture coordinate of transform data i of the component
    pub fn texcoord(&self, i: usize) -> Result<object::TextureCoordinate, String> {
        self.index_oob(i)?;
        Ok(self.component_data[i].texcoord)
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
        for i in 0..self.component_data.len() {
            let collides = self.instance_collides(i, x, y)?;
            if collides {
                return Ok(Some(i));
            }
        }

        Ok(None)
    }

    // checks if a item is in the 
    // component data vector
    fn index_oob(&self, i: usize) -> Result<(), String> {
        if i >= self.component_data.len() {
            Err(format!("Component Data with index '{}' not found.", i))
        } else {
            Ok(())
        }
    }
}

// this enum will 
// help in not rendering something because you forgot the
// load method
enum InstancedComponentState {
    NotLoaded,
    Ok,
}

// helps to store components in your struct
pub type ComponentContainer = HashMap<String, Component>;
pub type InstancedComponentContainer = HashMap<String, InstancedComponent>;

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