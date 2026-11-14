# Repository instructions

Read `.agents/foundry/AGENTS-adapter.md` for the pinned shared engineering core.
Keep this file and nested instructions project-owned: setup, commands, constraints and acceptance belong here.

## Repository-local accelerator contract

- Read README.md, SOURCE_RECOVERY_STATUS.md and docs/CODEX_MASTER_EXECUTION.md first. This recovered Windows-only Rust-first source has incomplete recovery/acceptance gates; verify actual files before following historical paths or commands.
- Preserve the original Mods, Saves, Tray, Resource.cfg and game installation. Accelerator-generated data must remain outside those protected roots; never merge or repair source packages in place.
- Preserve exact recovered source/version lineage. A later archive hash without matching recovered source is not proof of a newer implementation.
- Keep scanning, cache, ordering, shadow generation, launch/redirection and diagnostics independently owned with explicit contracts. Experimental paths cannot become mandatory dependencies of the conservative path.
- Check Cargo.toml and rust-toolchain.toml for actual workspace/toolchain requirements. Windows/native builds, current-game handshake, equivalent conflict winners, save/restart and protected-data audits remain separate acceptance gates from offline checks.
- Measure equivalent baseline/accelerated workloads without dropping content or changing conflict winners. Do not infer real-game performance or compatibility from fixtures or compilation.
