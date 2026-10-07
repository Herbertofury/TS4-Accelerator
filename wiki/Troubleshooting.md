# Troubleshooting

## Accelerator refuses the workspace

Treat this as a safety feature. Verify the workspace does not intersect Documents\Electronic Arts\The Sims 4, Mods/Saves/Tray, the game installation, or a reparse/symlink path that escapes the workspace.

## Package becomes passthrough

Inspect the parser reason. Corrupt/ambiguous DBPF structures, duplicate TGIs inside one source package, unsupported layout, or range/size violations should not be force-merged merely to improve the “merged count.”

## Game does not resume

Inspect injection/handshake diagnostics. Never bypass the handshake and resume with a half-installed runtime.

## “Faster” result changes content

That benchmark fails. Compare Resource.cfg ordering and conflict winners first; do not tune further until equivalence is restored.

## Cache seems stale

Use source path/size/write-time/parser-version/index metadata and content-addressed generated identity to determine why reuse occurred. A one-package change should rebuild only what its dependency/conflict plan requires.
