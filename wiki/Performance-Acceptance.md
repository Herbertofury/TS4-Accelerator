# Performance Acceptance

“Faster” requires equivalent work and equivalent results.

## Required measurements

- cold startup -> main menu / first stable frame;
- warm startup;
- one-package incremental update/relaunch;
- representative lot/zone transition;
- frame-time/hitch distribution;
- CPU;
- peak/steady RAM;
- disk bytes/IOPS/queue behavior.

Record the exact TS4 build, mod/package count, total bytes, storage device, Windows build, GPU/driver, shader state and Memory Boost state.

## Invalid wins

Do not accept speed caused by:

- loading fewer mods/resources;
- changing conflict winners/load order;
- hiding scripts/config files;
- skipping validation;
- losing tool compatibility;
- consuming materially more memory/disk in a way that simply moves the cost elsewhere.

Use PresentMon and WPR/WPA alongside accelerator-owned timing so the benchmark has independent evidence.
