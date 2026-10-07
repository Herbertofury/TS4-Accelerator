use crate::dbpf::DbpfPackage;
use crate::resource_cfg::ResourceConfig;
use anyhow::{Context, Result};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::UNIX_EPOCH;
use walkdir::WalkDir;

const PARSER_CACHE_PREFIX: &str = "v3:";

#[derive(Debug, Clone)]
pub struct ScanOptions {
    pub mods_dir: PathBuf,
    pub resource_cfg: ResourceConfig,
    pub full_hash: bool,
    pub cache: Option<Arc<HashMap<PathBuf, CachedPackageParse>>>,
    pub calibration_order: Option<Arc<HashMap<String, u64>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedPackageParse {
    pub size_bytes: u64,
    pub modified_ns: u128,
    pub fingerprint: String,
    pub package: Option<DbpfPackage>,
    pub parse_error: Option<String>,
    pub safety_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageRecord {
    pub canonical_path: PathBuf,
    pub relative_path: PathBuf,
    pub priority: i32,
    pub rule_ordinal: usize,
    pub load_sequence: u64,
    pub size_bytes: u64,
    pub modified_ns: u128,
    pub fingerprint: String,
    pub package: Option<DbpfPackage>,
    pub parse_error: Option<String>,
    pub safety_class: String,
    #[serde(default)]
    pub cache_hit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    pub mods_dir: PathBuf,
    pub packages: Vec<PackageRecord>,
    pub parsed_count: usize,
    pub passthrough_count: usize,
    pub total_resources: usize,
    pub cache_hit_count: usize,
    #[serde(default)]
    pub preserved_resource_cfg_lines: Vec<String>,
    #[serde(default)]
    pub calibrated_package_count: usize,
    #[serde(default)]
    pub uncalibrated_package_count: usize,
}

pub fn scan_packages(options: &ScanOptions) -> Result<ScanReport> {
    let mods_dir = fs::canonicalize(&options.mods_dir)
        .with_context(|| format!("canonicalize Mods directory: {}", options.mods_dir.display()))?;

    let compiled_config = options.resource_cfg.compile()?;
    let mut candidates = Vec::new();
    for item in WalkDir::new(&mods_dir).follow_links(false).into_iter() {
        let item = item.with_context(|| {
            format!(
                "walk Mods tree without skipping entries under {}",
                mods_dir.display()
            )
        })?;
        if item.file_type().is_symlink() {
            anyhow::bail!(
                "unsupported symlink or reparse-point entry in Mods tree: {}; refusing to risk omitting redirected content",
                item.path().display()
            );
        }
        if !item.file_type().is_file() {
            continue;
        }
        let path = item.path();
        if !path.extension().map(|v| v.eq_ignore_ascii_case("package")).unwrap_or(false) { continue; }
        let relative = path.strip_prefix(&mods_dir)?.to_path_buf();
        let placements = compiled_config.matching_placements(&relative);
        if placements.len() > 1 {
            anyhow::bail!(
                "Resource.cfg ambiguity: {} matches {} PackedFile rules; refusing to guess load semantics",
                relative.display(),
                placements.len()
            );
        }
        if let Some(placement) = placements.into_iter().next() {
            candidates.push((path.to_path_buf(), relative, placement));
        }
    }

    let calibrated_package_count = candidates
        .iter()
        .filter(|(_, relative, _)| {
            let key = normalize_sort(relative);
            options
                .calibration_order
                .as_deref()
                .map(|order| order.contains_key(&key))
                .unwrap_or(false)
        })
        .count();
    let uncalibrated_package_count = candidates.len().saturating_sub(calibrated_package_count);

    candidates.sort_by(|a, b| {
        let a_key = normalize_sort(&a.1);
        let b_key = normalize_sort(&b.1);
        let a_calibrated = options.calibration_order.as_deref().and_then(|map| map.get(&a_key)).copied();
        let b_calibrated = options.calibration_order.as_deref().and_then(|map| map.get(&b_key)).copied();
        b.2.priority.cmp(&a.2.priority)
            .then_with(|| a.2.rule_ordinal.cmp(&b.2.rule_ordinal))
            .then_with(|| match (a_calibrated, b_calibrated) {
                (Some(left), Some(right)) => left.cmp(&right),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a_key.cmp(&b_key),
            })
    });

    let records: Vec<PackageRecord> = candidates
        .into_par_iter()
        .enumerate()
        .map(|(load_sequence, (path, relative, placement))| {
            scan_one(
                path,
                relative,
                placement.priority,
                placement.rule_ordinal,
                load_sequence as u64,
                options.full_hash,
                options.cache.as_deref(),
            )
        })
        .collect::<Result<Vec<_>>>()?;

    let parsed_count = records.iter().filter(|r| r.package.is_some()).count();
    let passthrough_count = records.len() - parsed_count;
    let total_resources = records.iter().filter_map(|r| r.package.as_ref()).map(|p| p.entries.len()).sum();
    let cache_hit_count = records.iter().filter(|record| record.cache_hit).count();
    Ok(ScanReport {
        mods_dir,
        packages: records,
        parsed_count,
        passthrough_count,
        total_resources,
        cache_hit_count,
        preserved_resource_cfg_lines: options.resource_cfg.preserved_lines.clone(),
        calibrated_package_count,
        uncalibrated_package_count,
    })
}


pub fn verify_source_metadata_unchanged(report: &ScanReport) -> Result<usize> {
    let checked = report
        .packages
        .par_iter()
        .map(|record| {
            let metadata = fs::metadata(&record.canonical_path).with_context(|| {
                format!(
                    "recheck source package metadata: {}",
                    record.canonical_path.display()
                )
            })?;
            let modified_ns = metadata
                .modified()
                .ok()
                .and_then(|value| value.duration_since(UNIX_EPOCH).ok())
                .map(|value| value.as_nanos())
                .unwrap_or_default();
            if metadata.len() != record.size_bytes || modified_ns != record.modified_ns {
                anyhow::bail!(
                    "source package changed during accelerator build: {}",
                    record.canonical_path.display()
                );
            }
            Ok(())
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(checked.len())
}

fn scan_one(
    path: PathBuf,
    relative_path: PathBuf,
    priority: i32,
    rule_ordinal: usize,
    load_sequence: u64,
    full_hash: bool,
    cache: Option<&HashMap<PathBuf, CachedPackageParse>>,
) -> Result<PackageRecord> {
    let canonical_path = fs::canonicalize(&path)?;
    let metadata = fs::metadata(&canonical_path)?;
    let modified_ns = metadata.modified().ok()
        .and_then(|v| v.duration_since(UNIX_EPOCH).ok())
        .map(|v| v.as_nanos())
        .unwrap_or_default();

    if !full_hash {
        if let Some(cached) = cache.and_then(|cache| cache.get(&canonical_path)) {
            if cached.size_bytes == metadata.len()
                && cached.modified_ns == modified_ns
                && cached.fingerprint.starts_with(PARSER_CACHE_PREFIX)
            {
                return Ok(PackageRecord {
                    canonical_path,
                    relative_path,
                    priority,
                    rule_ordinal,
                    load_sequence,
                    size_bytes: cached.size_bytes,
                    modified_ns: cached.modified_ns,
                    fingerprint: cached.fingerprint.clone(),
                    package: cached.package.clone(),
                    parse_error: cached.parse_error.clone(),
                    safety_class: cached.safety_class.clone(),
                    cache_hit: true,
                });
            }
        }
    }

    let parsed = DbpfPackage::parse(&canonical_path);
    let (package, parse_error, safety_class, index_hash) = match parsed {
        Ok(package) if package.duplicate_keys_inside_package => {
            let hash = package.index_hash.clone();
            (Some(package), None, "passthrough-duplicate-tgi".to_string(), hash)
        }
        Ok(package) => {
            let hash = package.index_hash.clone();
            (Some(package), None, "merge-safe".to_string(), hash)
        }
        Err(error) => (None, Some(format!("{error:#}")), "passthrough-unparsed".to_string(), "unparsed".to_string()),
    };

    let fingerprint = if full_hash {
        let file = fs::File::open(&canonical_path)?;
        let mut reader = BufReader::with_capacity(1024 * 1024, file);
        let mut hasher = blake3::Hasher::new();
        hasher.update(PARSER_CACHE_PREFIX.as_bytes());
        let mut buffer = vec![0u8; 1024 * 1024];
        loop {
            let read = reader.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
        }
        format!("{PARSER_CACHE_PREFIX}{}", hasher.finalize().to_hex())
    } else {
        let mut hasher = blake3::Hasher::new();
        hasher.update(PARSER_CACHE_PREFIX.as_bytes());
        hasher.update(canonical_path.to_string_lossy().as_bytes());
        hasher.update(&metadata.len().to_le_bytes());
        hasher.update(&modified_ns.to_le_bytes());
        hasher.update(index_hash.as_bytes());
        format!("{PARSER_CACHE_PREFIX}{}", hasher.finalize().to_hex())
    };

    Ok(PackageRecord {
        canonical_path,
        relative_path,
        priority,
        rule_ordinal,
        load_sequence,
        size_bytes: metadata.len(),
        modified_ns,
        fingerprint,
        package,
        parse_error,
        safety_class,
        cache_hit: false,
    })
}

pub fn load_calibration_order(path: &Path) -> Result<HashMap<String, u64>> {
    let text = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(HashMap::new()),
        Err(error) => return Err(error).with_context(|| format!("read calibration order: {}", path.display())),
    };
    let mut order = HashMap::new();
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        order.entry(normalize_sort(Path::new(trimmed))).or_insert(index as u64);
    }
    Ok(order)
}

fn normalize_sort(path: &Path) -> String {
    path.to_string_lossy().replace('\', "/").to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn calibration_order_is_case_and_separator_insensitive() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "Creator\Hair.package")?;
        writeln!(file, "creator/Clothes.package")?;
        let order = load_calibration_order(file.path())?;
        assert_eq!(order.get("creator/hair.package"), Some(&0));
        assert_eq!(order.get("creator/clothes.package"), Some(&1));
        Ok(())
    }
}
