//! Channels crate skeleton - port channel adapters here.

pub fn channels_ready() -> &'static str { "channels ready" }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(channels_ready(), "channels ready");
    }
}
