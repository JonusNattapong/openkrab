//! Agents crate skeleton - port agent runtime here.

pub fn agents_ready() -> &'static str { "agents ready" }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(agents_ready(), "agents ready");
    }
}
