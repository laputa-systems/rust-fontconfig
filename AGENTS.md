## Verification

- `make test`: Rust tests with `ffi,cache`; warm 2.29s on the observed host, 79 passed and 2 ignored. The cache feature lets the registry example compile.
- `make lint`: workspace Clippy across targets; cold 91.51s on the observed host.
- Fork Rust API: candidate coverage and ordering in `tests/candidates.rs`; direct `is_default_ignorable` boundary: none.
- Fork C API: explicit script ranges in `tests/ffi_script_chain.rs`; candidates and freeing in `tests/candidates.rs`.
- CLI flags and exit codes, persisted and wire formats, endpoints: no fork additions.
- Performance workload: none; candidate ranking may be a hot path, but there is no repository benchmark.
- Downstream library consumer: laputa; test against a candidate checkout for API changes.
