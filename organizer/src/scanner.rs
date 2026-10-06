use std::path::{Path, PathBuf};
use folder::scan;
//use dir library to get the home directory of the user 
use dirs;

fn search_main_folders() -> Option<Vec<PathBuf>> {
    // scan the current user's home directory for the main folders (Documents, Downloads, Pictures)

    let home_dir = dirs::home_dir().unwrap();

    // we are going to find documents, downloads, pictures, music, videos, desktop and public folders in the home directory
    let main_folders = vec!["Documents", "Downloads", "Pictures", "Music", "Videos", "Desktop", "Public"];
    let mut found_folders = Vec::new();

    for folder in main_folders {
        let folder_path = home_dir.join(folder);
        if folder_path.exists() {
            println!("Found folder: {}", folder_path.display());
            // thi is to store the found folders in a vector to return them later
            found_folders.push(folder_path);
        } else {
            /// we create the folder if it does not exist
            std::fs::create_dir_all(&folder_path).unwrap();
            println!("Created folder: {}", folder_path.display());
        }
    }

    if found_folders.is_empty() {
        None
    } else {
        Some(found_folders)
    } 
}



