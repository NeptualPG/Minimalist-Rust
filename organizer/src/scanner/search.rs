//use dir library to get the home directory of the user 
use dirs;
use crate::scanner::mainfolders::MainFolders;


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

