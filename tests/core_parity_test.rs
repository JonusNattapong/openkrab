use krabkrab::config::{validate_config, AppConfig};
use krabkrab::memory::MemoryConfig;
use krabkrab::utils::{is_truthy_env, safe_json_parse, truncate_text};
use krabkrab::version::resolve_version;

#[test]
fn utils_truthy_env_parity() {
    assert!(is_truthy_env("true"));
    assert!(is_truthy_env("ON"));
    assert!(!is_truthy_env("no"));
}

#[test]
fn utils_safe_json_parse_parity() {
    assert!(safe_json_parse("{\"ok\":1}").is_some());
    assert!(safe_json_parse("not-json").is_none());
}

#[test]
fn utils_truncate_text_parity() {
    assert_eq!(truncate_text("abcdef", 3), "abc");
    assert_eq!(truncate_text("abc", 10), "abc");
}

#[test]
fn version_resolves_from_package() {
    let v = resolve_version();
    assert!(!v.trim().is_empty());
}

#[test]
fn config_validation_works() {
    let ok = AppConfig::default();
    assert!(validate_config(&ok).is_ok());

    let bad = AppConfig {
        profile: String::new(),
        ..AppConfig::default()
    };
    assert!(validate_config(&bad).is_err());
}

#[test]
fn memory_supported_provider_list_parity() {
    let providers = MemoryConfig::supported_embedding_providers();
    assert!(providers.contains(&"openai"));
    assert!(providers.contains(&"gemini"));
    assert!(providers.contains(&"ollama"));
    assert!(!providers.contains(&"minimax"));
}

