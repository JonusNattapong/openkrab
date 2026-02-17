use futures::future::BoxFuture;
use std::sync::Arc;

pub struct TypingCallbacks {
    pub on_reply_start: Arc<dyn Fn() -> BoxFuture<'static, Result<(), String>> + Send + Sync>,
    pub on_idle: Option<Arc<dyn Fn() -> BoxFuture<'static, Result<(), String>> + Send + Sync>>,
    pub on_cleanup: Option<Arc<dyn Fn() -> BoxFuture<'static, Result<(), String>> + Send + Sync>>,
}

pub struct CreateTypingParams {
    pub start: Arc<dyn Fn() -> BoxFuture<'static, Result<(), String>> + Send + Sync>,
    pub stop: Option<Arc<dyn Fn() -> BoxFuture<'static, Result<(), String>> + Send + Sync>>,
    pub on_start_error: Arc<dyn Fn(String) + Send + Sync>,
    pub on_stop_error: Option<Arc<dyn Fn(String) + Send + Sync>>,
}

pub fn create_typing_callbacks(params: CreateTypingParams) -> TypingCallbacks {
    let start = params.start.clone();
    let on_start_error = params.on_start_error.clone();
    let on_reply_start: Arc<dyn Fn() -> BoxFuture<'static, Result<(), String>> + Send + Sync> = Arc::new(move || {
        let start = start.clone();
        let on_start_error = on_start_error.clone();
        Box::pin(async move {
            match (start)().await {
                Ok(_) => Ok(()),
                Err(e) => {
                    (on_start_error)(e.clone());
                    Err(e)
                }
            }
        }) as BoxFuture<'static, Result<(), String>>
    });

    let fire_stop = params.stop.map(|stop| {
        let on_stop_error = params.on_stop_error.clone();
        Arc::new(move || {
            let stop = stop.clone();
            let on_stop_error = on_stop_error.clone();
            Box::pin(async move {
                match (stop)().await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        if let Some(handler) = on_stop_error.as_ref() {
                            (handler)(e.clone());
                        }
                        Err(e)
                    }
                }
            }) as BoxFuture<'static, Result<(), String>>
        }) as Arc<dyn Fn() -> BoxFuture<'static, Result<(), String>> + Send + Sync>
    });

    TypingCallbacks {
        on_reply_start,
        on_idle: fire_stop.clone(),
        on_cleanup: fire_stop,
    }
}
