## Verification

- `make test`: Rust tests with `ffi,cache`; warm 2.29s on the observed host, 79 passed and 2 ignored. The cache feature lets the registry example compile.
- `make lint`: workspace Clippy across targets; cold 91.51s on the observed host.
- Fork Rust API: candidate coverage, ordering, and `is_default_ignorable` in `tests/candidates.rs`.
- Fork C API: explicit script ranges in `tests/ffi_script_chain.rs`; candidates and freeing in `tests/candidates.rs`.
- CLI flags and exit codes, persisted and wire formats, endpoints: no fork additions.
- Performance workload: none; candidate ranking may be a hot path, but there is no repository benchmark.
- Downstream library consumer: laputa; test against a candidate checkout for API changes.
- Edit tier: `cargo test --features ffi,cache --test candidates` or `--test ffi_script_chain`; build cost depends on cache.
- Change tier: `make test` and `make lint`; tests include Rust and C API component contracts, with system-font cases ignored when unavailable.
- Scenario tier: the ignored installed-font test in `tests/candidates.rs` needs the named Noto and Arabic fonts. No Full benchmark is configured.
- Known flake: `waiting_for_fonts_resolves_no_chain` failed 1/5 full runs at base cf9f073 and 0/5 in the current tree under `cpu-guard --cores 4`; see RFC-1.
