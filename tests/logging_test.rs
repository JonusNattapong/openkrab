use openkrab::logging;

#[test]
fn init_and_log() {
    logging::init();
    logging::log_example();
    assert!(true);
}

