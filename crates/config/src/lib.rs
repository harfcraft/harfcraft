use serde::{Deserialize, Serialize};
use storage::{AppStorage, load_json, to_json, write_atomic};

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "Default Dark".to_string(),
        }
    }
}

impl AppConfig {
    pub fn load() -> AppConfig {
        match AppStorage::config_file() {
            Ok(path) => load_json(&path),
            Err(err) => {
                tracing::error!(%err, "Failed to resolve config file path");
                Self::default()
            }
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = AppStorage::config_file()?;
        let contents = to_json(self);
        write_atomic(&path, &contents)?;
        Ok(())
    }

    pub fn theme(&self) -> &str {
        &self.theme
    }
}
