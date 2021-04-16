use std::collections::HashMap;
use crate::core::*;

/*
Font holds the hashmap that connects a char to an image
*/
pub struct Font {
    dimension: i32,
    spacing: i32,
    chars: HashMap<String, image::RgbImage>,
}

impl Font {
    // create a new font struct
    pub fn new(dimension: i32, spacing: i32) -> Font {
        Font {
            chars: HashMap::new(),
            dimension,
            spacing,
        }
    }

    /* 
    specify the number of rows and cols in the font file
    dimension is the width and height of one cell in px
    */
    pub fn load(&mut self, file: String, rows: u8, cols: u8) {
        // the sequence of the chars from top left to bottom right
        let mut chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890?!".chars();
        let mut image = texture::image_data(&file).unwrap();

        // run through all rows and columns
        for j in 1..(rows + 1) {
            for i in 0..cols {
                // calculate the x and y value of the to cropp image
                let x = i as i32 * self.dimension;
                let y = (rows as i32 * self.dimension) - (j as i32 * self.dimension);
                let cropped = texture::crop_image(&mut image, x, y, self.dimension, self.dimension);
              
                if let Some(c) = chars.next() {
                    self.chars.insert(c.to_string(), cropped);
                }
            }
        }
    }

    // get an image by a character
    pub fn get(&self, character: &str) -> Option<&image::RgbImage> {
        self.chars.get(character)
    }

    // returns the dimension of a character cell
    pub fn dimension(&self) -> i32 {
        self.dimension
    }

    //returns the spacing of the font
    pub fn spacing(&self) -> i32 {
        self.spacing
    }
}