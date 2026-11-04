#![cfg(feature = "ffi")]

use rust_fontconfig::ffi::{
    fc_chain_query_for_text, fc_font_chain_free, fc_resolve_font_chain,
    fc_resolve_font_chain_with_scripts, fc_resolved_runs_free, fc_trace_free, FcFontFallbackChainC,
    FcTraceMsgC,
};
use rust_fontconfig::{FcFont, FcFontCache, FcPattern, FcWeight, PatternMatch, UnicodeRange};
use std::ffi::{c_char, CString};
use std::ptr;

const THAI: UnicodeRange = UnicodeRange { start: 0x0E00, end: 0x0E7F };

fn mock(family: &str, ranges: &[UnicodeRange]) -> (FcPattern, FcFont) {
    let pattern = FcPattern {
        name: Some(family.to_string()),
        family: Some(family.to_string()),
        unicode_ranges: ranges.to_vec(),
        ..Default::default()
    };
    let font = FcFont { bytes: vec![0, 1, 2, 3], font_index: 0, id: family.to_string() };
    (pattern, font)
}

fn cache() -> FcFontCache {
    let cache = FcFontCache::default();
    cache.with_memory_fonts(vec![
        mock("Base", &[UnicodeRange { start: 0x0000, end: 0x007F }]),
        mock("Thai Face", &[THAI]),
    ]);
    cache
}

fn resolves(cache: &FcFontCache, chain: *mut FcFontFallbackChainC, text: &str) -> bool {
    assert!(!chain.is_null());
    let text = CString::new(text).unwrap();
    let mut count = 0;
    let runs = fc_chain_query_for_text(chain, cache, text.as_ptr(), &mut count);
    assert!(!runs.is_null() && count == 1);
    let found = unsafe { (*runs).has_font };
    fc_resolved_runs_free(runs, count);
    fc_font_chain_free(chain);
    found
}

#[test]
fn script_ranges_build_a_fallback_the_default_chain_lacks() {
    let cache = cache();
    let family = CString::new("Base").unwrap();
    let families: [*const c_char; 1] = [family.as_ptr()];
    let mut trace: *mut FcTraceMsgC = ptr::null_mut();
    let mut trace_count = 0;

    let default_chain = fc_resolve_font_chain(
        &cache, families.as_ptr(), 1, FcWeight::Normal, PatternMatch::False, PatternMatch::False,
        &mut trace, &mut trace_count,
    );
    fc_trace_free(trace, trace_count);
    assert!(!resolves(&cache, default_chain, "\u{0E01}"));

    let scripts = [THAI];
    let script_chain = fc_resolve_font_chain_with_scripts(
        &cache, families.as_ptr(), 1, FcWeight::Normal, PatternMatch::False, PatternMatch::False,
        scripts.as_ptr(), scripts.len(), &mut trace, &mut trace_count,
    );
    fc_trace_free(trace, trace_count);
    assert!(resolves(&cache, script_chain, "\u{0E01}"));
}

#[test]
fn null_scripts_are_rejected() {
    let cache = cache();
    let family = CString::new("Base").unwrap();
    let families: [*const c_char; 1] = [family.as_ptr()];
    let mut trace: *mut FcTraceMsgC = ptr::null_mut();
    let mut trace_count = 0;
    let chain = fc_resolve_font_chain_with_scripts(
        &cache, families.as_ptr(), 1, FcWeight::Normal, PatternMatch::False, PatternMatch::False,
        ptr::null(), 1, &mut trace, &mut trace_count,
    );
    assert!(chain.is_null());
}
