// Lightweight test-only mocks for teloxide interactions
// Avoid constructing non-exhaustive upstream types; provide simple wrappers used in unit tests.

#[cfg(test)]
pub mod mocks {
    pub struct MockUpdate;
    impl MockUpdate {
        pub fn new() -> Self {
            MockUpdate
        }
    }
}
