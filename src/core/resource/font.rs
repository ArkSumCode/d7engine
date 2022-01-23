use rusttype::{point, Scale, PositionedGlyph};
use image::{Rgba, ImageBuffer, imageops::flip_vertical};

/*
Font holds the parsed font file
you can create a font using the load_ttf function
then use snapshot to get a bitmap/rgba image of the rendered text

this uses the external library rusttype:
github: https://github.com/redox-os/rusttype/
docs: https://docs.rs/rusttype/0.9.2/rusttype/index.html

// its part of the resouces system
// impartant is that we dont load the same image multiple time for 
// performance reseasons
*/
pub struct Font<'a> {
    font: rusttype::Font<'a>
}

impl<'a> Font<'a> {
    /*
    load a ttf file and convert it
    to the Font struct
    */
    pub fn new_ttf(path: &str) -> Result<Font, String> {
        if let Ok(data) = std::fs::read(path) {
            if let Some(font) = rusttype::Font::try_from_vec(data) {
                return Ok(Font{font});
            }
        }

        Err(format!("failed to open or parse font '{}'.", path))
    }

    /*
    returns an rgba image of a given text
    */
    pub fn snapshot(&self, text: &str, font_size: f32) -> Result<image::RgbaImage, String> {
        // the font size:
        let scale = Scale::uniform(font_size);
        let v_metrics = self.font.v_metrics(scale);

        // layout the glyph in a line with 20 pixels padding
        let glyphs: Vec<PositionedGlyph> = self.font
            .layout(&text, scale, point(0.0, 0.0 + v_metrics.ascent))
            .collect();
        
        // work out the layout size
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        if let Some(glyphs_width) = glyph_width(&glyphs) {
            // create a new rgba image with some random value 10, which does nothing until you set it to low, than it chrashes
            // because out of bounds
            let mut image = ImageBuffer::new(glyphs_width + 10, glyphs_height + 10);

            // loop through the glyphs in the text positioning each one on a line
            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    // draw the glyph into the image per-pixel using the draw closure
                    glyph.draw(|x,y,v| {
                        image.put_pixel(
                            // offset the position by the glyph bounding box
                            x + bounding_box.min.x as u32,
                            y + bounding_box.min.y as u32,
                            // turn the coverage into an alpha value
                            Rgba([255, 255, 255, (v * 255.0) as u8]),
                        )
                    });
                }
            }

            // flip it vertically because opengl needs them upside down
            let image = flip_vertical(&image);
            return Ok(image);
        } 

        Err("could not calculate the glyphs width".to_string())
    }
}

// resolves the results (Options) of operations done on glyphs width calculation
fn glyph_width(glyphs: &Vec<PositionedGlyph>) -> Option<u32> {
    if let Some(first) = glyphs.first() {
        if let Some(last) = glyphs.last() {
            let max_x = last.pixel_bounding_box()?.max.x;
            let min_x = first.pixel_bounding_box()?.min.x;
            return Some((max_x - min_x) as u32);
        }
    }
    
    None
}