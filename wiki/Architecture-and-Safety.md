# Architecture & Safety

## Source of truth

The real Sims 4 Mods/Saves/Tray/game installation remains authoritative and must not be rewritten by the accelerator.

Generated state belongs under `%LOCALAPPDATA%\TS4Accelerator` or another explicitly safe accelerator workspace.

## Redirect scope

The runtime may redirect only accelerator-owned package namespace/Resource.cfg paths required for the accelerated view. Original `.ts4script`, mod config/data and normal user files must remain visible.

## Fail-closed rules

- Hook/handshake uncertainty -> do not resume into a partially active accelerator.
- DBPF parse uncertainty -> passthrough or reject; never silently reinterpret.
- Resource.cfg ordering ambiguity -> stop instead of guessing.
- Source changed during generation -> invalidate/abort.
- Workspace intersects protected/reparse-escaped path -> block writes.

## Release architecture

Physical generated DBPF shadow units + narrow redirection remain the conservative release path. No-copy virtual DBPF, decompression interposition and arbitrary hot activation stay experimental until the conservative path has real proof.
