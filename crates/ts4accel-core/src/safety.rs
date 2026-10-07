use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyPolicy {
    workspace: PathBuf,
    protected_roots: Vec<PathBuf>,
}

impl SafetyPolicy {
    pub fn new(workspace: impl AsRef<Path>, protected_roots: impl IntoIterator<Item = PathBuf>) -> Result<Self> {
        let workspace = absolute_lexical(workspace.as_ref())?;
        let protected_roots = protected_roots
            .into_iter()
            .map(|path| absolute_lexical(&path))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { workspace, protected_roots })
    }

    pub fn workspace(&self) -> &Path { &self.workspace }

    pub fn assert_write_target(&self, target: impl AsRef<Path>) -> Result<PathBuf> {
        let target = absolute_lexical(target.as_ref())?;
        if !is_within(&target, &self.workspace) {
            bail!("SAFETY BLOCK: write target is outside accelerator workspace: {}", target.display());
        }
        for protected in &self.protected_roots {
            if is_within(&target, protected) || is_within(protected, &target) {
                bail!("SAFETY BLOCK: write target intersects protected Sims 4 data: {}", target.display());
            }
        }
        self.reject_symlink_escape(&target)?;
        Ok(target)
    }

    pub fn assert_read_source(&self, source: impl AsRef<Path>) -> Result<PathBuf> {
        let source = absolute_lexical(source.as_ref())?;
        if !source.exists() {
            bail!("read source does not exist: {}", source.display());
        }
        Ok(source)
    }

    fn reject_symlink_escape(&self, target: &Path) -> Result<()> {
        let mut current = PathBuf::new();
        for component in target.components() {
            current.push(component.as_os_str());
            if !current.exists() { continue; }
            let meta = fs::symlink_metadata(&current)
                .with_context(|| format!("inspect path safety: {}", current.display()))?;
            if is_link_or_reparse_point(&meta) {
                let resolved = fs::canonicalize(&current)?;
                if !is_within(&resolved, &self.workspace) {
                    bail!("SAFETY BLOCK: workspace symlink escapes workspace: {} -> {}", current.display(), resolved.display());
                }
            }
        }
        Ok(())
    }
}

#[cfg(windows)]
fn is_link_or_reparse_point(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    metadata.file_type().is_symlink()
        || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(windows))]
fn is_link_or_reparse_point(metadata: &fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

fn absolute_lexical(path: &Path) -> Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    let mut clean = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => { clean.pop(); }
            other => clean.push(other.as_os_str()),
        }
    }
    Ok(clean)
}

fn is_within(path: &Path, root: &Path) -> bool {
    if cfg!(windows) {
        let path = path.to_string_lossy().replace('/', "\").to_lowercase();
        let root = root.to_string_lossy().replace('/', "\").trim_end_matches('\').to_lowercase();
        path == root || path.starts_with(&(root + "\"))
    } else {
        path == root || path.starts_with(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn blocks_writes_outside_workspace() -> Result<()> {
        let temp = tempdir()?;
        let workspace = temp.path().join("workspace");
        let protected = temp.path().join("Mods");
        fs::create_dir_all(&workspace)?;
        fs::create_dir_all(&protected)?;
        let policy = SafetyPolicy::new(&workspace, vec![protected.clone()])?;
        assert!(policy.assert_write_target(workspace.join("cache/a.bin")).is_ok());
        assert!(policy.assert_write_target(protected.join("bad.package")).is_err());
        assert!(policy.assert_write_target(temp.path().join("other/bad.bin")).is_err());
        Ok(())
    }

    #[test]
    fn blocks_workspace_that_contains_protected_data() -> Result<()> {
        let temp = tempdir()?;
        let protected = temp.path().join("The Sims 4");
        fs::create_dir_all(&protected)?;
        let policy = SafetyPolicy::new(temp.path(), vec![protected])?;
        assert!(policy.assert_write_target(temp.path().join("cache.bin")).is_err());
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn blocks_workspace_symlink_escape() -> Result<()> {
        use std::os::unix::fs::symlink;

        let temp = tempdir()?;
        let workspace = temp.path().join("workspace");
        let outside = temp.path().join("outside");
        fs::create_dir_all(&workspace)?;
        fs::create_dir_all(&outside)?;
        symlink(&outside, workspace.join("escape"))?;
        let policy = SafetyPolicy::new(&workspace, Vec::<PathBuf>::new())?;
        assert!(policy
            .assert_write_target(workspace.join("escape").join("bad.bin"))
            .is_err());
        Ok(())
    }
}
