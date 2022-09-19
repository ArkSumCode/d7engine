use image;

// this struct image holds an image data
// its part of the resouces system
// impartant is that we dont load the same image multiple time for 
// performance reseasons

#[derive(Clone)]
pub struct Image {
    data: Box<image::RgbaImage>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(path: &str) -> Result<Image, String> {
        let data = Image::data(path)?; 
        let width = data.width();
        let height = data.height();
        Ok(Image{
            data: Box::new(data),
            width, height
        })
    }

    // create the image struct from a image::RgbaImage 
    // this will consume the image
    pub fn from(image: image::RgbaImage) -> Self {
        let width = image.width();
        let height = image.height();

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
            width, height,
        }
    }

    // returns the width of the image
    pub fn width(&self) -> u32 {
        self.width
    }

    // returns the height of the image
    pub fn height(&self) -> u32 {
        self.height
    }
}
