use std::path::{Path, PathBuf};
use folder::scan;
//use dir library to get the home directory of the user 
use dirs;
use scanner::search_main_folders;


fn classify_by_type() {
    // folder name is going to be equal to the type of the file, for example, 
    // if the file is a pdf, it will be moved to the folder named "pdfs" 
    // and if the file is a mp3, it will be moved to the folder named "music" and so on.
    
    if let Some(folders) = search_main_folders() {
        for folder in folders {
            let files = scan(&folder);
        }
    }
    
}




// fn classify_folders() {
//     if let Some(folders) = search_main_folders() {
//         for folder in folders {
//             let files = scan(&folder);
//             for file in files {
//                 // classify the file based on its extension and move it to the corresponding folder
//                 let extension = file.extension().and_then(|ext| ext.to_str()).unwrap_or("");
//                 match extension {
//                     "jpg" | "jpeg" | "png" | "gif" => {
//                         // move to Pictures folder
//                         let pictures_folder = dirs::picture_dir().unwrap();
//                         let new_path = pictures_folder.join(file.file_name().unwrap());
//                         std::fs::rename(&file, &new_path).expect("Failed to move file");
//                         println!("Moved {} to {}", file.display(), new_path.display());
//                     }
//                     "mp3" | "wav" | "flac" => {
//                         // move to Music folder
//                         let music_folder = dirs::audio_dir().unwrap();
//                         let new_path = music_folder.join(file.file_name().unwrap());
//                         std::fs::rename(&file, &new_path).expect("Failed to move file");
//                         println!("Moved {} to {}", file.display(), new_path.display());
//                     }
//                     "mp4" | "mkv" | "avi" => {
//                         // move to Videos folder
//                         let videos_folder = dirs::video_dir().unwrap();
//                         let new_path = videos_folder.join(file.file_name().unwrap());
//                         std::fs::rename(&file, &new_path).expect("Failed to move file");
//                         println!("Moved {} to {}", file.display(), new_path.display());
//                     }
//                     _ => {
//                         // move to Documents folder
//                         let documents_folder = dirs::document_dir().unwrap();
//                         let new_path = documents_folder.join(file.file_name().unwrap());
//                         std::fs::rename(&file, &new_path).expect("Failed to move file");
//                         println!("Moved {} to {}", file.display(), new_path.display());
//                     }
//                 }
//             }
//         }
//     } else {
//         eprintln!("No main folders found in the home directory.");
//     }
// }
