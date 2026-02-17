pub fn should_monitor_thread(thread_ts: Option<&str>) -> bool {
    thread_ts.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monitors_when_thread_present() {
        assert!(should_monitor_thread(Some("123")));
    }

    #[test]
    fn does_not_monitor_when_none() {
        assert!(!should_monitor_thread(None));
    }
}
