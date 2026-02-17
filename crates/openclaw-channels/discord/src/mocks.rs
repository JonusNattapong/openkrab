// Test mocks for Discord HTTP interactions
// Kept minimal to avoid depending on internal non-exhaustive serenity types

#[cfg(test)]
pub mod mocks {
    use crate::UserId;
    use uuid::Uuid;

    #[derive(Clone, Debug)]
    pub struct MockUser {
        pub id: u64,
        pub username: String,
    }

    impl MockUser {
        pub fn new(id: u64, username: impl Into<String>) -> Self {
            Self {
                id,
                username: username.into(),
            }
        }
    }
}
