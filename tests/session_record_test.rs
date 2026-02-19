use krabkrab::channels::session::{record_inbound_session, RecordInboundSessionParams, InboundLastRouteUpdate};
use serde_json::json;
use std::fs;
use tempfile::tempdir;

#[test]
fn record_and_read_session_file() {
    let td = tempdir().unwrap();
    let store = td.path().to_string_lossy().to_string();
    let params = RecordInboundSessionParams {
        store_path: store.clone(),
        session_key: "abc123".to_string(),
        ctx: json!({"user":"tester"}),
        create_if_missing: true,
        update_last_route: Some(InboundLastRouteUpdate {
            session_key: "abc123".to_string(),
            channel: "whatsapp".to_string(),
            to: "+123".to_string(),
            account_id: Some("acct1".to_string()),
            thread_id: Some("t1".to_string()),
        }),
    };
    record_inbound_session(params);
    let contents = fs::read_to_string(td.path().join("session-abc123.json")).unwrap();
    assert!(contents.contains("tester"));
    assert!(contents.contains("+123"));
}
