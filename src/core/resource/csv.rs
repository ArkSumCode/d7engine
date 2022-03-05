use crate::prelude::*;

// holds the data of a csv file
pub struct Csv {
    _data: Vec<HashMap<String, String>>
}

impl Csv {
    // creates a empty csv struct
    pub fn new() -> Csv {
        Csv {
            _data: vec![]
        }
    }

    pub fn read_file(&mut self, _path: &str) {
        
    } 
}