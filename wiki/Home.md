# TS4 Accelerator

**Hide the cost of file count, not the user's mods.**

TS4 Accelerator is a Windows package-loading acceleration project for The Sims 4. The conservative design keeps the real Mods folder authoritative and builds a fast accelerator-owned package view under LocalAppData, then redirects only the game process' accelerator package reads.

## Status

| Area | State |
|---|---|
| Recovered V8 source | Yes |
| Strict DBPF parser/shard writer | Implemented |
| SQLite cache / Resource.cfg ordering | Implemented |
| Suspended launch + DLL handshake | Implemented in source |
| Current Windows build proof | Open |
| Current TS4 runtime proof | Open |
| Real 10k-package benchmark | Open |
| Tool compatibility / protected-data audit | Open |
| Advanced no-copy virtual DBPF | Experimental |

## Start here

- [Codex Master Execution](../docs/CODEX_MASTER_EXECUTION.md)
- [Developer Toolbox](../docs/DEVELOPER_TOOLBOX.md)
- [Recovered binary lineage](../RECOVERED_BINARY_ARTIFACTS.md)
- [Full TODO](TODO.md)

## Release rule

A build is not a win. A release is a win only when the exact current game + exact same mod workload is faster **and** conflict results, files, saves, tools and visible behavior remain equivalent.
