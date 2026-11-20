Prefix: DSLP
Next: 11

Deslop campaign backlog ([plan](https://github.com/laputa-systems/stuff/blob/main/docs/campaigns/deslop.md), tier E). One agent works this file with the `deslop` skill: take the lowest-numbered open ticket whose area does not start with `Needs owner ·`. After DSLP-5, work the open P1 and P2 findings, then fill in `## Report` below.

## Report

Base: campaign `d997582dd9afa4903ab26a1f336fff74ba66ad72`; admission `cf9f073d744202db17204db2a6cf3a909c5da89f`; upstream merge base `966dc9c8187601ec4ad206a095a5f48fbe490279`. Scope is fork additions only.

Scorecard: fork Rust source additions 257 to 260 lines; fork test additions 227 to 240 lines; C header additions 57 lines. The initial warm `make test` took 2.29s (79 passed, 2 ignored); after the fix it took 27.60s under concurrent host load (80 passed, 2 ignored). Summed runner-reported execution times were about 0.85s and 1.11s, within the 30s test budget. Line and branch coverage: Needs environment (`cargo-llvm-cov` unavailable). Automated mutants caught/missed/timeout/unviable: not measured; `cargo-mutants` unavailable. Fork escape-hatch matches: 3 before and after. Removed: 0 lines, 0 dependencies. Findings: one P2 filed and fixed (DSLP-6); three P3 open (DSLP-7, DSLP-9, DSLP-10); no known escaped regression count. Performance verdict: unmeasured, no benchmark exists.

Steps and observed costs (wall times include compilation or lock contention; CI minutes unavailable because this is a local-only pass):

- Ground: added FFI and cache test entry point and PR invocation; initial cold test attempt 187.48s failed to compile the registry example without cache; corrected run 66.33s, warm 2.29s; cold lint 91.51s.
- Diagnose: reproduced the missing default-ignorable range and filed DSLP-6; suite 10.77s and lint 32.91s. Review was limited to the fork diff.
- Types: combined candidate Clippy run took 44.50s, with 53 total library warnings; suite 29.89s and lint 1.33s. No strict setting was adopted: no warning on added fork lines supplied a witness. Rejected candidates, crate-wide warning counts: wildcard enum 6; match wildcard single variant 0; truncation 7; sign loss 2; unwrap 4; expect 3; redundant clone 2; needless pass by value 3. `unsafe_code=forbid` rejects the fork's C pointer handling (3 added escape-hatch matches). Counts are not a classification of each upstream warning as false or true.
- Tests: one direct predicate test added. Manual injected faults caught: partial coverage (`all` to `any`), ranking reversal, null candidate result, ignored script ranges, non-null null-script result, omitted ignorable filtering, removal of U+200D, and removal of U+1BCA0 (8 caught, 0 missed in this chosen scope; no exhaustive survivor claim). The last two new or expanded boundary assertions failed before restoration. Base/current flake comparison: 4/5 versus 5/5 full-suite passes; base failed the named upstream assertion once. Needs environment: coverage and automated mutation tools, installed-font scenario. No mutation CI lane was added per owner ruling.
- Dead code: rustc reported an upstream-only unused `clipped` helper; fork symbols are referenced by the candidate path, tests, or C header. No behavior-preserving cut was established. Needs environment: cargo-machete and cargo-udeps unavailable; no dependency removal claimed. Lint 24.68s; full test evidence from current comparison.
- Bench: no bench files or harness found. Outcomes without coverage: font candidate selection latency, CPU, memory, and scaling; DSLP-9 records the highest-value gap. No Edit or Change benchmark can run without a harness; Full not run. Suite 27.59s, lint 1.30s.
- DSLP-6: added the three Unicode property ranges missing from the fork predicate. The pre-fix test failed; the fixed focused suite passed, and deleting U+1BCA0 again made both predicate and candidate tests fail. Full suite 27.60s, lint 42.23s. Contract effect: previously rejected text containing these default-ignorable code points can now retain a covering font.
- Report: cumulative diff reviewed locally; the `test` target false-pass case was reproduced and filed as P3 DSLP-10. Review cost not separately timed.

Needs environment or owner coordination: laputa consumer tests against this candidate were not run because the lane cannot touch another repository; hosted CI was not run; installed-font scenario, coverage and mutation tools, and dependency scanners were unavailable. No performance claim is made on the busy host. Remaining tickets: DSLP-7 (pre-existing flake), DSLP-9 (benchmark gap), DSLP-10 (`test` target can be shadowed by a file). No open P1/P2.

Method feedback: a phony-target negative control would have caught DSLP-10 during Ground. A single suite failure was insufficient to assign the registry flake to the fork; the paired base/current runs corrected that conclusion. Shared-file gaps: no qualified mirrored Rust coverage or mutation tools are available for this lane; no shared files were changed.

## Tickets

### DSLP-7 · P3 · Stabilize registry memo assertion during background builds

- **Area:** deslop · tests
- **Where:** tests/registry_completion.rs:135; src/lib.rs:1517
- **Problem:** Under cpu-guard --cores 4 the existing waiting_for_fonts_resolves_no_chain full-suite test failed 1 of 5 times at base cf9f073 and 0 of 5 times in the current tree; it observed one memo entry instead of two. An unproven hypothesis is that a background font insertion clears the memo between resolutions.
- **Recommendation:** Determine the interleaving and make the test or registry synchronization deterministic without weakening the behavior contract.
- **Fix with:** root-cause
- **Effort:** M

### DSLP-9 · P3 · Measure representative font candidate selection

- **Area:** deslop · benchmarks
- **Where:** src/fallback.rs:234; src/lib.rs:1608
- **Problem:** There is no benchmark for the fork candidate-selection journey, including repeated font table reads, latency, CPU cost, or memory as chain and text sizes grow. No performance regression verdict is possible.
- **Recommendation:** Build a representative candidate-selection workload with a correctness oracle and measure CPU and memory across realistic chain and text sizes before setting budgets.
- **Fix with:** bench-pyramid
- **Effort:** M

### DSLP-10 · P3 · Declare the test target phony

- **Area:** deslop · tests
- **Where:** Makefile:36; Makefile:84
- **Problem:** A file named test in the working directory makes make test report it up to date and skip the suite. Reproduced using the repository Makefile in a scratch directory containing such a file.
- **Recommendation:** Declare test phony and verify the target still runs when that filename exists.
- **Fix with:** test-audit
- **Effort:** S

