use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
use directories::BaseDirs;

pub mod installation;

// read a file
pub fn read(file: &str) -> Result<String, String> {
    // open one file
    if let Ok(mut file) = File::open(file) {
        let mut contents = String::new();

        // put the file into a string
        if let Ok(_) = file.read_to_string(&mut contents) {
            return Ok(contents)
        }
    }

    Err(format!("Could not read file {}.", file))
}

// write a file
pub fn write(file: &str, data: &str) -> Result<(), String> {
    // create the filestream
    if let Ok(mut open) = File::create(file) {
        // write to the filestream
        if let Ok(_) = open.write_all(data.as_bytes()) {
            return Ok(())
        }
    } 

    Err(format!("Could not write file {}.", file))
}

// returns os string to appdata
fn appdata() -> Result<String, String> {
    /*
    Linux:   /home/markus/.config
    Windows: C:\Users\Markus\AppData\Roaming
    macOS:   /Users/Markus/Library/Application Support
    */

    if let Some(base_dirs) = BaseDirs::new() {
        let path = base_dirs.config_dir();

        if path.exists() {
            let formatted = path_as_string(path)?;
            return Ok(formatted)
        } 
    } 

    Err(String::from("os appdata path not found."))   
}

// turns Path struct into a string
pub fn path_as_string(path: &Path) -> Result<String, String> {
    if let Some(formatted) = path.to_str() {
        return Ok(String::from(formatted))
    }

    Err(String::from("could not get path as string."))
}

