# Package-processing alternatives and baseline methods

## Sources and comparable workflows

[s4pi-reforged](https://github.com/HrBingR/s4pi-reforged) documents Rust package merge/unmerge tooling and compatibility goals with Sims 4 Studio. It is a relevant correctness and workflow reference, not proof of faster actual game loading.

## How this project should earn its place

Compare untouched loose packages against generated shadow data and an explicitly reversible merge baseline. Measure cold/warm game load, package/index correctness, conflict ordering, RAM, disk cost and rebuild cost after a small update. Verify resource winners, manifests and round-trip identity; never trade read-only original Mods/Saves/Tray safety for a headline speedup.

## Evidence and maintenance

Source review: 2026-11-12. Scope: documented capabilities and proposed evaluation criteria, not installed comparative tests or a latest-version certification. Unknown is not unsupported. No measured superiority is claimed.

For each meaningful upstream change, record the source URL, publication/version date, review date, affected workflow, licensing/access constraints, opportunity and reproducible acceptance test. Preserve last-good findings and failed tests. Recheck primary sources before implementation decisions; do not copy proprietary code/assets or change the product scope merely to match another tool. Proposed features remain proposals until authorized. Daily review should record no change honestly rather than invent updates.

## Next comparison pass

- [ ] Pin exact compared versions and representative workloads.
- [ ] Capture source evidence and verify applicable licenses.
- [ ] Measure current implementation before a replacement or optimization.
- [ ] Record correctness, usability, resource cost and recovery results together.
- [ ] Link only proven improvements to implementation milestones; retain unproven candidates as research.

## Reuse foundations and concrete parts to evaluate

[s4pi-reforged](https://github.com/HrBingR/s4pi-reforged) is a candidate differential merge/unmerge oracle. Inspect index handling, manifest preservation and streaming IO before choosing reusable modules; do not replace the existing accelerator solely because both use Rust.

## Integration with our projects

Hover and External-CAS could share a bounded read-only DBPF metadata contract and public/synthetic malformed fixtures. Keep accelerator-generated shadow data separate from their original-file views and all actual Mods/Saves/Tray.

## Target improvement and acceptance boundary

Target faster real game loading and incremental rebuilds, not just faster generated microbenchmarks. Require identical effective resource winners, rollback and bounded disk growth.

## License, access and adoption gate

Status: research candidate, not integrated or benchmark-proven. Before copying code, pin the exact upstream revision and record its LICENSE, dependency licenses, notices and distribution obligations. Missing/ambiguous terms block reuse; behavioral study or a permitted interoperable adapter may remain possible. Service documentation is not a source-code license or API grant. Preserve existing user permissions, private/public boundaries and creator/game asset rights.

Choose reuse, adapter, fork or independent implementation only after a target-workload comparison. Retain raw failures and rollback; share no credentials, private fixtures or account details. Adoption remains blocked until licensing, correctness, performance and maintenance cost are reviewed together. Review delta dated 2026-11-12.
