use std::fs;
use dirs_next::home_dir;

pub fn load_file() -> String {
    match home_dir() {
        Some(mut path) => {
            path.push(".scratchnotes");
            path.push("main-content");
            path.set_extension("txt");
            
            match fs::read_to_string(path) {
                Ok(s) => s,
                Err(_e) => String::from(""),
            }
        }

        None => {
            String::from("")
        }
    }
}

/// Saves text file
///
/// Naively assumes path was not found and create one
pub fn save_file(contents: String) {
    match home_dir() {
        Some(mut path) => {
            path.push(".scratchnotes");

            let mut path_with_file = path.clone();
            path_with_file.push("main-content");
            path_with_file.set_extension("txt");
            
            match fs::write(path_with_file.clone(), contents.clone()) {
                Ok(_) => {},
                Err(_e) => {
                    match fs::create_dir(path.clone()) {
                        Ok (_) => {
                            match fs::write(path_with_file, contents) {
                                Ok (_) => {},
                                Err(_) => {},
                            }
                        },
                        Err(_) => {},
                    }
                },
            }
        }

        None => {}
    }
}

