use image;

// this struct image holds an image data
// its part of the resouces system
// impartant is that we dont load the same image multiple time for 
// performance reseasons

#[derive(Clone)]
pub struct Image {
    data: Box<image::RgbaImage>,
    pub width: f32,
    pub height: f32,
}

impl Image {
    pub fn new(path: &str) -> Result<Image, String> {
        let data = Self::data(path)?; 
        let width = data.width() as f32;
        let height = data.height() as f32;
        Ok(Image{
            data: Box::new(data),
            width, height
        })
    }

    // create the image struct from a image::RgbaImage 
    // this will consume the image
    pub fn from(image: image::RgbaImage) -> Self {
        let width = image.width() as f32;
        let height = image.height() as f32;

        Image {
            data: Box::new(image),
            width, height,
        }
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

    // crop an image out of this image
    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Image {
        let mut data = self.to_rgba_image();
        let img = image::imageops::crop(&mut data, x, y, width, height);
        Image{
            data: Box::new(img.to_image()),
            width: width as f32, height: height as f32,
        }
    }
}
