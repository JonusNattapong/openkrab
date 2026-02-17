use krabkrab::hello;

#[test]
fn hello_contains_version() {
    let h = hello();
    assert!(h.message.contains("krabkrab") || h.message.contains("hello"));
    assert!(!h.version.is_empty());
}
