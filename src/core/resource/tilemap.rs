use crate::prelude::*;

/*
this struct holds the information that is 
in a tilemap file

this data can be used for instanced texture components
to draw different tiles out of a single texture
*/
#[derive(Clone)]
pub struct TileData {
    pub name: String,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

// helps with code readability 
// of the TileData texcorrd method
#[derive(Clone)]
pub enum TileDataRotation {
    Bottom,
    Left,
    Top,
    Right,
}

impl TileData {
    /*
    create the texture coordinates 
    from the data in the tiles
    Standard TileDataRotation is bottom (no rotation)
    */
    pub fn texcoord(&self, tile_data_rotation: &TileDataRotation) -> object::TextureCoordinate {
        match tile_data_rotation {
            TileDataRotation::Bottom => {
                [
                    self.x1, self.y2,       
                    self.x1, self.y1,       
                    self.x2, self.y1,    
                    self.x2, self.y2,       
                ]
            },
            TileDataRotation::Left => {
                [
                    self.x2, self.y2,  
                    self.x1, self.y2,       
                    self.x1, self.y1,       
                    self.x2, self.y1,    
                ]
            },
            TileDataRotation::Top => {
                [
                    self.x2, self.y1,    
                    self.x2, self.y2,  
                    self.x1, self.y2,       
                    self.x1, self.y1,       
                ]
            },
            TileDataRotation::Right => {
                [
                    self.x1, self.y1,       
                    self.x2, self.y1,    
                    self.x2, self.y2,  
                    self.x1, self.y2,       
                ]
            }
        }
    }
}

pub struct TileMap {
    tiles: Vec<TileData>,
    image: Image,
    pub dim: usize,
}

impl TileMap {
    /*
    create a tilemap struct 
    using the path to the file
    and the dimension of a tile in the texture e.g. 32, 64 etc.

    out data is constructed from the top left
    but texture coordinates are from bottom left
    */
    pub fn new(path: &str, image: Image, dim: usize) -> Result<Self, String> {
        let file = file::read(path)?;
        let image_dim = (image.width() as f32, image.height() as f32);

        let mut tiles = vec![];
      
        for line in file.lines() {
            // read each line and parse the values
            match Self::parse_line(&line) {
                Ok((name, x, y)) => {
                    // create the values between 0.0 and 1.0
                    let x = x as f32;
                    let y = y as f32;
                    let dim = dim as f32;
                    let width = image_dim.0;
                    let height = image_dim.1;

                    let x1 = x / width;            
                    let y1 = 1.0 - (y / height);          
                    let x2 = (x + dim) / width;                   
                    let y2 = 1.0 - ((y + dim) / height);
                    
                    let tile_data = TileData {
                        name, x1, y1, x2, y2
                    };
                    tiles.push(tile_data);
                },
                Err(_) => return Err(format!("Could not parse line '{}'", line)),
            }
        }

        let tilemap = TileMap{tiles, image, dim};
        Ok(tilemap)
    }

    // get a copy of the image
    pub fn image(&self) -> Image {
        self.image.clone()
    }

    // get all the tiledata as a vector
    pub fn as_vec(&self) -> Vec<TileData> {
        self.tiles.clone()
    }

    // get the tiledata of a certain item
    pub fn get(&self, name: &str) -> Result<TileData, String> {
        for item in &self.tiles {
            if item.name == name {
                return Ok(item.clone());
            }
        }

        Err(format!("Could not get item '{}'.", name))
    }

    // returns the values from a line of the tilemap file
    fn parse_line(line: &str) -> Result<(String, usize, usize), std::num::ParseIntError> {
        let parts: Vec<&str> = line.split(',').collect();
        let name = parts[0].to_string();
        let x = parts[1].parse::<usize>()?;
        let y = parts[2].parse::<usize>()?;
        Ok((name, x, y))
    }
}
