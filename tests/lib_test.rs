use openkrab::hello;

#[test]
fn hello_contains_version() {
    let h = hello();
    assert!(h.message.contains("openkrab") || h.message.contains("hello"));
    assert!(!h.version.is_empty());
}

