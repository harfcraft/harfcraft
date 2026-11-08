use gpui_kit::base::dock::DockAreaState;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use storage::{AppStorage, load_json, to_json, write_atomic};

#[derive(Serialize, Deserialize, Default)]
pub struct WorkspaceSession {
    project_path: PathBuf,
    pub dock_state: Option<DockAreaState>,
}

impl WorkspaceSession {
    pub fn new(project_path: PathBuf, dock_state: DockAreaState) -> Self {
        Self {
            project_path,
            dock_state: Some(dock_state),
        }
    }

    pub fn load(path: &Path) -> WorkspaceSession {
        match AppStorage::workspace_session_file(path) {
            Ok(path) => load_json(&path),
            Err(err) => {
                tracing::error!(%err, "Failed to resolve session file path");
                Self {
                    project_path: path.to_path_buf(),
                    ..Self::default()
                }
            }
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = AppStorage::workspace_session_file(&self.project_path)?;
        let contents = to_json(self);
        write_atomic(&path, &contents)?;
        Ok(())
    }
}
