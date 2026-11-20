Prefix: DSLP
Next: 8

Deslop campaign backlog ([plan](https://github.com/laputa-systems/stuff/blob/main/docs/campaigns/deslop.md), tier E). One agent works this file with the `deslop` skill: take the lowest-numbered open ticket whose area does not start with `Needs owner ·`. After DSLP-5, work the open P1 and P2 findings, then fill in `## Report` below.

## Report

Not started.

## Tickets

### DSLP-6 · P2 · Recognize all default-ignorable code points

- **Area:** font candidates
- **Where:** src/fallback.rs:205
- **Problem:** U+1BCA0 has the Unicode Default_Ignorable_Code_Point property but is_default_ignorable returns false; candidate coverage can reject text for a character that requires no glyph. A focused test fails and the pinned Rust Unicode property reports true.
- **Recommendation:** Cover the complete Unicode property and retain a regression at the candidate boundary.
- **Fix with:** root-cause
- **Effort:** S

### DSLP-7 · P3 · Stabilize registry memo assertion during background builds

- **Area:** deslop · tests
- **Where:** tests/registry_completion.rs:135; src/lib.rs:1517
- **Problem:** Under cpu-guard --cores 4 the existing waiting_for_fonts_resolves_no_chain full-suite test failed 1 of 5 times at base cf9f073 and 0 of 5 times in the current tree; it observed one memo entry instead of two. An unproven hypothesis is that a background font insertion clears the memo between resolutions.
- **Recommendation:** Determine the interleaving and make the test or registry synchronization deterministic without weakening the behavior contract.
- **Fix with:** root-cause
- **Effort:** M

