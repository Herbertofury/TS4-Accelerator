# TS4 Accelerator — Codex Master Execution Contract

Last reconciled: 2026-10-06
Recovered baseline: V8 source / Cargo 0.3.0
Known later lineage evidence: v8-v0.3.4 source archive hash only; matching archive not recovered

## Objective

Finish TS4 Accelerator as a measurable, no-loss Windows performance layer for The Sims 4 that reduces startup/package-loading cost and runtime hitching on large mod libraries without modifying the user's real Mods/Saves/Tray/game installation in place.

## Context

The recovered V8 tree already implements the conservative architecture: parallel package discovery, strict DBPF parsing, persistent SQLite metadata, Resource.cfg-aware ordering, conflict-aware physical shadow planning, front-indexed generated DBPF units, unchanged-unit reuse, prewarm, suspended FPB launch, DLL injection and a narrow path-redirect runtime handshake.

Do not restart this as a generic launcher or static package merger.

## Constraints

- Real Sims 4 content/install paths are read-only from the accelerator's generated-data path.
- No speed win is valid if content, conflicts, load order, saves, tools or visible quality regress.
- Measure equivalent baseline and accelerated workloads.
- Build/static success is not live-game proof.
- Physical shadow + narrow redirection is the conservative release path until proven.
- No-copy virtual DBPF, decompression interposition and arbitrary hot activation are experimental until the conservative path is runtime-proven.
- Preserve exact artifact/source provenance and recovered version lineage.
- Do not claim 0.3.4 source unless the missing archive is reconstructed and its SHA-256 matches the recorded hash.

## Resume rule

Continue from the earliest unchecked/invalidated task whose dependencies are satisfied. Work in bounded internal windows of roughly 6–12 ready tasks. Record blockers as `BLOCKED: ...; NEXT: ...`. Two materially unchanged failed attempts require a strategy change.

## Phase A — source lineage and reproducibility

- [ ] **T001** · Verify the recovered V8 archive SHA-256 and repository manifest.
- [ ] **T002** · Reconcile Cargo/package version 0.3.0 with the later recorded v0.3.4 archive hash; do not invent missing changes.
- [ ] **T003** · Normalize build/release version metadata so executable, DLL, docs and package artifacts agree.
- [ ] **T004** · Generate a deterministic source manifest and release manifest.
- [ ] **T005** · Preserve the historical specs/reports but clearly mark which architecture is authoritative.
- [ ] **G001 · GATE** — Source lineage is reproducible and version claims match recovered evidence.

## Phase B — Windows build and runtime handshake

- [ ] **T006** · Build Rust workspace on current Windows x64 MSVC using the pinned toolchain or document the verified compatible update.
- [ ] **T007** · Build the C++23 runtime DLL with the same release configuration Codex will ship.
- [ ] **T008** · Verify the launcher starts the correct TS4 FPB executable suspended.
- [ ] **T009** · Verify DLL injection succeeds before the primary game thread resumes.
- [ ] **T010** · Verify the runtime handshake fails closed if hooks are not active.
- [ ] **T011** · Verify only accelerator-owned `Resource.cfg` / `__TS4Accel` paths are redirected.
- [ ] **T012** · Verify all unrelated real Mods/config/script paths remain visible.
- [ ] **T013** · Add current-build diagnostics for hook targets/API availability rather than assuming old offsets/imports.
- [ ] **G002 · GATE** — Current Windows build launches the current game with a proven, narrow, fail-closed handshake.

## Phase C — DBPF correctness and conflict equivalence

- [ ] **T014** · Re-run the independent 10,000-resource DBPF validator.
- [ ] **T015** · Preserve DBPF 2.1/index 0.3 bounds, constant flags 0–7 and TS4 reserved-header semantics.
- [ ] **T016** · Reject/passthrough payload overlap, corrupt sizes, invalid index ranges and duplicate-TGI generated units.
- [ ] **T017** · Preserve raw/zlib/internal/unknown compression metadata byte-for-byte unless a proven transformation is explicitly enabled.
- [ ] **T018** · Build a real-mod conflict fixture and compare ordinary TS4 winner/order with accelerated winner/order.
- [ ] **T019** · Verify Resource.cfg priority/rule ordering with nested paths and mixed case.
- [ ] **T020** · Verify unchanged source packages reuse cached metadata only when all invalidation keys still match.
- [ ] **T021** · Verify source mutation during generation invalidates/aborts the build rather than publishing stale shadows.
- [ ] **G003 · GATE** — Generated package view is structurally valid and conflict/load-order equivalent to ordinary TS4 behavior.

## Phase D — incremental rebuild and file-count attack

- [ ] **T022** · Profile cold discovery, warm discovery and one-file incremental change on a large real mod tree.
- [ ] **T023** · Add/verify USN Journal or another Windows-native change-tracking path only if it materially beats safe metadata scanning.
- [ ] **T024** · Ensure change tracking falls back cleanly when journal access/state is unavailable.
- [ ] **T025** · Bound CPU/concurrency so rebuilding does not fight TS4 launch for the same disk/CPU resources.
- [ ] **T026** · Keep generated unit identity content-addressed so unchanged units are not rewritten.
- [ ] **T027** · Make atomic shadow-generation swap crash-safe.
- [ ] **T028** · Verify one changed package rebuilds only the minimal necessary unit(s).
- [ ] **G004 · GATE** — Warm/incremental launches avoid unnecessary whole-tree work without changing results.

