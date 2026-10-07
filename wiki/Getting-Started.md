# Getting Started

TS4 Accelerator is not a static “merge all your mods” utility. The real Mods tree remains the source of truth. Accelerator-generated data lives outside the protected Sims 4 tree.

## Conservative execution path

```text
real Mods
 -> strict read-only discovery/DBPF parse
 -> persistent metadata/cache
 -> conflict-aware shadow plan
 -> generated front-indexed DBPF units
 -> prewarm
 -> launch TS4 FPB suspended
 -> inject narrow redirect runtime
 -> handshake
 -> resume game
```

## Before testing

1. Use a copy/test profile first.
2. Record the current game executable/build.
3. Snapshot/hash protected roots.
4. Build Rust and C++ on Windows x64.
5. Exercise the ordinary baseline before accelerated launch.
6. Keep identical mod content/settings for performance comparisons.

A build succeeding is only the beginning; current-game runtime equivalence and speed proof are required.
