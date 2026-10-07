# Compatibility

The accelerator must coexist with the normal Sims ecosystem.

## Required proof

- Sims 4 Tray Importer sees the real expected files/content.
- Sims 4 Studio workflows continue against the real Mods tree.
- `.ts4script` loading remains ordinary.
- Replacing/updating a source package invalidates the right cache/shadow unit.
- Generated accelerator files are not mistaken for source-of-truth mods by update tooling.
- Save/load/restart behavior remains normal.

For each generated resource/shard, diagnostics should be able to answer which original source file owns it.
