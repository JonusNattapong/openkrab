use krabkrab::channels::channel_config::*;
use std::collections::HashMap;

#[test]
fn normalize_slug_examples() {
    assert_eq!(normalize_channel_slug("#Hello World!"), "hello-world");
    assert_eq!(normalize_channel_slug("  Foo_bar  "), "foo-bar");
    assert_eq!(normalize_channel_slug("---Baz---"), "baz");
}

#[test]
fn build_candidates_and_resolve() {
    let keys = build_channel_key_candidates(vec![Some("a"), Some("b"), Some("a"), None]);
    assert_eq!(keys, vec!["a", "b"]);

    let mut entries: HashMap<String, i32> = HashMap::new();
    entries.insert("a".to_string(), 1);
    entries.insert("*".to_string(), 99);

    let match1 = resolve_channel_entry_match(&entries, &vec!["a".to_string()], Some("*"));
    assert_eq!(match1.entry, Some(1));
    assert_eq!(match1.key, Some("a".to_string()));

    let match2 = resolve_channel_entry_match_with_fallback(
        &entries,
        &vec!["x".to_string()],
        None,
        Some("*"),
        None::<fn(&str) -> Option<String>>,
    );
    assert_eq!(match2.entry, Some(99));
    assert_eq!(match2.match_source, Some(ChannelMatchSource::Wildcard));
}

#[test]
fn nested_allowlist() {
    assert!(resolve_nested_allowlist_decision((
        false, false, false, false
    )));
    assert!(!resolve_nested_allowlist_decision((
        true, false, true, true
    )));
    assert!(resolve_nested_allowlist_decision((
        true, true, false, false
    )));
    assert!(resolve_nested_allowlist_decision((true, true, true, true)));
}
