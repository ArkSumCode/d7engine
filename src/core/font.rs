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
                    // width and height of the new bitmap
                    const WIDTH: usize = 32;
                    const HEIGHT: usize = 24;
                    // get the chars instances and make a new empty ImageBuffer
                    let glyph = face.glyph();
                    let bitmap = glyph.bitmap();
                    let mut image: image::RgbaImage = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
                    // p and q are needed to calculate the offset in the buffer
                    let mut p = 0;
                    let mut q = 0;
                    // calculate values for the rectangle we need
                    let x = glyph.bitmap_left() as usize;
                    let y = HEIGHT - glyph.bitmap_top() as usize;
                    let w = bitmap.width() as usize;
                    let x_max = x + w;
                    let y_max = y + bitmap.rows() as usize;
                    // run through the cols 
                    for i in x .. x_max {
                        let mut j = y_max - 1;
                        // run through the rows backwards because images for opengl have to be upsidedown
                        loop {
                            if i < WIDTH && j < HEIGHT {
                                // calculate byte in the buffer -> returns u8 single value color
                                // 255 -> a pixel that is in the letter
                                // 0 -> a pixel that is outside of the letter
                                let result = bitmap.buffer()[q * w + p];
                                // set the pixel in the new ImageBuffer
                                if result == 0 {
                                    *image.get_pixel_mut(i as u32, j as u32) = image::Rgba([0, 0, 0, 0]);
                                } else if result < 69 {
                                    *image.get_pixel_mut(i as u32, j as u32) = image::Rgba([255, 255, 255, 77]);
                                } else {
                                    *image.get_pixel_mut(i as u32, j as u32) = image::Rgba([255, 255, 255, 255]);
                                }
                               
                                q += 1;
                            }
                            
                            // exit condition for the run thorugh the cols
                            if j - 1 == 0 || j - 1 < y {
                                break;
                            }

                            j = j - 1;
                        }
                       
                        q = 0;
                        p += 1;
                    }
                    // image.save(format!("{}.png", c).as_str()).unwrap();
                    self.cache.insert(c, image.clone());
                    return Ok(image);
                } 
            },
            None => return Err("font has not bee loaded".to_string()),
        }
       
        Err(format!("could not get char '{}' from truetype font", c))
    }
}