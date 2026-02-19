pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn resolve_version() -> String {
    VERSION.to_string()
}
