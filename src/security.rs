//! security — Security hardening: sandbox validation, env sanitization, prompt sanitization.
//! Ported from `openkrab/src/agents/sandbox/` and `openkrab/src/agents/sanitize-for-prompt.ts` (Phase 18).

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

// ──────────────────────────────────────────────────────────────────────────────
// 1. Sandbox Security Validation
// ──────────────────────────────────────────────────────────────────────────────

/// Host paths that should never be exposed inside sandbox containers.
pub const BLOCKED_HOST_PATHS: &[&str] = &[
    "/etc",
    "/private/etc",
    "/proc",
    "/sys",
    "/dev",
    "/root",
    "/boot",
    // Directories that commonly contain (or alias) the Docker socket.
    "/run",
    "/var/run",
    "/private/var/run",
    "/var/run/docker.sock",
    "/private/var/run/docker.sock",
    "/run/docker.sock",
];

static BLOCKED_NETWORK_MODES: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert("host".to_string());
    set
});

static BLOCKED_SECCOMP_PROFILES: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert("unconfined".to_string());
    set
});

static BLOCKED_APPARMOR_PROFILES: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert("unconfined".to_string());
    set
});

/// Reason why a bind mount is blocked.
#[derive(Debug, Clone, PartialEq)]
pub enum BlockedBindReason {
    /// Bind targets a blocked path.
    Targets { blocked_path: String },
    /// Bind covers the system root.
    Covers { blocked_path: String },
    /// Non-absolute source path.
    NonAbsolute { source_path: String },
}

/// Parse the host/source path from a Docker bind mount string.
/// Format: `source:target[:mode]`
pub fn parse_bind_source_path(bind: &str) -> String {
    let trimmed = bind.trim();
    if let Some(idx) = trimmed.find(':') {
        if idx > 0 {
            return trimmed[..idx].to_string();
        }
    }
    trimmed.to_string()
}

/// Normalize a POSIX path: resolve `.`, `..`, collapse `//`, strip trailing `/`.
pub fn normalize_host_path(raw: &str) -> String {
    let trimmed = raw.trim();
    let normalized = Path::new(trimmed)
        .components()
        .fold(PathBuf::new(), |mut acc, c| {
            match c {
                std::path::Component::ParentDir => {
                    acc.pop();
                }
                std::path::Component::CurDir => {}
                _ => acc.push(c),
            }
            acc
        });
    let s = normalized.to_string_lossy().to_string();
    if s.is_empty() {
        "/".to_string()
    } else {
        s
    }
}

/// Check if a normalized source path is blocked (string-only, no filesystem I/O).
pub fn get_blocked_reason_for_source_path(source_normalized: &str) -> Option<BlockedBindReason> {
    if source_normalized == "/" {
        return Some(BlockedBindReason::Covers {
            blocked_path: "/".to_string(),
        });
    }
    for blocked in BLOCKED_HOST_PATHS {
        if source_normalized == *blocked || source_normalized.starts_with(&format!("{}/", blocked))
        {
            return Some(BlockedBindReason::Targets {
                blocked_path: blocked.to_string(),
            });
        }
    }
    None
}

/// Get the reason why a bind mount is blocked.
pub fn get_blocked_bind_reason(bind: &str) -> Option<BlockedBindReason> {
    let source_raw = parse_bind_source_path(bind);
    if !source_raw.starts_with('/') {
        return Some(BlockedBindReason::NonAbsolute {
            source_path: source_raw,
        });
    }
    let normalized = normalize_host_path(&source_raw);
    get_blocked_reason_for_source_path(&normalized)
}

