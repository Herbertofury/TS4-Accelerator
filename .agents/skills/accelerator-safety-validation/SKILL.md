---
name: accelerator-safety-validation
description: Recover and qualify TS4 Accelerator's conservative shadow-package path with source lineage, read-only protected roots, conflict-winner equivalence, failure fallback and matched live-game measurements.
---

# Accelerator safety and validation

Use for recovery, scanner/cache/ordering/shadow changes, runtime launch qualification or performance claims. Read `README.md`, `SOURCE_RECOVERY_STATUS.md`, `docs/CODEX_MASTER_EXECUTION.md`, actual manifests and relevant current source first.

## Recover before replacing

The status document distinguishes mirrored modules from missing archive contents. Resolve the authorized archive, verify its recorded SHA-256, inspect extraction paths and restore exact source to a separate staging directory before refactoring. Retain provenance and compare against current repository edits. A later sidecar hash without its archive does not upgrade recovered 0.3.0 source to 0.3.4. Never invent absent CLI/runtime/build commands or claim a complete build from a partial tree.

## Preserve the user source of truth

Treat real Mods, Saves, Tray, Screenshots, Resource.cfg and the game installation as protected read-only sources for accelerator-generated data. Before any authorized live test, inventory these roots and take a verifiable backup; generated/cache/shadow files belong only in the approved accelerator workspace. Check resolved-path containment and links/reparse points rather than trusting path strings. Reject staging locations inside protected roots.

Do not move/rename/merge-in-place/repair original packages or scripts. A cleanup step may remove only verified accelerator-owned generated files within its approved scope; never infer ownership from a familiar filename. Preserve external tools' original-file visibility.

## Conservative correctness gates

- Scanner and parser: distinguish missing, inaccessible, malformed and unsupported files; preserve source identity. Validate actual DBPF version/index contracts, offsets, bounds and independent reader round trips.
- Ordering and shadows: compare ordinary Resource.cfg-aware ordering to accelerated effective resource winners, including duplicate keys, nested packages and partial conflicts. Equal counts alone do not prove equal content or winners.
- Cache/reuse: test cold, warm and changed/deleted/replaced inputs; stale metadata must not preserve obsolete winners. Keep scanner/cache/planner independently testable and diagnostic failures explicit.
- Launch/runtime: on the actual supported Windows/current FPB build, verify suspended launch, narrow redirect ownership and successful runtime handshake before resume. On failure, preserve originals and use the documented safe fallback; do not broaden hooks or bypass a failed handshake to obtain a speed result.
- Interoperability: verify actual Sims 4 Studio/Tray Importer workflows against unchanged originals, plus real game save/load/restart. Preserve scripts and unrelated paths outside the narrow generated routing boundary.
- After test: compare protected-root before/after manifests, retain differences and failures, and independently verify readback. A fixture parse or compile pass is not proof of safe live redirection.

## Performance qualification

Use equivalent baseline and accelerated profiles: same game/runtime, packages, conflict winners, settings and content. Record hardware, source/package counts, cache state, cold/warm/incremental scenarios, repeated samples, median/tail latency and memory. Include generated-data creation/reuse and failure overhead rather than timing only index warmup. Never trade away content, fidelity or external-tool compatibility for speed.

Keep no-copy serving, decompression interposition and hot activation experimental until the conservative path and their own real no-loss comparisons pass. Report recovered source, offline tests, Windows build, current-game handshake, protected-data audit, compatibility and measured speed as separate gates. Do not claim production readiness from synthetic fixtures.
