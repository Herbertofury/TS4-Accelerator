# Source Recovery Status

Recovered archive: `ts4-accelerator-v8-source(1).zip`  
SHA-256: `001281ee21b9d411d7f1dc40fa0272153340a100e377b0c7fd49ba1aa46de5a5`

Recovered tree: 53 files, including Rust core/CLI, C++ runtime, PowerShell scripts, Python tools, CI, specs/docs and a generated synthetic DBPF fixture.

## Directly mirrored so far

- workspace `Cargo.toml`
- `rust-toolchain.toml`
- `config.example.json`
- core crate Cargo manifest
- `src/lib.rs`, `workspace.rs`, `resource_cfg.rs`, `safety.rs`, `prewarm.rs`, `scanner.rs`
- current Codex/research/wiki documentation

## Recovered source still to mirror from the verified archive

Codex/local Git should import the remaining exact files before refactoring, including:

- `crates/ts4accel-core/src/dbpf.rs`
- `crates/ts4accel-core/src/manifest.rs`
- `crates/ts4accel-core/src/shard.rs`
- `crates/ts4accel-cli/*`
- `runtime/*`
- `scripts/*`
- `tools/*`
- `.github/workflows/windows-ci.yml`
- retained historical specs/reports

## Binary fixture

`fixtures/synthetic.package` is 800,040 bytes with SHA-256:
`57d527bda4c50701e6ea07c88ba62fe990fc81ea07a8cd545395ab3c3e10a9d5`

## Later unrecovered lineage

A separate Library sidecar records:
`ts4-accelerator-v8-v0.3.4-source.zip`
SHA-256 `6f42e69116140e68fe850429cc5665df156a5bfca84d35fc895c5077782fee87`

The matching archive was not recovered. The verified V8 source currently identifies itself as Cargo `0.3.0`. Do not relabel it as 0.3.4.