fn format_bind_blocked_error(bind: &str, reason: &BlockedBindReason) -> String {
    match reason {
        BlockedBindReason::NonAbsolute { source_path } => {
            format!(
                "Sandbox security: bind mount \"{}\" uses a non-absolute source path \"{}\". \
                 Only absolute POSIX paths are supported for sandbox binds.",
                bind, source_path
            )
        }
        BlockedBindReason::Covers { blocked_path } => {
            format!(
                "Sandbox security: bind mount \"{}\" covers blocked path \"{}\". \
                 Mounting system directories (or Docker socket paths) into sandbox containers is not allowed. \
                 Use project-specific paths instead (e.g. /home/user/myproject).",
                bind, blocked_path
            )
        }
        BlockedBindReason::Targets { blocked_path } => {
            format!(
                "Sandbox security: bind mount \"{}\" targets blocked path \"{}\". \
                 Mounting system directories (or Docker socket paths) into sandbox containers is not allowed. \
                 Use project-specific paths instead (e.g. /home/user/myproject).",
                bind, blocked_path
            )
        }
    }
}

/// Validate bind mounts — returns Err if any source path is dangerous.
/// Includes a symlink/realpath pass when the source path exists.
pub fn validate_bind_mounts(binds: Option<&[String]>) -> Result<(), String> {
    let binds = match binds {
        Some(b) if !b.is_empty() => b,
        _ => return Ok(()),
    };

    for raw_bind in binds {
        let bind = raw_bind.trim();
        if bind.is_empty() {
            continue;
        }

        // Fast string-only check
        if let Some(blocked) = get_blocked_bind_reason(bind) {
            return Err(format_bind_blocked_error(bind, &blocked));
        }

        // Symlink escape hardening: resolve existing absolute paths and re-check
        let source_raw = parse_bind_source_path(bind);
        let source_normalized = normalize_host_path(&source_raw);
        let source_real = try_realpath_absolute(&source_normalized);
        if source_real != source_normalized {
            if let Some(reason) = get_blocked_reason_for_source_path(&source_real) {
                return Err(format_bind_blocked_error(bind, &reason));
            }
        }
    }

    Ok(())
}

fn try_realpath_absolute(path: &str) -> String {
    if !path.starts_with('/') {
        return path.to_string();
    }
    match std::fs::canonicalize(path) {
        Ok(p) => normalize_host_path(&p.to_string_lossy()),
        Err(_) => path.to_string(),
    }
}

/// Validate network mode — blocks "host" network.
pub fn validate_network_mode(network: Option<&str>) -> Result<(), String> {
    if let Some(net) = network {
        let trimmed = net.trim().to_lowercase();
        if BLOCKED_NETWORK_MODES.contains(&trimmed) {
            return Err(format!(
                "Sandbox security: network mode \"{}\" is blocked. \
                 Network \"host\" mode bypasses container network isolation. \
                 Use \"bridge\" or \"none\" instead.",
                net
            ));
        }
    }
    Ok(())
}

/// Validate seccomp profile — blocks "unconfined".
pub fn validate_seccomp_profile(profile: Option<&str>) -> Result<(), String> {
    if let Some(prof) = profile {
        let trimmed = prof.trim().to_lowercase();
        if BLOCKED_SECCOMP_PROFILES.contains(&trimmed) {
            return Err(format!(
                "Sandbox security: seccomp profile \"{}\" is blocked. \
                 Disabling seccomp removes syscall filtering and weakens sandbox isolation. \
                 Use a custom seccomp profile file or omit this setting.",
                prof
            ));
        }
    }
    Ok(())
}

/// Validate AppArmor profile — blocks "unconfined".
pub fn validate_apparmor_profile(profile: Option<&str>) -> Result<(), String> {
    if let Some(prof) = profile {
        let trimmed = prof.trim().to_lowercase();
        if BLOCKED_APPARMOR_PROFILES.contains(&trimmed) {
            return Err(format!(
                "Sandbox security: AppArmor profile \"{}\" is blocked. \
                 Disabling AppArmor removes mandatory access controls and weakens sandbox isolation. \
                 Use a named AppArmor profile or omit this setting.",
                prof
            ));
        }
    }
    Ok(())
}

/// Full sandbox security validation.
#[derive(Debug, Clone, Default)]
pub struct SandboxConfig {
    pub binds: Option<Vec<String>>,
    pub network: Option<String>,
    pub seccomp_profile: Option<String>,
    pub apparmor_profile: Option<String>,
}

