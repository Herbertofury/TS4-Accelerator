# Full TODO — TS4 Accelerator

The canonical executable list is [docs/CODEX_MASTER_EXECUTION.md](../docs/CODEX_MASTER_EXECUTION.md).

## P0 — prove the recovered conservative path

- Reconcile V8/Cargo 0.3.0 with the unrecovered v0.3.4 hash lineage.
- Build Rust + C++ runtime on current Windows x64.
- Prove suspended FPB launch/injection/handshake on current TS4.
- Prove narrow redirect scope.
- Prove DBPF conflict/load-order equivalence.

## P0 — safety

- Protected path before/after hashes.
- Reparse/symlink workspace guards.
- Source-change-during-build abort.
- Crash-safe atomic generated swap.
- Save/load/restart proof.

## P1 — real speed

- Cold ordinary vs accelerated.
- Warm ordinary vs accelerated.
- One-package incremental update.
- Lot transition/hitch/frame-time comparison.
- CPU/RAM/disk I/O comparison.
- Environment metadata with every benchmark.

## P1 — ecosystem

- Sims 4 Tray Importer.
- Sims 4 Studio.
- Mod updater/manager behavior.
- `.ts4script` visibility.
- Source-to-generated resource diagnostic mapping.

## P2 — only after conservative proof

- USN Journal incremental discovery.
- PrefetchVirtualMemory/I/O rings.
- libdeflate/zlib-ng if decompression is actually hot.
- Tiny NTFS stub + in-process virtual DBPF.
- Safe-boundary package activation/hot staging.

## Release gate

No production claim until the exact release build is current-game runtime-proven and equivalent workloads show a measured gain with no material content, compatibility, safety or resource regression.
