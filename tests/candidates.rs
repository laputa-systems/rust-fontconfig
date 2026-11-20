#![cfg(all(feature = "std", feature = "parsing"))]

use rust_fontconfig::{
    FcFont, FcFontCache, FcPattern, FcWeight, FontFallbackChain, PatternMatch, TraceMsg, UnicodeRange,
};

const LATN: u32 = u32::from_be_bytes(*b"latn");
const ARAB: u32 = u32::from_be_bytes(*b"arab");

#[test]
fn default_ignorable_distinguishes_format_controls_and_letters() {
    assert!(rust_fontconfig::fallback::is_default_ignorable('\u{200D}'));
    assert!(!rust_fontconfig::fallback::is_default_ignorable('a'));
}

fn serif_font() -> (FcPattern, FcFont) {
    let pattern = FcPattern {
        name: Some("Instrument Serif".to_string()),
        family: Some("Instrument Serif".to_string()),
        ..Default::default()
    };
    let font = FcFont {
        bytes: include_bytes!("fixtures/InstrumentSerif-Regular.ttf").to_vec(),
        font_index: 0,
        id: "serif".to_string(),
    };
    (pattern, font)
}

fn unreadable_font(family: &str, ranges: &[(u32, u32)]) -> (FcPattern, FcFont) {
    let pattern = FcPattern {
        name: Some(family.to_string()),
        family: Some(family.to_string()),
        unicode_ranges: ranges.iter().map(|(start, end)| UnicodeRange { start: *start, end: *end }).collect(),
        ..Default::default()
    };
    let font = FcFont { bytes: vec![0, 1, 2, 3], font_index: 0, id: family.to_string() };
    (pattern, font)
}

fn chain(cache: &FcFontCache, families: &[&str]) -> FontFallbackChain {
    let mut trace: Vec<TraceMsg> = Vec::new();
    let families: Vec<String> = families.iter().map(|family| family.to_string()).collect();
    cache.resolve_font_chain_with_scripts(
        &families,
        FcWeight::Normal,
        PatternMatch::False,
        PatternMatch::False,
        None,
        &mut trace,
    )
}

#[test]
fn candidates_cover_the_whole_text_and_report_script_support() {
    let cache = FcFontCache::default();
    cache.with_memory_fonts(vec![serif_font()]);
    let chain = chain(&cache, &["Instrument Serif"]);

    let latin = chain.candidates_for_text(&cache, "a", &[LATN]);
    assert_eq!(latin.len(), 1);
    assert!(latin[0].supports_script);
    assert!(!latin[0].color);

    let wrong_script = chain.candidates_for_text(&cache, "a", &[ARAB]);
    assert_eq!(wrong_script.len(), 1);
    assert!(!wrong_script[0].supports_script);

    assert!(chain.candidates_for_text(&cache, "\u{628}", &[ARAB]).is_empty());
    assert!(chain.candidates_for_text(&cache, "a\u{628}", &[]).is_empty(), "partial coverage is not a candidate");
    assert_eq!(chain.candidates_for_text(&cache, "a\u{200D}\u{FE0F}", &[LATN]).len(), 1, "ignorables need no glyph");
}

#[test]
fn a_font_declaring_the_script_outranks_an_earlier_one_that_does_not() {
    let cache = FcFontCache::default();
    cache.with_memory_fonts(vec![unreadable_font("Wide", &[(0x20, 0x7E)]), serif_font()]);
    let chain = chain(&cache, &["Wide", "Instrument Serif"]);

    let order = chain.candidates_for_text(&cache, "a", &[LATN]);
    assert_eq!(order.len(), 2);
    assert!(order[0].supports_script && !order[1].supports_script);
}

#[test]
#[ignore = "needs Noto Sans Math, an Arabic font, and Noto Color Emoji installed"]
fn arabic_beh_prefers_a_font_that_shapes_arabic_over_noto_sans_math() {
    let cache = FcFontCache::build();
    let mut trace: Vec<TraceMsg> = Vec::new();
    let scripts = [
        UnicodeRange { start: 0x0600, end: 0x06FF },
        UnicodeRange { start: 0x1F300, end: 0x1FAFF },
    ];
    let chain = cache.resolve_font_chain_with_scripts(
        &["sans-serif".to_string()],
        FcWeight::Normal,
        PatternMatch::False,
        PatternMatch::False,
        Some(&scripts),
        &mut trace,
    );
    let beh = chain.candidates_for_text(&cache, "\u{628}", &[ARAB]);
    let path = |candidate: &rust_fontconfig::FontCandidate| match cache.get_font_by_id(&candidate.id) {
        Some(rust_fontconfig::OwnedFontSource::Disk(path)) => path.path,
        other => panic!("{other:?}"),
    };
    assert!(beh[0].supports_script, "{:?}", beh.iter().map(path).collect::<Vec<_>>());
    assert!(!path(&beh[0]).contains("NotoSansMath"), "{}", path(&beh[0]));

    let emoji = chain.candidates_for_text(&cache, "\u{1F3AC}", &[]);
    assert!(emoji.iter().any(|candidate| candidate.color));
}

#[cfg(feature = "ffi")]
#[test]
fn ffi_candidates_round_trip_and_reject_null() {
    use rust_fontconfig::ffi::{
        fc_chain_candidates_for_text, fc_font_candidates_free, fc_font_chain_free, fc_resolve_font_chain,
        fc_trace_free,
    };
    use std::ffi::{c_char, CString};
    use std::ptr;

    let cache = FcFontCache::default();
    cache.with_memory_fonts(vec![serif_font()]);
    let family = CString::new("Instrument Serif").unwrap();
    let families: [*const c_char; 1] = [family.as_ptr()];
    let mut trace = ptr::null_mut();
    let mut trace_count = 0;
    let chain = fc_resolve_font_chain(
        &cache, families.as_ptr(), 1, FcWeight::Normal, PatternMatch::False, PatternMatch::False,
        &mut trace, &mut trace_count,
    );
    fc_trace_free(trace, trace_count);
    assert!(!chain.is_null());

    let text = CString::new("a").unwrap();
    let tags = [LATN];
    let mut count = 7;
    let found = fc_chain_candidates_for_text(chain, &cache, text.as_ptr(), tags.as_ptr(), 1, &mut count);
    assert!(!found.is_null() && count == 1);
    fc_font_candidates_free(found, count);

    let missing = CString::new("\u{628}").unwrap();
    let none = fc_chain_candidates_for_text(chain, &cache, missing.as_ptr(), ptr::null(), 0, &mut count);
    assert!(none.is_null() && count == 0);

    let rejected = fc_chain_candidates_for_text(chain, &cache, text.as_ptr(), ptr::null(), 1, &mut count);
    assert!(rejected.is_null());
    fc_font_chain_free(chain);
}