pub fn validate_sandbox_security(cfg: &SandboxConfig) -> Result<(), String> {
    let binds_slice: &[String] = cfg.binds.as_deref().unwrap_or(&[]);
    validate_bind_mounts(Some(binds_slice))?;
    validate_network_mode(cfg.network.as_deref())?;
    validate_seccomp_profile(cfg.seccomp_profile.as_deref())?;
    validate_apparmor_profile(cfg.apparmor_profile.as_deref())?;
    Ok(())
}

// ──────────────────────────────────────────────────────────────────────────────
// 2. Environment Variable Sanitization
// ──────────────────────────────────────────────────────────────────────────────

static BLOCKED_ENV_PATTERNS: LazyLock<Vec<regex::Regex>> = LazyLock::new(|| {
    let patterns = vec![
        r"^ANTHROPIC_API_KEY$",
        r"^OPENAI_API_KEY$",
        r"^GEMINI_API_KEY$",
        r"^OPENROUTER_API_KEY$",
        r"^MINIMAX_API_KEY$",
        r"^ELEVENLABS_API_KEY$",
        r"^SYNTHETIC_API_KEY$",
        r"^TELEGRAM_BOT_TOKEN$",
        r"^DISCORD_BOT_TOKEN$",
        r"^SLACK_(BOT|APP)_TOKEN$",
        r"^LINE_CHANNEL_SECRET$",
        r"^LINE_CHANNEL_ACCESS_TOKEN$",
        r"^OPENKRAB_GATEWAY_(TOKEN|PASSWORD)$",
        r"^AWS_(SECRET_ACCESS_KEY|SECRET_KEY|SESSION_TOKEN)$",
        r"^(GH|GITHUB)_TOKEN$",
        r"^(AZURE|AZURE_OPENAI|COHERE|AI_GATEWAY|OPENROUTER)_API_KEY$",
        r"_?(API_KEY|TOKEN|PASSWORD|PRIVATE_KEY|SECRET)$",
    ];
    patterns
        .into_iter()
        .map(|p| regex::Regex::new(p).unwrap())
        .collect()
});

static ALLOWED_ENV_PATTERNS: LazyLock<Vec<regex::Regex>> = LazyLock::new(|| {
    let patterns = vec![
        r"^LANG$",
        r"^LC_.*$",
        r"^PATH$",
        r"^HOME$",
        r"^USER$",
        r"^SHELL$",
        r"^TERM$",
        r"^TZ$",
        r"^NODE_ENV$",
    ];
    patterns
        .into_iter()
        .map(|p| regex::Regex::new(p).unwrap())
        .collect()
});

/// Result of environment variable sanitization.
#[derive(Debug, Clone, Default)]
pub struct EnvSanitizationResult {
    pub allowed: std::collections::HashMap<String, String>,
    pub blocked: Vec<String>,
    pub warnings: Vec<String>,
}

/// Options for environment variable sanitization.
#[derive(Debug, Clone)]
pub struct EnvSanitizationOptions {
    pub strict_mode: bool,
    pub custom_blocked_patterns: Vec<regex::Regex>,
    pub custom_allowed_patterns: Vec<regex::Regex>,
}

impl Default for EnvSanitizationOptions {
    fn default() -> Self {
        Self {
            strict_mode: false,
            custom_blocked_patterns: vec![],
            custom_allowed_patterns: vec![],
        }
    }
}

fn validate_env_var_value(value: &str) -> Option<String> {
    if value.contains('\0') {
        return Some("Contains null bytes".to_string());
    }
    if value.len() > 32768 {
        return Some("Value exceeds maximum length".to_string());
    }
    // Check for base64-encoded credential data (80+ chars of base64)
    if regex::Regex::new(r"^[A-Za-z0-9+/=]{80,}$")
        .unwrap()
        .is_match(value)
    {
        return Some("Value looks like base64-encoded credential data".to_string());
    }
    None
}

