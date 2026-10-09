use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

const FOLDER_NAME_FILE: &str = "src/data/folder_name.json";


#[derive(Serialize, Deserialize, Debug)]
pub struct FolderInfo {
    pub paths: Vec<PathBuf>,
    pub active: bool,
}


pub struct MainFolders {
   pub folders: HashMap<String, Vec<PathBuf>>,
   pub active_folder: Vec<bool>
}

impl MainFolders {
    pub fn is_empty(&self) -> bool {
        self.folders.is_empty()
    }
    pub fn new() -> Self {
        Self {
            folders: HashMap::new(),
            active_folder: vec![],
        }
    }

    pub fn show(&self) -> String {
        let mut result = String::new();
        for (folder_name, paths) in &self.folders {
            result.push_str(&format!("{}: {:?}\n", folder_name, paths));
        }
        result
    }
}
