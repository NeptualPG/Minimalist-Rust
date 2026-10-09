use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderName {
    pub folders: HashMap<String, Vec<PathBuf>>,
    pub active_folder: Vec<Option<bool>>
}

const FOLDER_NAME_FILE: &str = "src/data/folder_name.json";

// CRUD operations for FolderName struct
// [
//     {
//         "name": "documents",
//         "active_folder": true
//     },
//     {
//         "name": "downloads",
//         "active_folder": false
//     },
//     {
//         "name": "pictures",
//         "active_folder": false
//     },
//     {
//         "name": "music",
//         "active_folder": false
//     },
//     {
//         "name": "videos",
//         "active_folder": false
//     } // FORTAMT AND CONTENT OF THE JSON FILE
// ]

pub fn create_folder_name(folder_name: &FolderName) -> Result<(), Box<dyn std
    ::error::Error>> {
        let json = serde_json::to_string_pretty(folder_name)?;
        fs::write(FOLDER_NAME_FILE, json
    )?;
        Ok(())
}

pub fn read_folder_name() -> Result<FolderName, Box<dyn std::error::Error>> {
    let json = fs::read_to_string(FOLDER_NAME_FILE)?;
    let folder_name: FolderName = serde_json::from_str(&json)?;
    Ok(folder_name)
}

pub fn update_folder_name(folder_name: &FolderName) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(folder_name)?;
    fs::write(FOLDER_NAME_FILE, json)?;
    Ok(())
}

pub fn is_active_folder(folder_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let folder_name_data = read_folder_name()?;
    if let Some(active_folder) = folder_name_data.active_folder.iter().find(|&&active| active == Some(true)) {
        Ok(active_folder == &Some(true))
    } else {
        Ok(false)
    }
}

pub fn delete_folder_name() -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_file(FOLDER_NAME_FILE)?;
    Ok(())
}

