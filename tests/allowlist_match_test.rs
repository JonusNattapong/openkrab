use krabkrab::channels::allowlist_match::*;

#[test]
fn wildcard_allows() {
    let allow = vec!["*".to_string()];
    let m = resolve_allowlist_match_simple(&allow, "user1", Some("Alice"));
    assert!(m.allowed);
    assert_eq!(m.match_key.unwrap(), "*");
}

#[test]
fn id_and_name_match() {
    let allow = vec!["alice".to_string(), "bob".to_string()];
    let m1 = resolve_allowlist_match_simple(&allow, "alice", Some("Alice"));
    assert!(m1.allowed);
    assert_eq!(m1.match_source.unwrap(), AllowlistMatchSource::Id);

    let m2 = resolve_allowlist_match_simple(&allow, "charlie", Some("bob"));
    assert!(m2.allowed);
    assert_eq!(m2.match_source.unwrap(), AllowlistMatchSource::Name);
}
