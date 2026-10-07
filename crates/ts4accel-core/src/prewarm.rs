use anyhow::{bail, Context, Result};
use byteorder::{ByteOrder, LittleEndian};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::dbpf::DBPF_HEADER_SIZE;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrewarmReport {
    pub package_count: usize,
    pub index_bytes_read: u64,
    pub failed_packages: Vec<PathBuf>,
}

/// Reads only each package's DBPF header and index. This intentionally warms
/// metadata needed by TS4 without streaming all resource payloads into memory.
pub fn prewarm_package_indexes(mods_root: &Path) -> Result<PrewarmReport> {
    let package_paths = WalkDir::new(mods_root)
        .follow_links(false)
        .into_iter()
        .filter_map(|item| item.ok())
        .filter(|item| item.file_type().is_file())
        .map(|item| item.into_path())
        .filter(|path| {
            path.extension()
                .map(|extension| extension.eq_ignore_ascii_case("package"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    let results = package_paths
        .par_iter()
        .map(|path| warm_one(path).map_err(|error| (path.clone(), error)))
        .collect::<Vec<_>>();

    let mut report = PrewarmReport {
        package_count: package_paths.len(),
        ..PrewarmReport::default()
    };
    for result in results {
        match result {
            Ok(bytes) => report.index_bytes_read = report.index_bytes_read.saturating_add(bytes),
            Err((path, _error)) => report.failed_packages.push(path),
        }
    }
    Ok(report)
}

fn warm_one(path: &Path) -> Result<u64> {
    let mut file = File::open(path)
        .with_context(|| format!("open package for read-only prewarm: {}", path.display()))?;
    let mut header = [0u8; DBPF_HEADER_SIZE];
    file.read_exact(&mut header)
        .with_context(|| format!("read DBPF header for prewarm: {}", path.display()))?;
    if &header[0..4] != b"DBPF" {
        bail!("not a DBPF package: {}", path.display());
    }

    let short_offset = LittleEndian::read_u32(&header[40..44]);
    let index_size = LittleEndian::read_u32(&header[44..48]) as u64;
    let long_offset = LittleEndian::read_u64(&header[64..72]);
    let index_offset = if short_offset == 0 {
        long_offset
    } else {
        short_offset as u64
    };

    let file_size = file.metadata()?.len();
    let index_end = index_offset
        .checked_add(index_size)
        .context("DBPF index range overflow during prewarm")?;
    if index_offset < DBPF_HEADER_SIZE as u64 || index_end > file_size {
        bail!("invalid DBPF index range during prewarm: {}", path.display());
    }

    file.seek(SeekFrom::Start(index_offset))?;
    let mut remaining = index_size;
    let mut buffer = vec![0u8; 1024 * 1024];
    while remaining > 0 {
        let amount = remaining.min(buffer.len() as u64) as usize;
        file.read_exact(&mut buffer[..amount])?;
        remaining -= amount as u64;
    }
    Ok(DBPF_HEADER_SIZE as u64 + index_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dbpf::{write_front_indexed_shard, ResourceEntry, ResourceKey, ShardInput};
    use tempfile::tempdir;

    #[test]
    fn warms_only_valid_dbpf_index_ranges() -> Result<()> {
        let temp = tempdir()?;
        let payload = temp.path().join("payload.bin");
        std::fs::write(&payload, b"payload")?;
        let entry = ResourceEntry {
            key: ResourceKey {
                type_id: 1,
                group_id: 2,
                instance_id: 3,
            },
            source_offset: 0,
            stored_size: 7,
            uncompressed_size: 7,
            compression_type: 0,
            committed: 1,
            source_entry_index: 0,
        };
        let package = temp.path().join("test.package");
        write_front_indexed_shard(
            &package,
            &[ShardInput {
                package_path: &payload,
                entry: &entry,
            }],
        )?;

        let report = prewarm_package_indexes(temp.path())?;
        assert_eq!(report.package_count, 1);
        assert!(report.index_bytes_read > DBPF_HEADER_SIZE as u64);
        assert!(report.failed_packages.is_empty());
        Ok(())
    }
}
