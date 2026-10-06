fn serach_main_folders(){
    // scan the current user's home directory for the main folders (Documents, Downloads, Pictures)

    let home_dir = dirs::home_dir().unwrap();

    // we are going to find documents, downloads, pictures, music, videos, desktop and public folders in the home directory
    let main_folders = vec!["Documents", "Downloads", "Pictures", "Music", "Videos", "Desktop", "Public"];
    
    for folder in main_folders {
        let folder_path = home_dir.join(folder);
        if folder_path.exists() {
            println!("Found folder: {}", folder_path.display());
        } else {
            
        }
    }
}

