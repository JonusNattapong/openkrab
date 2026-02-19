use futures::executor::block_on;
use futures::future::BoxFuture;
use krabkrab::channels::draft_stream_loop::*;
use std::sync::{Arc, Mutex};

fn make_send_counter() -> (
    Arc<Mutex<Vec<String>>>,
    Arc<dyn Fn(String) -> BoxFuture<'static, Option<bool>> + Send + Sync>,
) {
    let sent = Arc::new(Mutex::new(Vec::new()));
    let sent_clone = sent.clone();
    let f = Arc::new(move |text: String| {
        let s = sent_clone.clone();
        Box::pin(async move {
            s.lock().unwrap().push(text);
            Some(true)
        }) as BoxFuture<'static, Option<bool>>
    });
    (sent, f)
}

#[test]
fn test_flush_sends_pending() {
    let (sent, f) = make_send_counter();
    let is_stopped = Arc::new(|| false);
    let loopr = create_draft_stream_loop(100, is_stopped, f);
    loopr.update("hello".to_string());
    block_on(loopr.flush());
    let v = sent.lock().unwrap();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], "hello");
}
