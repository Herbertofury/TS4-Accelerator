use crate::SafetyPolicy;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub root: PathBuf,
    pub state: PathBuf,
    pub cache: PathBuf,
    pub shadow_mods: PathBuf,
    pub logs: PathBuf,
    pub runtime_config: PathBuf,
}

impl Workspace {
    pub fn new(root: impl AsRef<Path>) -> Self {
        let root = root.as_ref().to_path_buf();
        Self {
            state: root.join("state"),
            cache: root.join("cache"),
            shadow_mods: root.join("shadow").join("Mods"),
            logs: root.join("logs"),
            runtime_config: root.join("runtime.json"),
            root,
        }
    }

    pub fn create(&self, safety: &SafetyPolicy) -> Result<()> {
        for path in [&self.root, &self.state, &self.cache, &self.shadow_mods, &self.logs] {
            safety.assert_write_target(path)?;
            fs::create_dir_all(path)
                .with_context(|| format!("create accelerator workspace path: {}", path.display()))?;
        }
        Ok(())
    }
}
