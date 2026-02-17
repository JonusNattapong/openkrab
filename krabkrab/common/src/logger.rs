pub fn init_logger() {
    let _ = env_logger::builder().is_test(false).try_init();
}

pub fn info(msg: &str) {
    log::info!("{}", msg);
}

pub fn error(msg: &str) {
    log::error!("{}", msg);
}
