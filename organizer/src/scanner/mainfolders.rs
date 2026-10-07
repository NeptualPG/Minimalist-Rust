use std::path::{Path, PathBuf};
pub struct MainFolders {
    pub documents: Option<PathBuf>,
    pub downloads: Option<PathBuf>,
    pub pictures: Option<PathBuf>,
    pub music: Option<PathBuf>,
    pub videos: Option<PathBuf>,
    pub desktop: Option<PathBuf>,
    pub public: Option<PathBuf>,
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
    pub fn new() -> Self {
        Self {
            documents: None,
            downloads: None,
            pictures: None,
            music: None,
            videos: None,
            desktop: None,
            public: None,
        }
    }

    pub fn show_optional(&self) -> String {
        let mut result = String::new();
        if let Some(documents) = &self.documents {
            result.push_str(&format!("Documents: {:?}\n", documents));
        }
        if let Some(downloads) = &self.downloads {
            result.push_str(&format!("Downloads: {:?}\n", downloads));
        }
        if let Some(pictures) = &self.pictures {
            result.push_str(&format!("Pictures: {:?}\n", pictures));
        }
        if let Some(music) = &self.music {
            result.push_str(&format!("Music: {:?}\n", music));
        }
        if let Some(videos) = &self.videos {
            result.push_str(&format!("Videos: {:?}\n", videos));
        }
        if let Some(desktop) = &self.desktop {
            result.push_str(&format!("Desktop: {:?}\n", desktop));
        }
        if let Some(public) = &self.public {
            result.push_str(&format!("Public: {:?}\n", public));
        }
        result
    }
}
