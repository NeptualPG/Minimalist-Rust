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

impl FolderName {
    // Create a new instance of FolderName
    pub fn create_folder(
        &mut self,
        folder_name: String,
        folder_path: PathBuf,
        active: Option<bool>
    ) {
        self.folders.insert(folder_name, vec![folder_path]);
        self.active_folder.push(active);
    }

    // DELETE
    pub fn delete_folder(&mut self, folder_name: &str) {
        self.folders.remove(folder_name);
    }

    // LOAD
    pub fn load_from_file() -> Self {
        let file_content = fs::read_to_string(FOLDER_NAME_FILE).unwrap_or_else(|_| {
            // If the file doesn't exist, create a new instance and save it to the file
            let new_instance = FolderName {
                folders: HashMap::new(),
                active_folder: vec![],
            };
            new_instance.save_to_file();
            String::new()
        });

        if file_content.is_empty() {
            FolderName {
                folders: HashMap::new(),
                active_folder: vec![],
            }
        } else {
            serde_json::from_str(&file_content).expect("Failed to parse JSON")
        }
    }
}