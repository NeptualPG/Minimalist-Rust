use std::path::{Path, PathBuf};
use folder::scan;
//use dir library to get the home directory of the user 
use dirs;

pub struct MainFolders {
    documents: Option<PathBuf>,
    downloads: Option<PathBuf>,
    pictures: Option<PathBuf>,
    music: Option<PathBuf>,
    videos: Option<PathBuf>,
    desktop: Option<PathBuf>,
    public: Option<PathBuf>,
}

impl MainFolders {
    pub fn is_empty(&self) -> bool {
        self.documents.is_none() &&
        self.downloads.is_none() &&
        self.pictures.is_none() &&
        self.music.is_none() &&
        self.videos.is_none() &&
        self.desktop.is_none() &&
        self.public.is_none()
    }
}


pub fn search_main_folders() -> Option<MainFolders> {
    // scan the current user's home directory for the main folders (Documents, Downloads, Pictures)

    let home_dir = dirs::home_dir().unwrap();
    let mut found_folders: MainFolders = MainFolders {
        documents: None,
        downloads: None,
        pictures: None,
        music: None,
        videos: None,
        desktop: None,
        public: None,
    };
    // we are going to find documents, downloads, pictures, music, videos, desktop and public folders in the home directory
    let main_folders = vec!["Documents", "Downloads", "Pictures", "Music", "Videos", "Desktop", "Public"];

    for folder in main_folders {
        let folder_path = home_dir.join(folder);
        if folder_path.exists() {
            match folder {
                "Documents" => found_folders.documents = Some(folder_path),
                "Downloads" => found_folders.downloads = Some(folder_path),
                "Pictures" => found_folders.pictures = Some(folder_path),
                "Music" => found_folders.music = Some(folder_path),
                "Videos" => found_folders.videos = Some(folder_path),
                "Desktop" => found_folders.desktop = Some(folder_path),
                "Public" => found_folders.public = Some(folder_path),
                _ => (),
            }
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

