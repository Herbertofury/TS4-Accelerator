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
