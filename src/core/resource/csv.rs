use crate::prelude::*;

// holds the data of a csv file
pub struct Csv {
    data: Vec<HashMap<String, String>>
}

impl Csv {
    // creates a empty csv struct
    pub fn new() -> Csv {
        Csv {
            data: vec![]
        }
    }

    pub fn read_file(&mut self, path: &str) {
        
    } 
}