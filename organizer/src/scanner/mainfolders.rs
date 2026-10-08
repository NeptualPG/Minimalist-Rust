use std::{collections::HashMap, path::PathBuf};

pub struct MainFolders {
   pub folders: HashMap<String, Vec<PathBuf>>,
   pub active_folder: Vec<Option<bool>>
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
