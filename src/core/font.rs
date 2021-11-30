use std::collections::HashMap;
use freetype::{Face,Library,face::LoadFlag};

/*
Font holds the hashmap that connects a char 
to an image of a char
*/
pub struct Font {
    cache: HashMap<char, image::RgbaImage>,
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
    pub fn char(&mut self, c: char) -> Result<image::RgbaImage, String> {
        if let Some(cached) = self.cache.get(&c) {
            let img = cached.clone();
            return Ok(img);
        } 

        match &self.face {
            Some(face) => {
                if let Ok(_) = face.load_char(c as usize, LoadFlag::RENDER) {
                    let glyph = face.glyph();
                    let bmap = glyph.bitmap();
                    let buffer = bmap.buffer();
                   
                    if let Some(img) = image::RgbaImage::from_raw(bmap.width() as u32, bmap.width() as u32, Vec::from(buffer)) {
                        self.cache.insert(c, img.clone());
                        return Ok(img);
                    }

                    return Err("font conversion bmap to rbgimage failed".to_string());
                } 
            },
            None => return Err("font has not bee loaded".to_string()),
        }
       
        Err(format!("could not get char '{}' from truetype font", c))
    }
}