# TS4 Accelerator

> Windows-only, Rust-first package-loading accelerator for **The Sims 4**. It attacks file-count, package-index and launch-path overhead while keeping the real Mods/Saves/Tray/game installation as the source of truth and read-only from the accelerator's generated-data path.

**Recovered implementation:** V8 source, internal Cargo version `0.3.0`  
**Recovered source archive SHA-256:** `001281ee21b9d411d7f1dc40fa0272153340a100e377b0c7fd49ba1aa46de5a5`  
**Additional lineage evidence:** a later `ts4-accelerator-v8-v0.3.4-source.zip.sha256` record exists with SHA-256 `6f42e69116140e68fe850429cc5665df156a5bfca84d35fc895c5077782fee87`, but the matching source archive was not recovered in this workspace. Do not call the current tree 0.3.4 without reconstructing/verifying that missing lineage.

## What is already implemented in the recovered source

```text
real Mods folder (read-only source)
  -> parallel .package discovery
  -> strict DBPF 2.1 / index 0.3 parsing
  -> persistent SQLite metadata cache
  -> Resource.cfg-aware effective ordering
  -> conflict-aware physical shadow planning
  -> front-indexed generated DBPF units
  -> unchanged-unit reuse
  -> parallel index prewarm
  -> suspended TS4 FPB launch
  -> injected narrow path-redirect runtime DLL
  -> handshake before TS4 primary thread resumes
```

The game is redirected only for accelerator-owned `Resource.cfg` / `__TS4Accel` package paths. Original `.ts4script`, package/config/data paths remain visible to external tools and normal gameplay.

## Absolute safety contract

TS4 Accelerator must not move, delete, rename, repair, merge-in-place or write generated data into:

- the real `Documents\Electronic Arts\The Sims 4` tree;
- Mods, Saves, Tray, Screenshots or the real `Resource.cfg`;
- original `.package` / `.ts4script` files;
- the EA/Steam/Epic game installation;
- supplied EA executables/DLLs/assets.

Generated/cache data belongs under `%LOCALAPPDATA%\TS4Accelerator` or explicit safe build/release staging paths.

## Current status

This is a **source-complete pre-release alpha**, not a production-speed claim. Offline structural work is substantial, including a 10,000-resource independent DBPF fixture, but the major Windows/current-game gates remain open:

- Windows Rust/MSVC build and CI proof;
- runtime DLL handshake against the actual current FPB game build;
- exact ordinary-vs-accelerated conflict-winner comparison;
- Sims 4 Tray Importer and Sims 4 Studio compatibility;
- save/load/restart regression;
- protected-data before/after audit;
- real ~10,000-package cold/warm/incremental benchmarks.

See [`PROGRESS.md`](PROGRESS.md) and the refreshed [`docs/CODEX_MASTER_EXECUTION.md`](docs/CODEX_MASTER_EXECUTION.md).

## Why this architecture

The goal is not “merge your Mods folder.” The goal is to hide the **cost of file count** while preserving the user's normal files and debug/update workflow.

The conservative release path is physical shadow DBPF packages + narrow process-local redirection. More invasive no-copy virtual DBPF serving, decompression interposition and hot activation remain experimental until the conservative path is proven on real TS4 builds.

## Repository map

- `crates/ts4accel-core` — scanner, DBPF parse/write, cache, ordering, sharding, safety, prewarm.
- `crates/ts4accel-cli` — CLI/build flow/fingerprinting/suspended launch/injection/fallback.
- `runtime` — C++23 MinHook runtime DLL.
- `tools` — independent validators/probes/source-map checks.
- `scripts` — build/start/calibration/safety/release helpers.
- `docs/specs` — retained research/design inputs.
- `docs/` — current architecture, compatibility, current Codex execution contract and toolbox.
- `wiki/` — canonical Markdown mirror for the GitHub Wiki.
- `RECOVERED_BINARY_ARTIFACTS.md` — exact unreproduced binary fixture identity.

## Codex start order

1. Read [`docs/CODEX_MASTER_EXECUTION.md`](docs/CODEX_MASTER_EXECUTION.md).
2. Reconcile the recovered `0.3.0` source with the missing `0.3.4` hash lineage before claiming a later version.
3. Build on Windows x64 and close the conservative live-game gates first.
4. Measure equivalent baseline vs accelerated workloads.
5. Only then promote advanced no-copy/USN/decompression/hot-activation work that demonstrates a real no-loss win.

## External research/tooling

See [`docs/DEVELOPER_TOOLBOX.md`](docs/DEVELOPER_TOOLBOX.md) for current DBPF readers, Sims tooling, PresentMon, ETW/profiling, MinHook/Detours, compression libraries and filesystem candidates that Codex can evaluate instead of reinventing them.