fn matches_any_pattern(value: &str, patterns: &[&regex::Regex]) -> bool {
    patterns.iter().any(|p| p.is_match(value))
}

/// Sanitize environment variables — filter out credentials and dangerous values.
pub fn sanitize_env_vars(
    env_vars: &std::collections::HashMap<String, String>,
    options: &EnvSanitizationOptions,
) -> EnvSanitizationResult {
    let mut result = EnvSanitizationResult::default();

    let blocked_patterns: Vec<_> = BLOCKED_ENV_PATTERNS
        .iter()
        .chain(options.custom_blocked_patterns.iter())
        .collect();

    let allowed_patterns: Vec<_> = ALLOWED_ENV_PATTERNS
        .iter()
        .chain(options.custom_allowed_patterns.iter())
        .collect();

    for (raw_key, value) in env_vars {
        let key = raw_key.trim();
        if key.is_empty() {
            continue;
        }

        // Check blocked patterns
        if matches_any_pattern(key, &blocked_patterns) {
            result.blocked.push(key.to_string());
            continue;
        }

        // Strict mode: only allow explicitly allowed patterns
        if options.strict_mode && !matches_any_pattern(key, &allowed_patterns) {
            result.blocked.push(key.to_string());
            continue;
        }

        // Validate value
        if let Some(warning) = validate_env_var_value(value) {
            if warning == "Contains null bytes" {
                result.blocked.push(key.to_string());
                continue;
            }
            result.warnings.push(format!("{}: {}", key, warning));
        }

        result.allowed.insert(key.to_string(), value.clone());
    }

    result
}

/// Get blocked pattern sources for auditing.
pub fn get_blocked_env_patterns() -> Vec<String> {
    BLOCKED_ENV_PATTERNS
        .iter()
        .map(|p| p.as_str().to_string())
        .collect()
}

/// Get allowed pattern sources for auditing.
pub fn get_allowed_env_patterns() -> Vec<String> {
    ALLOWED_ENV_PATTERNS
        .iter()
        .map(|p| p.as_str().to_string())
        .collect()
}

// ──────────────────────────────────────────────────────────────────────────────
// 3. Prompt Sanitization
// ──────────────────────────────────────────────────────────────────────────────

/// Check if a character is a control character (Unicode Cc category).
fn is_control_char(c: char) -> bool {
    // Control characters are U+0000 to U+001F and U+007F to U+009F
    let cp = c as u32;
    (cp <= 0x1F) || (cp >= 0x7F && cp <= 0x9F)
}

/// Check if a character is a format character (Unicode Cf category).
/// This includes bidi marks and zero-width characters.
fn is_format_char(c: char) -> bool {
    let cp = c as u32;
    // Common format characters
    matches!(cp,
        0x00AD | // Soft hyphen
        0x0600..=0x0605 | // Arabic format chars
        0x06DD | // Arabic end of ayah
        0x070F | // Syriac abbreviation mark
        0x08E2 | // Arabic disjoined mark
        0x200C..=0x200F | // ZWNJ, ZWJ, LRM, RLM
        0x202A..=0x202E | // LRE, RLE, PDF, LRO, RLO (bidi)
        0x2060..=0x206F | // Invisible operators, etc.
        0xFEFF | // Zero-width no-break space (BOM)
        0xFFF9..=0xFFFB | // Interlinear annotation chars
        0x110BD | // Kaithi number sign
        0x1BCA0..=0x1BCA3 | // Shorthand format chars
        0x1D173..=0x1D17A | // Musical symbols
        0xE0001 | // Language tag
        0xE0020..=0xE007F // Tag characters
    )
}

