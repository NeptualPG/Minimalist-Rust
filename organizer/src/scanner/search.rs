//use dir library to get the home directory of the user 
use dirs;
use crate::scanner::mainfolders::MainFolders;
use std::fs;
use std::path::{Path, PathBuf};


pub fn search_main_folders() -> Option<MainFolders> {
    // scan the current user's home directory for the main folders (Documents, Downloads, Pictures)

    let home_dir = dirs::home_dir().unwrap();
    let mut found_folders: MainFolders = MainFolders::new();
    // we are going to find documents, downloads, pictures, music, videos, desktop and public folders in the home directory
    let main_folders = vec!["Documents", "Downloads", "Pictures", "Music", "Videos", "Desktop", "Public"];

    for folder in main_folders {
        let folder_path = home_dir.join(folder);
        if folder_path.exists() {
            found_folders.folders.insert(folder.to_string(), vec![folder_path]);
            found_folders.active_folder.push(Some(false));
        } else {
            println!("Folder {} not found in home directory", folder);
        }
    }

    if found_folders.is_empty() {
        None
    } else {
        Some(found_folders)
    }
}


fn find_folder(root: &Path, target: &str) -> Option<PathBuf> {
    let mut result = Vec::new();

    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(folder_name) = path.file_name().and_then(|n| n.to_str()) {
                    if folder_name.eq_ignore_ascii_case(target) {
                        result.push(path.clone());
                    }
                }
                if let Some(found) = find_folder(&path, target) {
                    result.push(found);
                }
            }
        }
    }
    if result.is_empty() {
        None
    } else {
        Some(result[0].clone())
    }
}

pub fn search_specific_folder(folder_name: &str) -> Option<MainFolders> {
    // scan the current user's home directory for a specific folder (Documents, Downloads, Pictures)
    let home_dir = dirs::home_dir().unwrap();
    let mut found_folders: MainFolders = MainFolders::new();
    if let Some(folder_path) = find_folder(&home_dir, folder_name) {
        found_folders.folders.insert(folder_name.to_string(), vec![folder_path]);
        found_folders.active_folder.push(Some(false));
    } else {
        println!("Folder {} not found in home directory", folder_name);
    }
    if found_folders.is_empty() {
        None
    } else {
        Some(found_folders)
    }
}