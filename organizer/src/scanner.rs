use std::path::{Path, PathBuf};
use folder::scan;
//use dir library to get the home directory of the user 
use dirs;

fn serach_main_folders() -> Option<Vec<PathBuf>> {
    // scan the current user's home directory for the main folders (Documents, Downloads, Pictures)

    let home_dir = dirs::home_dir().unwrap();

    // we are going to find documents, downloads, pictures, music, videos, desktop and public folders in the home directory
    let main_folders = vec!["Documents", "Downloads", "Pictures", "Music", "Videos", "Desktop", "Public"];
    let mut found_folders = Vec::new();

    for folder in main_folders {
        let folder_path = home_dir.join(folder);
        if folder_path.exists() {
            println!("Found folder: {}", folder_path.display());
            found_folders.push(folder_path);
        } else {
            eprintln!("Folder not found: {}", folder_path.display());
        }
    }

    if found_folders.is_empty() {
        None
    } else {
        Some(found_folders)
    } 
}


// now we review the files one by one in the found folders and classify them into different types of files (documents, images, videos, music, etc.) and move them to the corresponding folder. We will use the scan function from the folder library to scan the folders and get the files. We will then use the file extension to classify the files and move them to the corresponding folder. We will also create a log file to keep track of the files that have been moved and their new location.

fn classify_folders() {
    if let Some(folders) = serach_main_folders() {
        for folder in folders {
            let files = scan(&folder);
            for file in files {
                // classify the file based on its extension and move it to the corresponding folder
                // we will implement this logic later
            }
        }
    } else {
        eprintln!("No main folders found in the home directory.");
    }
}
