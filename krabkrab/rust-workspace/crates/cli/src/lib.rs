//! CLI crate skeleton - port CLI commands here.

pub fn cli_ready() -> &'static str { "cli ready" }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(cli_ready(), "cli ready");
    }
}