## Phase E — prewarm and decompression challengers

- [ ] **T029** · Measure index/page-fault/file-I/O cost before changing prewarm strategy.
- [ ] **T030** · Verify `PrefetchVirtualMemory` or equivalent prewarm is beneficial on representative SSD/NVMe systems before enabling by default.
- [ ] **T031** · Evaluate Windows I/O rings only against measured bottlenecks and equivalent results.
- [ ] **T032** · Measure actual zlib/internal decompression share in TS4 before interposing any faster library.
- [ ] **T033** · Benchmark libdeflate and zlib-ng on captured representative payloads if decompression is proven material.
- [ ] **T034** · Reject faster decompression routes that cannot prove byte-exact output and safe calling semantics.
- [ ] **G005 · GATE** — Prewarm/decompression changes are retained only when measured and result-equivalent.

## Phase F — tool and ecosystem compatibility

- [ ] **T035** · Verify Sims 4 Tray Importer still sees real package/script/config content correctly.
- [ ] **T036** · Verify Sims 4 Studio workflows against the real Mods tree are unaffected.
- [ ] **T037** · Verify mod updater/manager behavior does not target generated shadows as source-of-truth content.
- [ ] **T038** · Verify package replacement/update followed by accelerator rebuild yields the new content without stale cache.
- [ ] **T039** · Verify `.ts4script` loading remains ordinary and is not hidden by package redirection.
- [ ] **T040** · Add a diagnostic explaining which source file owns any generated shard/resource.
- [ ] **G006 · GATE** — Normal Sims modding tools and script/config workflows remain intact.

## Phase G — protected-data and save safety

- [ ] **T041** · Snapshot hashes/metadata for Saves, Tray, Screenshots, real Resource.cfg, Mods and game install before accelerated runs.
- [ ] **T042** · Run accelerated launch/play/save/exit/restart.
- [ ] **T043** · Prove protected paths changed only where the game/user normally changes them; accelerator generated writes stay outside protected roots.
- [ ] **T044** · Verify workspace-intersection and reparse/symlink escape guards.
- [ ] **T045** · Verify crash/interrupted build cannot replace/corrupt the user's real Mods tree.
- [ ] **G007 · GATE** — Protected-data audit and save/restart behavior are runtime-proven.

## Phase H — real performance acceptance

- [ ] **T046** · Define one equivalent benchmark pack near the user's real scale (target ~10,000 packages where practical).
- [ ] **T047** · Record ordinary TS4 cold startup to main menu/first stable frame.
- [ ] **T048** · Record accelerated cold startup with identical content/result set.
- [ ] **T049** · Record ordinary and accelerated warm startup.
- [ ] **T050** · Record one-package incremental update/relaunch.
- [ ] **T051** · Record lot/zone transition and representative hitch/frame-time metrics.
- [ ] **T052** · Record CPU, memory and disk I/O so a faster startup is not accepted if it causes a material resource regression elsewhere.
- [ ] **T053** · Store benchmark environment, TS4 build, storage, mod count, bytes, shader/memory-boost state and driver metadata.
- [ ] **G008 · GATE** — At least one target hot path is measurably faster on equivalent content with no material protected regression.

## Phase I — advanced experimental path

- [ ] **T054** · Re-evaluate tiny NTFS stub + in-process virtual DBPF serving only after G008.
- [ ] **T055** · If implemented, prove virtual offsets map to exact original payload bytes and preserve TS4 mapping/read semantics.
- [ ] **T056** · Implement hot staging as a distinct state from successful in-game activation.
- [ ] **T057** · Attempt safe-boundary activation before arbitrary resource-manager hooks.
- [ ] **T058** · Never label a newly dropped package “live” until the game has actually observed the intended resource state.
- [ ] **G009 · GATE** — Any promoted experimental path beats the conservative baseline with full correctness/runtime proof.

## Release convergence

- [ ] **T059** · Run cargo format/lint/test, C++ build, static source checks and independent DBPF validators.
- [ ] **T060** · Run real current-game launch, package conflict, tool compatibility, save/restart and benchmark gates on the exact release build.
- [ ] **T061** · Produce source archive, binaries, SHA-256 manifests, changelog, third-party notices and exact install/uninstall/rollback docs.
- [ ] **T062** · Run one bounded challenger scan of current public DBPF/filesystem/hooking/performance tooling and integrate only proven compatible wins.

- [ ] **G010 · FINAL COMPLETION GATE** — All accepted tasks/gates are complete; the exact release build is Windows-build-proven and current-game-runtime-proven; protected content is safe; ordinary and accelerated content/conflict behavior match; third-party tools remain usable; equivalent benchmarks show a real target-path gain without material CPU/memory/disk/quality regression; artifacts and hashes are published; and no missing 0.3.4 lineage or experimental feature is misrepresented as verified.
