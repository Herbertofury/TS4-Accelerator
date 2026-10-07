# TS4 Accelerator — Developer Toolbox

Codex should evaluate these before reinventing equivalent components.

## Sims/DBPF references

- `https://thesims4moddersreference.org/reference/dbpf-format/`
- `https://thesims4moddersreference.org/reference/internal-compression-dbpf/`
- `https://thesims4moddersreference.org/reference/file-types/`
- `https://github.com/ytaa/dbpf_reader`
- `https://github.com/sims4toolkit/compression`
- `https://github.com/stark-studio-labs/sims4-stark-devkit`

## Windows/runtime

- Microsoft Projected File System (ProjFS) documentation.
- Windows I/O rings documentation.
- `PrefetchVirtualMemory` documentation.
- Windows Performance Recorder / Windows Performance Analyzer.
- `https://github.com/GameTechDev/PresentMon`
- `https://github.com/TsudaKageyu/minhook`
- `https://github.com/microsoft/Detours`
- `https://frida.re/` for investigation/prototyping, not necessarily shipping.

## Compression

- `https://github.com/ebiggers/libdeflate`
- `https://github.com/zlib-ng/zlib-ng`

## Existing architecture candidates

- Physical front-indexed shadow DBPF units: conservative release path.
- ProjFS/WinFsp: evaluate only with real latency/file-count proof.
- Tiny NTFS stub + in-process virtual DBPF: advanced path after conservative runtime proof.
- USN Journal incremental discovery: candidate after correct fallback and invalidation semantics are proven.

## Candidate disposition rule

For every integration candidate record: benchmark/reason, use-as-is/adapt/reference/reject, license/provenance, fallback, and exact acceptance test.
