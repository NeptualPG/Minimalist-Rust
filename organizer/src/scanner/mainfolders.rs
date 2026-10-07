use std::{collections::HashMap, path::PathBuf};

pub struct MainFolders {
   pub folders: HashMap<String, Vec<PathBuf>>,
}

impl MainFolders {
    pub fn is_empty(&self) -> bool {
        self.folders.is_empty()
    }
    pub fn new() -> Self {
        Self {
            folders: HashMap::new(),
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
