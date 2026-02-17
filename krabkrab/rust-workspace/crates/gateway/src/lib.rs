//! Gateway crate skeleton - port core gateway logic here.

pub fn hello_gateway() -> &'static str {
    "gateway ready"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(hello_gateway(), "gateway ready");
    }
}
