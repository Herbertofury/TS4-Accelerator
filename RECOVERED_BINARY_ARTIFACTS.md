# Recovered Binary Artifact

The connected GitHub write surface does not expose raw binary uploads. The recovered V8 tree contains this generated DBPF fixture:

- Path: `fixtures/synthetic.package`
- Size: 800,040 bytes
- SHA-256: `57d527bda4c50701e6ea07c88ba62fe990fc81ea07a8cd545395ab3c3e10a9d5`

It is reproducible from the repository's generator/validator tooling and must not be silently replaced by an unverified fixture.

## Recovered source lineage

- `ts4-accelerator-v8-source(1).zip`
- SHA-256: `001281ee21b9d411d7f1dc40fa0272153340a100e377b0c7fd49ba1aa46de5a5`

Separate later record:

- `ts4-accelerator-v8-v0.3.4-source.zip`
- recorded SHA-256: `6f42e69116140e68fe850429cc5665df156a5bfca84d35fc895c5077782fee87`
- matching archive/source tree: **not recovered here**

Codex must reconcile this lineage before changing the declared release version.
