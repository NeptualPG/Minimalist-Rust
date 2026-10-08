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
}