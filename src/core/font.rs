use std::collections::HashMap;
use freetype::{Face,Library,face::LoadFlag};

/*
Font holds the hashmap that connects a char 
to an image of a char
*/
pub struct Font {
    cache: HashMap<char, image::RgbImage>,
    face: Option<Face>,
}

impl Font {
    // create a new font struct
    pub fn new() -> Font {
        Font {
            cache: HashMap::new(),
            face: None,
        }
    }

    /*
    load a ttf file and convert it
    to a hashmap with letter to bitmap
    */
    pub fn load_ttf(&mut self, path: &str) -> Result<(), freetype::Error> {
        let lib = Library::init()?;
        let face = lib.new_face(path, 0)?;
        face.set_char_size(40 * 64, 0, 50, 0)?;
        self.face = Some(face);
        Ok(())
    }

    /*
    get a character from the loaded truetype font
    uses a cache system so we dont have to 
    crop images out every time
    */
    pub fn char(&self, c: char) -> Result<image::RgbImage, String> {
        if let Some(cached) = self.cache.get(&c) {
            return Ok(*cached);
        } 

        match self.face {
            Some(face) => {
                if let Ok(loaded) = face.load_char(c as usize, LoadFlag::RENDER) {
                    let glyph = face.glyph();
                    let bmap = glyph.bitmap();
                    self.cache.insert(c, bmap);
                    return Ok(bmap);
                } 
            },
            None => return Err("font has not bee loaded".to_string()),
        }
       
        Err(format!("could not get char '{}' from truetype font", c))
    }
}