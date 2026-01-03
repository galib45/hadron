use std::path::PathBuf;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub proton_path: PathBuf,
    pub umu_path: PathBuf,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub name: String,
    pub cover_path: PathBuf,
    pub exe_path: PathBuf,
    pub wine_prefix: PathBuf,
}


#[allow(dead_code)]
impl Game {
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_cover_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.cover_path = path.into();
        self
    }

    pub fn with_exe_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.exe_path = path.into();
        self
    }

    pub fn with_wine_prefix(mut self, path: impl Into<PathBuf>) -> Self {
        self.wine_prefix = path.into();
        self
    }
}
