use std::{
    fs,
    path::{Path, PathBuf},
};

pub enum ProjectSource {
    UnifiedFontObject,
    DesignSpace,
}

pub struct Project {
    source: ProjectSource,
    path: PathBuf,
    name: String,
}

impl Project {
    pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {
        let path = path.canonicalize()?;
        let metadata = fs::metadata(&path)?;
        Ok(Self {
            source: if metadata.is_dir() {
                ProjectSource::UnifiedFontObject
            } else {
                ProjectSource::DesignSpace
            },
            name: path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or("Untitled".to_string()),
            path,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn source(&self) -> &ProjectSource {
        &self.source
    }
}
