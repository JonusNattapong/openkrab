use futures::executor::block_on;
use futures::future::BoxFuture;
use krabkrab::channels::typing::*;
use std::sync::Arc;

fn boxed_ok() -> BoxFuture<'static, Result<(), String>> {
    Box::pin(async move { Ok(()) })
}

#[test]
fn test_create_typing_callbacks_runs_start() {
    let started = std::sync::Arc::new(std::sync::Mutex::new(false));
    let started_clone = started.clone();
    let start: Arc<dyn Fn() -> BoxFuture<'static, Result<(), String>> + Send + Sync> =
        Arc::new(move || {
            let s = started_clone.clone();
            Box::pin(async move {
                let mut g = s.lock().unwrap();
                *g = true;
                Ok(())
            })
        });

    let on_start_error = Arc::new(|_e: String| {});

    let params = CreateTypingParams {
        start,
        stop: None,
        on_start_error,
        on_stop_error: None,
    };

    let callbacks = create_typing_callbacks(params);
    let f = (callbacks.on_reply_start)();
    block_on(f).unwrap();
    assert!(*started.lock().unwrap());
}
