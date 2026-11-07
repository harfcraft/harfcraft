use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use storage::{AppStorage, load_json, to_json, write_atomic};

pub const SESSION_MAX_RECENT_PROJECTS: usize = 10;

#[derive(Deserialize, Serialize, Default)]
pub struct AppSession {
    pub last_project_path: Option<PathBuf>,
    pub recent_projects: Vec<PathBuf>,
}

impl AppSession {
    pub fn load() -> AppSession {
        match AppStorage::session_file() {
            Ok(path) => load_json(&path),
            Err(err) => {
                tracing::error!(%err, "Failed to resolve session file path");
                Self::default()
            }
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = AppStorage::session_file()?;
        let contents = to_json(self);
        write_atomic(&path, &contents)?;
        Ok(())
    }

    pub fn record_project_open(&mut self, path: PathBuf) {
        self.last_project_path = Some(path.clone());
        self.recent_projects.retain(|p| p != &path);
        self.recent_projects.insert(0, path);
        self.recent_projects.truncate(SESSION_MAX_RECENT_PROJECTS);
        if let Err(err) = self.save() {
            tracing::error!(%err, "Failed to persist updated session history");
        }
    }
}
