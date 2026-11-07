use anyhow::Context;
use directories::ProjectDirs;
use harfcraft_core::constants::{APP_NAME, APP_ORGANIZATION, APP_QUALIFIER};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct AppStorage;

impl AppStorage {
    pub fn project_dirs() -> anyhow::Result<ProjectDirs> {
        ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME)
            .context("Failed to determine system configuration directories.")
    }

    pub fn config_dir() -> anyhow::Result<PathBuf> {
        let dirs = Self::project_dirs()?;
        let path = dirs.config_dir();
        fs::create_dir_all(path)?;
        Ok(path.to_owned())
    }

    pub fn config_file() -> anyhow::Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.json"))
    }

    pub fn keybindings_file() -> anyhow::Result<PathBuf> {
        Ok(Self::config_dir()?.join("keybindings.json"))
    }

    pub fn themes_dir() -> anyhow::Result<PathBuf> {
        let path = Self::config_dir()?.join("themes");
        fs::create_dir_all(&path)?;
        Ok(path.to_owned())
    }

    pub fn state_dir() -> anyhow::Result<PathBuf> {
        let dirs = Self::project_dirs()?;
        let path = dirs.data_local_dir();
        fs::create_dir_all(path)?;
        Ok(path.to_owned())
    }

    pub fn session_file() -> anyhow::Result<PathBuf> {
        Ok(Self::state_dir()?.join("session.json"))
    }
}

pub fn load_json<T: DeserializeOwned + Default>(path: &Path) -> T {
    match fs::read_to_string(path) {
        Ok(text) => serde_json::from_str(&text).unwrap_or_else(|err| {
            tracing::error!(
                ?path,
                %err,
                "Failed to deserialize file"
            );
            T::default()
        }),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => T::default(),
        Err(err) => {
            tracing::error!(
                ?path,
                %err,
                "Failed to open file"
            );
            T::default()
        }
    }
}

pub fn to_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).expect("json should always be serializable")
}

pub fn write_atomic(path: &Path, contents: &str) -> anyhow::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, contents)?;
    Ok(std::fs::rename(&tmp, path)?)
}
