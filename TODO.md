Prefix: RFC
Next: 4

# Open work
### RFC-1 · P3 · Stabilize registry memo assertion during background builds

- **Area:** tests
- **Where:** tests/registry_completion.rs:135; src/lib.rs:1517
- **Problem:** Under cpu-guard --cores 4 the existing waiting_for_fonts_resolves_no_chain full-suite test failed 1 of 5 times at base cf9f073 and 0 of 5 times in the current tree; it observed one memo entry instead of two. An unproven hypothesis is that a background font insertion clears the memo between resolutions.
- **Recommendation:** Determine the interleaving and make the test or registry synchronization deterministic without weakening the behavior contract.
- **Fix with:** root-cause
- **Effort:** M

### RFC-2 · P3 · Measure representative font candidate selection

- **Area:** benchmarks
- **Where:** src/fallback.rs:234; src/lib.rs:1608
- **Problem:** There is no benchmark for the fork candidate-selection journey, including repeated font table reads, latency, CPU cost, or memory as chain and text sizes grow. No performance regression verdict is possible.
- **Recommendation:** Build a representative candidate-selection workload with a correctness oracle and measure CPU and memory across realistic chain and text sizes before setting budgets.
- **Fix with:** bench-pyramid
- **Effort:** M

### RFC-3 · P3 · Declare the test target phony

- **Area:** tests
- **Where:** Makefile:36; Makefile:84
- **Problem:** A file named test in the working directory makes make test report it up to date and skip the suite. Reproduced using the repository Makefile in a scratch directory containing such a file.
- **Recommendation:** Declare test phony and verify the target still runs when that filename exists.
- **Fix with:** test-audit
- **Effort:** S

