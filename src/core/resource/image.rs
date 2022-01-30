use image;

// this struct image holds an image data
// its part of the resouces system
// impartant is that we dont load the same image multiple time for 
// performance reseasons

pub struct Image {
    data: Box<image::RgbaImage>,
}

impl Image {
    pub fn new(path: &str) -> Result<Image, String> {
        let data = Image::data(path)?; 
        Ok(Image{data: Box::new(data)})
    }

    // convert the image to an image::RgbaImage which can be used 
    // for the shaderbuffer
    pub fn to_rgba_image(&self) -> image::RgbaImage {
        let container = self.data.as_raw();
        let mut buffer = image::ImageBuffer::new(self.data.width(), self.data.height());
        buffer.clone_from_slice(container);
        buffer
    }

    // get data from an image file
    pub fn data(path: &str) -> Result<image::RgbaImage, String> {
        if let Ok(data) = image::open(path) {
            // need to flip because opengl starts bottom left
            let flipped = data.flipv();
            return Ok(flipped.to_rgba8())
        }
    
        Err(format!("could not open image '{}'", path))
    }

    // crop an image out of another image
    pub fn crop(image: &mut image::RgbImage, x: i32, y: i32, width: i32, height: i32) -> image::RgbImage {
        let img = image::imageops::crop(image, x as u32, y as u32, width as u32, height as u32);
        img.to_image()
    }
}

