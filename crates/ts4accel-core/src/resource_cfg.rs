use anyhow::{Context, Result};
use globset::{GlobBuilder, GlobMatcher};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadRule {
    pub priority: i32,
    pub pattern: String,
    pub ordinal: usize,
}

#[derive(Debug, Clone)]
pub struct CompiledRule {
    pub rule: LoadRule,
    matcher: GlobMatcher,
}

#[derive(Debug, Clone)]
pub struct CompiledResourceConfig {
    rules: Vec<CompiledRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    pub source: Option<PathBuf>,
    pub rules: Vec<LoadRule>,
    /// Original lines that are not Priority/PackedFile directives. These are
    /// copied into the accelerator-owned Resource.cfg so unusual user setups
    /// keep their non-package behavior without ever editing the real file.
    #[serde(default)]
    pub preserved_lines: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LoadPlacement {
    pub priority: i32,
    pub rule_ordinal: usize,
}

impl ResourceConfig {
    pub fn load(path: Option<&Path>) -> Result<Self> {
        match path {
            Some(path) if path.exists() => {
                let text = fs::read_to_string(path)
                    .with_context(|| format!("read Resource.cfg: {}", path.display()))?;
                let (rules, preserved_lines) = parse_config(&text);
                Ok(Self { source: Some(path.to_path_buf()), rules, preserved_lines })
            }
            _ => Ok(Self { source: None, rules: default_rules(), preserved_lines: Vec::new() }),
        }
    }

    pub fn compile(&self) -> Result<CompiledResourceConfig> {
        let rules = self.rules
            .iter()
            .cloned()
            .map(|rule| {
                let normalized = normalize_pattern(&rule.pattern);
                let matcher = GlobBuilder::new(&normalized)
                    .case_insensitive(true)
                    .literal_separator(true)
                    .build()
                    .with_context(|| format!("compile Resource.cfg pattern {}", rule.pattern))?
                    .compile_matcher();
                Ok(CompiledRule { rule, matcher })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(CompiledResourceConfig { rules })
    }

    pub fn placement_for(&self, relative: &Path) -> Result<Option<LoadPlacement>> {
        self.compile()?.placement_for(relative)
    }
}

impl CompiledResourceConfig {
    pub fn matching_placements(&self, relative: &Path) -> Vec<LoadPlacement> {
        let slash = relative.to_string_lossy().replace('\', "/");
        self.rules
            .iter()
            .filter(|compiled| compiled.matcher.is_match(&slash))
            .map(|compiled| LoadPlacement {
                priority: compiled.rule.priority,
                rule_ordinal: compiled.rule.ordinal,
            })
            .collect()
    }

    pub fn placement_for(&self, relative: &Path) -> Result<Option<LoadPlacement>> {
        let mut placements = self.matching_placements(relative);
        placements.sort_by(|left, right| {
            right
                .priority
                .cmp(&left.priority)
                .then_with(|| left.rule_ordinal.cmp(&right.rule_ordinal))
        });
        Ok(placements.into_iter().next())
    }
}

fn parse_config(text: &str) -> (Vec<LoadRule>, Vec<String>) {
    let mut priority = 500i32;
    let mut rules = Vec::new();
    let mut preserved_lines = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            preserved_lines.push(raw.to_string());
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(command) = parts.next() else { continue };
        if command.eq_ignore_ascii_case("Priority") {
            if let Some(value) = parts.next().and_then(|v| v.parse::<i32>().ok()) {
                priority = value;
            }
        } else if command.eq_ignore_ascii_case("PackedFile") {
            let pattern = parts.collect::<Vec<_>>().join(" ");
            if !pattern.is_empty() {
                rules.push(LoadRule { priority, pattern, ordinal: rules.len() });
            }
        } else {
            preserved_lines.push(raw.to_string());
        }
    }
    let rules = if rules.is_empty() { default_rules() } else { rules };
    (rules, preserved_lines)
}

fn default_rules() -> Vec<LoadRule> {
    (0..=5)
        .map(|depth| {
            let pattern = if depth == 0 {
                "*.package".to_string()
            } else {
                format!("{}*.package", "*/".repeat(depth))
            };
            LoadRule { priority: 500, pattern, ordinal: depth }
        })
        .collect()
}

fn normalize_pattern(pattern: &str) -> String {
    pattern.replace('\', "/").trim_start_matches("./").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_non_package_directives_and_comments() {
        let (_, preserved) = parse_config(
            "# keep me\nPriority 500\nPackedFile *.package\nDirectoryFiles unpackedmod autoupdate\n",
        );
        assert!(preserved.iter().any(|line| line == "# keep me"));
        assert!(preserved
            .iter()
            .any(|line| line == "DirectoryFiles unpackedmod autoupdate"));
    }

    #[test]
    fn parses_default_style_config() -> Result<()> {
        let cfg = ResourceConfig {
            source: None,
            rules: parse_config("Priority 500\nPackedFile *.package\nPackedFile */*.package\n").0,
            preserved_lines: Vec::new(),
        };
        assert!(cfg.placement_for(Path::new("a.package"))?.is_some());
        assert!(cfg.placement_for(Path::new("Folder/a.package"))?.is_some());
        assert!(cfg.placement_for(Path::new("A/B/a.package"))?.is_none());
        Ok(())
    }
}