/// Sanitize untrusted strings before embedding them into an LLM prompt.
///
/// Strategy:
/// - Strip Unicode "control" (Cc) + "format" (Cf) characters (includes CR/LF/NUL, bidi marks, zero-width chars).
/// - Strip explicit line/paragraph separators (Zl/Zp): U+2028/U+2029.
///
/// This is intentionally lossy; it trades edge-case path fidelity for prompt integrity.
pub fn sanitize_for_prompt_literal(value: &str) -> String {
    value
        .chars()
        .filter(|c| {
            let cp = *c as u32;
            !is_control_char(*c) && !is_format_char(*c) && !matches!(cp, 0x2028 | 0x2029)
            // Line/paragraph separators (Zl/Zp)
        })
        .collect()
}

// ──────────────────────────────────────────────────────────────────────────────
// 4. Legacy Redaction (existing)
// ──────────────────────────────────────────────────────────────────────────────

/// Redact sensitive keywords from a string (basic).
pub fn redact_sensitive(input: &str) -> String {
    let mut out = input.to_string();
    for key in ["token", "apikey", "api_key", "secret", "password"] {
        out = out.replace(key, "[REDACTED_KEY]");
    }
    out
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ─── Sandbox Security Tests ───────────────────────────────────────────────

    #[test]
    fn test_parse_bind_source_path() {
        assert_eq!(parse_bind_source_path("/host:/container"), "/host");
        assert_eq!(parse_bind_source_path("/host:/container:ro"), "/host");
        assert_eq!(parse_bind_source_path("/host"), "/host");
        assert_eq!(parse_bind_source_path(":/container"), ":/container");
    }

    #[test]
    fn test_normalize_host_path() {
        assert_eq!(normalize_host_path("/etc"), "/etc");
        assert_eq!(normalize_host_path("/etc/"), "/etc");
        assert_eq!(normalize_host_path("/etc//passwd"), "/etc/passwd");
        assert_eq!(normalize_host_path("/etc/../home"), "/home");
        assert_eq!(normalize_host_path("/etc/./passwd"), "/etc/passwd");
    }

    #[test]
    fn test_get_blocked_reason_targets() {
        let reason = get_blocked_reason_for_source_path("/etc/passwd");
        assert!(matches!(reason, Some(BlockedBindReason::Targets { .. })));
    }

    #[test]
    fn test_get_blocked_reason_covers_root() {
        let reason = get_blocked_reason_for_source_path("/");
        assert!(
            matches!(reason, Some(BlockedBindReason::Covers { blocked_path }) if blocked_path == "/")
        );
    }

    #[test]
    fn test_get_blocked_reason_non_absolute() {
        let reason = get_blocked_bind_reason("relative:/container");
        assert!(
            matches!(reason, Some(BlockedBindReason::NonAbsolute { source_path }) if source_path == "relative")
        );
    }

    #[test]
    fn test_validate_bind_mounts_blocks_etc() {
        let binds = vec!["/etc:/container/etc".to_string()];
        let result = validate_bind_mounts(Some(&binds));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("targets blocked path"));
    }

    #[test]
    fn test_validate_bind_mounts_allows_safe() {
        let binds = vec!["/home/user/project:/app".to_string()];
        assert!(validate_bind_mounts(Some(&binds)).is_ok());
    }

    #[test]
    fn test_validate_bind_mounts_empty() {
        assert!(validate_bind_mounts(None).is_ok());
        assert!(validate_bind_mounts(Some(&[])).is_ok());
    }

    #[test]
    fn test_validate_network_mode_blocks_host() {
        let result = validate_network_mode(Some("host"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("host\" is blocked"));
    }

    #[test]
    fn test_validate_network_mode_allows_bridge() {
        assert!(validate_network_mode(Some("bridge")).is_ok());
    }

    #[test]
    fn test_validate_seccomp_profile_blocks_unconfined() {
        let result = validate_seccomp_profile(Some("unconfined"));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_apparmor_profile_blocks_unconfined() {
        let result = validate_apparmor_profile(Some("unconfined"));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sandbox_security_full() {
        let cfg = SandboxConfig {
            binds: Some(vec!["/home/user:/app".to_string()]),
            network: Some("bridge".to_string()),
            ..Default::default()
        };
        assert!(validate_sandbox_security(&cfg).is_ok());

        let bad_cfg = SandboxConfig {
            binds: Some(vec!["/etc:/app".to_string()]),
            ..Default::default()
        };
        assert!(validate_sandbox_security(&bad_cfg).is_err());
    }

    // ─── Env Var Sanitization Tests ───────────────────────────────────────────

    #[test]
    fn test_sanitize_env_vars_blocks_api_keys() {
        let mut env = std::collections::HashMap::new();
        env.insert("OPENAI_API_KEY".to_string(), "sk-12345".to_string());
        env.insert("SAFE_VAR".to_string(), "safe_value".to_string());

        let result = sanitize_env_vars(&env, &EnvSanitizationOptions::default());
        assert!(result.blocked.contains(&"OPENAI_API_KEY".to_string()));
        assert!(result.allowed.contains_key("SAFE_VAR"));
    }

    #[test]
    fn test_sanitize_env_vars_strict_mode() {
        let mut env = std::collections::HashMap::new();
        env.insert("PATH".to_string(), "/usr/bin".to_string());
        env.insert("UNKNOWN_VAR".to_string(), "value".to_string());

        let options = EnvSanitizationOptions {
            strict_mode: true,
            ..Default::default()
        };
        let result = sanitize_env_vars(&env, &options);
        assert!(result.allowed.contains_key("PATH"));
        assert!(result.blocked.contains(&"UNKNOWN_VAR".to_string()));
    }

    #[test]
    fn test_sanitize_env_vars_blocks_null_bytes() {
        let mut env = std::collections::HashMap::new();
        env.insert("BAD_VAR".to_string(), "value\0with\0nulls".to_string());

        let result = sanitize_env_vars(&env, &EnvSanitizationOptions::default());
        assert!(result.blocked.contains(&"BAD_VAR".to_string()));
    }

    #[test]
    fn test_sanitize_env_vars_warns_base64() {
        let mut env = std::collections::HashMap::new();
        // 80+ chars of base64-like data
        let base64_like = "a".repeat(80);
        env.insert("SUSPICIOUS".to_string(), base64_like);

        let result = sanitize_env_vars(&env, &EnvSanitizationOptions::default());
        assert!(result.warnings.iter().any(|w| w.contains("base64")));
    }

    // ─── Prompt Sanitization Tests ────────────────────────────────────────────

    #[test]
    fn test_sanitize_for_prompt_literal_strips_control_chars() {
        let input = "hello\nworld\0test";
        let sanitized = sanitize_for_prompt_literal(input);
        assert!(!sanitized.contains('\n'));
        assert!(!sanitized.contains('\0'));
        assert_eq!(sanitized, "helloworldtest");
    }

    #[test]
    fn test_sanitize_for_prompt_literal_strips_line_separators() {
        let input = "hello\u{2028}world\u{2029}test";
        let sanitized = sanitize_for_prompt_literal(input);
        assert!(!sanitized.contains('\u{2028}'));
        assert!(!sanitized.contains('\u{2029}'));
        assert_eq!(sanitized, "helloworldtest");
    }

    #[test]
    fn test_sanitize_for_prompt_literal_preserves_normal_text() {
        let input = "Hello, World! 123 @#$%";
        assert_eq!(sanitize_for_prompt_literal(input), input);
    }

    #[test]
    fn test_sanitize_for_prompt_literal_strips_bidi_marks() {
        // U+202E is a bidi mark (Right-to-Left Override)
        let input = "hello\u{202E}world";
        let sanitized = sanitize_for_prompt_literal(input);
        assert!(!sanitized.contains('\u{202E}'));
    }

    // ─── Redaction Tests ──────────────────────────────────────────────────────

    #[test]
    fn test_redact_sensitive() {
        let input = "my token is secret and api_key too";
        let redacted = redact_sensitive(input);
        assert!(redacted.contains("[REDACTED_KEY]"));
        assert!(!redacted.contains("token"));
    }
}
