//! Common utilities crate skeleton.

pub fn common_ready() -> &'static str { "common ready" }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(common_ready(), "common ready");
    }
}
