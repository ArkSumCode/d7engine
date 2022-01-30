use std::fs::{create_dir, File};
use std::io::{Write, Read};
use std::path::{PathBuf, Path};
use directories::BaseDirs;

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
        let path = appdata()?;
        let mut path = PathBuf::from(&path);
        
        // create folders %appdata%\d7engine\<title>
        update_folder(&mut path, "d7engine")?;
        update_folder(&mut path, &self.title)?;

        let path = path_as_string(path.as_path())?;
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
            let path = path_as_string(path.as_path())?;

            // create the filestream
            if let Ok(mut open) = File::create(&path) {
                // write to the filestream
                if let Ok(_) = open.write_all(text.as_bytes()) {
                    return Ok(())
                }
            } 

            return Err(format!("could not write file {}.", file));
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
            let path = path_as_string(path.as_path())?;
            // read the file
            let text = read(path)?;
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
    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }
}

// read a file
pub fn read(file: String) -> Result<String, String> {
    // open one file
    if let Ok(mut file) = File::open(&file) {
        let mut contents = String::new();

        // put the file into a string
        if let Ok(_) = file.read_to_string(&mut contents) {
            return Ok(contents)
        }
    }

    Err(format!("could not read file {}.", file))
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
fn path_as_string(path: &Path) -> Result<String, String> {
    if let Some(formatted) = path.to_str() {
        return Ok(String::from(formatted))
    }

    Err(String::from("could not get path as string."))
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