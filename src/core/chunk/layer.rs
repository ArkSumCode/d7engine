use crate::core::texture::Texture;

// reference a layer with a more coherent name
pub enum LayerName {
    BACK,
    MIDDLE,
    FRONT,
}

/*
layer that can be referneced whit a Layername
and holds the textures
can be disabeld with active
*/
pub struct Layer {
    pub textures: Vec<Texture>,
    pub active: bool,
}

impl Layer {
    // draws the layer to the screen
    pub fn draw(&self) {
        // loop through all the textures and draw them to the screen
        for texture in &self.textures {
            texture.draw();
        }
    }
}

// create an empty layer array
pub fn empty_layers(active: bool) -> [Layer; 3] {
    [
        Layer{textures: vec![], active}, // background
        Layer{textures: vec![], active}, // middle
        Layer{textures: vec![], active}, // front
    ]
}