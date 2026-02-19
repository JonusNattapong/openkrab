use futures::future::BoxFuture;
// std imports
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type SendOrEditFn = dyn Fn(String) -> BoxFuture<'static, Option<bool>> + Send + Sync;
type IsStoppedFn = dyn Fn() -> bool + Send + Sync;

struct Inner {
    throttle_ms: u64,
    is_stopped: Arc<IsStoppedFn>,
    send_or_edit: Arc<SendOrEditFn>,
    last_sent_at: u128,
    pending_text: String,
    in_flight: Option<BoxFuture<'static, bool>>,
    timer_running: bool,
}

#[derive(Clone)]
pub struct DraftStreamLoop {
    inner: Arc<Mutex<Inner>>,
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

impl DraftStreamLoop {
    pub fn update(&self, text: String) {
        let mut inner = self.inner.lock().unwrap();
        if (inner.is_stopped)() {
            return;
        }
        inner.pending_text = text;
        if inner.in_flight.is_some() {
            Self::schedule_inner(self.inner.clone());
            return;
        }
        if !inner.timer_running
            && now_ms().saturating_sub(inner.last_sent_at) >= inner.throttle_ms as u128
        {
            // spawn immediate flush
            let me = self.clone();
            thread::spawn(move || {
                futures::executor::block_on(me.flush());
            });
            return;
        }
        Self::schedule_inner(self.inner.clone());
    }

    fn schedule_inner(inner_arc: Arc<Mutex<Inner>>) {
        let mut inner = inner_arc.lock().unwrap();
        if inner.timer_running {
            return;
        }
        inner.timer_running = true;
        let throttle = inner.throttle_ms;
        let last = inner.last_sent_at;
        drop(inner);
        let me = DraftStreamLoop {
            inner: inner_arc.clone(),
        };
        thread::spawn(move || {
            let delay = throttle.saturating_sub((now_ms().saturating_sub(last)) as u64);
            thread::sleep(Duration::from_millis(delay));
            // clear timer_running before flush to allow re-scheduling
            {
                let mut locked = inner_arc.lock().unwrap();
                locked.timer_running = false;
            }
            futures::executor::block_on(me.flush());
        });
    }

    pub async fn flush(&self) {
        loop {
            if (self.inner.lock().unwrap().is_stopped)() {
                return;
            }

            // wait for existing in-flight
            if let Some(fut) = { self.inner.lock().unwrap().in_flight.take() } {
                let _ = fut.await;
                // continue loop to re-check pending
                continue;
            }

            let text = {
                let inner = self.inner.lock().unwrap();
                inner.pending_text.clone()
            };

            if text.trim().is_empty() {
                let mut inner = self.inner.lock().unwrap();
                inner.pending_text.clear();
                return;
            }

            // clear pending and create in-flight future
            {
                let mut inner = self.inner.lock().unwrap();
                inner.pending_text.clear();
                let fut = (inner.send_or_edit)(text.clone());
                // map Option<bool> -> bool (None -> true)
                let mapped: BoxFuture<'static, bool> =
                    Box::pin(async move { fut.await.unwrap_or(true) });
                inner.in_flight = Some(mapped);
            }

            // take and await
            let sent = {
                let mut inner = self.inner.lock().unwrap();
                let fut = inner.in_flight.take().unwrap();
                drop(inner);
                fut.await
            };

            let mut inner = self.inner.lock().unwrap();
            if sent == false {
                // restore pending text
                inner.pending_text = text;
                return;
            }
            inner.last_sent_at = now_ms();
            if inner.pending_text.is_empty() {
                return;
            }
            // else loop to send next
        }
    }

    pub fn stop(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.pending_text.clear();
        inner.timer_running = false;
    }

    pub fn reset_pending(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.pending_text.clear();
    }

    pub async fn wait_for_in_flight(&self) {
        if let Some(fut) = { self.inner.lock().unwrap().in_flight.take() } {
            let _ = fut.await;
        }
    }
}

pub fn create_draft_stream_loop(
    throttle_ms: u64,
    is_stopped: Arc<IsStoppedFn>,
    send_or_edit: Arc<SendOrEditFn>,
) -> DraftStreamLoop {
    let inner = Inner {
        throttle_ms,
        is_stopped,
        send_or_edit,
        last_sent_at: 0,
        pending_text: String::new(),
        in_flight: None,
        timer_running: false,
    };
    DraftStreamLoop {
        inner: Arc::new(Mutex::new(inner)),
    }
}
