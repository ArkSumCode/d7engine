use std::fs::create_dir;
use std::path::PathBuf;
use crate::*;

/* 
files system only works with an installation struct, so 
it is save that the %appdata%/d7engine/<title> folders exist
*/
pub struct Installation {
    title: String,
    path: Option<String>,
}

impl Installation {
    // create a new empty Installation
    // we put the files in a subfolder of d7engine
    // with the name title
    pub fn new(title: &str) -> Installation {
        Installation {title: title.to_string(), path: None}
    }

    /*
    creates a folder for the game in appdata
    call this method first if the game needs some file storage on a user pc
    */
    pub fn create_folder(&mut self) -> Result<(), String> {
        let path = file::appdata()?;
        let mut path = PathBuf::from(&path);
        
        // create folders %appdata%\d7engine\<title>
        update_folder(&mut path, "d7engine")?;
        update_folder(&mut path, &self.title)?;

        let path = file::path_as_string(path.as_path())?;
        self.path = Some(path);
        Ok(())
    }

    // create or overwrite a file in %appdata%/d7engine/<title>
    pub fn overwrite(&mut self, file: &str, extension: &str, text: &str) -> Result<(), String> {
        // if the folder is not created
        if let None = &self.path {
            // create it now
            self.create_folder()?;
        }

        if let Some(path) = &self.path {
            // create pathbuffer and add the file that
            let mut path = PathBuf::from(&path);
            let file = format!("{}.{}", file, extension);
            path.push(&file);
            let path = file::path_as_string(path.as_path())?;
            return file::write(&path, text);
        }

        Err("could not create folder".to_string())
    }

    // read a file in the installation folder
    // and return the file as a vector of lines (Strings) 
    pub fn read(&self, file: &str, extension: &str) -> Result<Vec<String>, String> {
        if let Some(path) = &self.path {
            // create the path + file
            let mut path = PathBuf::from(&path);
            let file = format!("{}.{}", file, extension);
            path.push(&file);
            let path = file::path_as_string(path.as_path())?;
            // read the file
            let text = file::read(&path)?;
            // convert the text in the file to a line (String) vector
            let collection = text.lines().collect::<Vec<&str>>();
            let mut lines = vec![];
            // convert &str to String
            for line in collection {
                lines.push(line.to_string());
            }
            
            return Ok(lines);
        }

        Err(format!("could not read file {}.{}", file, extension))
    }

    // returns the path of the installation
    pub fn path(&self) -> Result<PathBuf, String> {
        if let Some(path) = &self.path {
            let buffer = PathBuf::from(&path);
            return Ok(buffer);
        }

        Err("No installation path.".to_string())
    }
}

// create a folder for a path if not existent
fn update_folder(path: &mut PathBuf, sub: &str) -> Result<(), String> {
    path.push(sub);

    if !path.exists() {
        if let Err(_) = create_dir(path) {
           return Err(String::from("could not create directory."));
        };
    } 

    Ok(())
}