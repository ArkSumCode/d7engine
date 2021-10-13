use std::fs::{create_dir, File};
use std::io::Write;
use std::path::{PathBuf, Path};
use directories::BaseDirs;

/* 
files system only works with an installation struct, so 
it is save that the %appdata%/d7engine/<title> folders exist
*/
pub struct Installation {
    path: String
}

/*
creates a folder for the game in appdata
call this method first if the game needs some file storage on a user pc
*/
pub fn install(title: &str) -> Result<Installation, String> {
    let path = appdata()?;
    let mut path = PathBuf::from(&path);
    
    // create folders %appdata%\d7engine\<title>
    update_folder(&mut path, "d7engine")?;
    update_folder(&mut path, title)?;

    let path = path_as_string(path.as_path())?;
    Ok(Installation{path: path})
}

// create or overwrite a file in %appdata%/d7engine/<title>
pub fn overwrite(installation: &Installation, file: &str, text: &str) -> Result<(), String> {
    // create pathbuffer and add the file that
    let mut path = PathBuf::from(&installation.path);
    path.push(file);
    let path = path_as_string(path.as_path())?;

    // create the filestream
    if let Ok(mut open) = File::create(&path) {
        // write to the filestream
        if let Ok(_) = open.write_all(text.as_bytes()) {
            return Ok(())
        }
    } 

    Err(format!("could not write file {}.", file))
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